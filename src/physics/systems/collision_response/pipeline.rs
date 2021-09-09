use bevy::{
    prelude::*,
    math::{
        DMat3,
        DVec3,
    },
};

use crate::{
    physics::components::{
        AngularVelocity,
        Contact,
        Impulse,
        ImpulsiveTorque,
        InertiaTensor,
        Mass,
        PhysTransform,
        Velocity,
    },
    user_interaction::components::Player,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
/// System labels covering collision response sub-systems.
enum CollisionResponseSystems {
    CalculateImpulse,
}

/// A SystemSet that calculates and applies a dynamic response to Entitys that are in collision.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(calc_impulse.system()
                     .label(CollisionResponseSystems::CalculateImpulse)
        )
}

/// A system that iterates through available collision contacts, calculating and applying
/// appropriate impulse and impulsive torques to resolve them.
fn calc_impulse(
    mut commands: Commands,
    contact_query: Query<(Entity, &Contact)>,
    mut q: QuerySet<(
        Query<(&AngularVelocity, &InertiaTensor, &Mass, &PhysTransform, &Velocity)>,
        Query<(&mut PhysTransform, &mut Transform)>,
        Query<(&InertiaTensor, &Mass, &mut Velocity, &mut AngularVelocity), With<Player>>,
    )>,
) {
    for (contact_entity, contact) in contact_query.iter() {
        info!("processing contact: {:?}", contact);

        // transformation matrices to go between global coords and contact coords.
        let contact_to_global_transform = calc_contact_basis(contact.normal);
        let global_to_contact_transform = contact_to_global_transform.transpose();

        info!("contact to global transform {}", contact_to_global_transform);
        info!("global to contact transform {}", contact_to_global_transform.transpose());

        let mut inverse_mass = [0.0, 0.0];

        let mut relative_contact_position = [DVec3::ZERO, DVec3::ZERO];
        let mut delta_velocity_per_unit_impulse = 0.0;
        let mut contact_velocity = [DVec3::ZERO, DVec3::ZERO];

        for (i, entity_option) in contact.entities.iter().enumerate() {
            if let Some(entity) = entity_option {
                let (angular_velocity, inertia_tensor, mass, phys_transform, velocity) =
                    q.q0().get(*entity).expect("Invalid contact entity");

                inverse_mass[i] = mass.inverse();

                relative_contact_position[i] = contact.point - phys_transform.translation();

                delta_velocity_per_unit_impulse += calc_normal_velocity_per_unit_impulse(
                    global_to_contact_transform,
                    mass.inverse(),
                    inertia_tensor.inverse_global(),
                    contact.normal,
                    relative_contact_position[i],
                );

                contact_velocity[i] = calc_contact_velocity(
                    global_to_contact_transform,
                    relative_contact_position[i],
                    angular_velocity.vector(),
                    velocity.vector(),
                );
            }
        }

        let closing_velocity_contact = contact_velocity[0] - contact_velocity[1];

        info!("closing velocity in contact coords {}", closing_velocity_contact);
        info!("delta velocity along normal per unit impulse {}", delta_velocity_per_unit_impulse);

        // Desired change in velocity = -(1 + c) * closing velocity in direction of contact normal.
        let restitution = 0.4; // TODO make this contact specific.
        let delta_velocity = -(1.0 + restitution) * closing_velocity_contact.x;

        // Frictionless, so impulse is only in direction of contact normal.
        let mut impulse_contact = DVec3::new(
            delta_velocity / delta_velocity_per_unit_impulse,
            0.0,
            0.0,
        );

        info!("impulse in contact coords before checking sign {}", impulse_contact);

        // Make sure normal impulse is < 0. i.e. it acts in the opposite direction to the contact
        // normal, pushing the 0th body away from the contact.
        impulse_contact.x = impulse_contact.x.min(0.0);
        info!("impulse in contact coords after {}", impulse_contact);

        // Transform impulse back into global coords.
        let impulse = contact_to_global_transform.mul_vec3(impulse_contact);
        info!("impulse in global coords {}", impulse);

        // Add impulse and impulsive torque to both bodies, reversing the impulse direction for the
        // subsequent body.
        for ((i, entity_option), c) in contact.entities.iter().enumerate().zip([1.0, -1.0].iter()) {
            if let Some(entity) = entity_option {
                let impulsive_torque = relative_contact_position[i].cross(*c * impulse);

                if let Ok((inertia_tensor, mass, mut velocity, mut ang_velocity)) = q.q2_mut().get_mut(*entity) {
                    velocity.add(mass.inverse() * c * impulse);
                    ang_velocity.add(inertia_tensor.inverse_global().mul_vec3(impulsive_torque));
                }

                commands.entity(*entity).insert(Impulse(*c * impulse));
                commands.entity(*entity).insert(ImpulsiveTorque(*c * impulsive_torque));
            }
        }

        // Resolve interpenetration.
        if let Some(entity) = contact.entities[0] {
            let (mut phys_transform, mut transform) = q.q1_mut().get_mut(entity).expect("Entity does not exist!");

            let factor = inverse_mass[0] / (inverse_mass[0] + inverse_mass[1]);
            phys_transform.translation -= contact.normal * contact.penetration * factor;
            transform.translation = phys_transform.translation.as_f32();
        }
        if let Some(entity) = contact.entities[1] {
            let (mut phys_transform, mut transform) = q.q1_mut().get_mut(entity).expect("Entity does not exist!");

            let factor = inverse_mass[1] / (inverse_mass[0] + inverse_mass[1]);
            phys_transform.translation += contact.normal * contact.penetration * factor;
            transform.translation = phys_transform.translation.as_f32();
        }

        // finally remove contact
        commands.entity(contact_entity).despawn();
    }
}

// --- Helper methods

/// Takes the contact normal in global space and returns an arbitrary orthonormal basis for the
/// contact as a DMat3. The matrix represents transformation from contact space into global space.
/// Note, the origin of contact space is the global origin to simplify calculations (the transform
/// is a rotation only, so its inverse is the transpose).
fn calc_contact_basis(normal: DVec3) -> DMat3 {
    // The normal will be the new x-axis.
    // Initially, choose the y-axis to be either the global x-axis or global y-axis, whichever is
    // further from the normal to avoid the parallel case.
    let mut y = if normal.dot(DVec3::X).abs() > normal.dot(DVec3::Y).abs() {
        // normal nearer to global x-axis
        DVec3::Y
    } else {
        // normal nearer to global y-axis
        DVec3::X
    };

    // z is the normalised vector at right angles to the normal and the chosen y-axis.
    let z = normal.cross(y).normalize();

    // Then y must be the vector at right angles to this z axis and the normal.
    y = z.cross(normal);

    DMat3::from_cols(
        normal,
        y,
        z,
    )
}

/// Calculates the change in velocity along the contact normal per unit of impulse and returns it
/// as a f64 value.
fn calc_normal_velocity_per_unit_impulse(
    global_to_contact_transform: DMat3,
    inverse_mass: f64,
    inverse_inertia_tensor: DMat3,
    normal: DVec3,
    relative_contact_position: DVec3,
) -> f64 {
    // Normal linear velocity change per unit impulse
    // v_lin = inverse_mass * J --> replace impulse vector with contact normal.
    let linear_component = inverse_mass;

    // Angular velocity change per unit impulse
    // delta ang vel = inverse inertia tensor * impulsive torque
    // where; impulsive torque = rel position x J --> replace J vector with contact normal.
    let impulsive_torque_per_unit_impulse = relative_contact_position.cross(normal);

    let angular_velocity_per_unit_impulse =
        inverse_inertia_tensor.mul_vec3(impulsive_torque_per_unit_impulse);

    // The angular component of the velocity change in global coords.
    let angular_component_global =
        angular_velocity_per_unit_impulse.cross(relative_contact_position);

    // Transform to contact coords and take the component along the contact normal (x-axis in
    // contact coords).
    let angular_component_vector = global_to_contact_transform.mul_vec3(angular_component_global);
    let angular_component = angular_component_vector.x;

    // Combine the angular and linear parts to get the result.
    angular_component + linear_component
}

/// Returns the velocity of the contact point in contact coords as a DVec3.
fn calc_contact_velocity(
    global_to_contact_transform: DMat3,
    relative_contact_position: DVec3,
    angular_velocity: DVec3,
    velocity: DVec3,
) -> DVec3 {
    // velocity = linear velocity + (angular velocity x relative position).

    // first calculate velocity in global coords.
    let mut result = velocity + angular_velocity.cross(relative_contact_position);

    info!("contact velocity in global coords = {}", result);

    // convert to contact coords
    result = global_to_contact_transform.mul_vec3(result);

    info!("contact velocity in contact coords = {}", result);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_contact_basis() {
        let normal = DVec3::Y;
        let basis = calc_contact_basis(normal);

        // the normal is close to y-axis, so the new y-axis is the former x-axis.
        assert_eq!(normal, basis.mul_vec3(DVec3::X));
        assert_eq!(DVec3::X, basis.mul_vec3(DVec3::Y));
        assert_eq!(-DVec3::Z, basis.mul_vec3(DVec3::Z));

        let normal = DVec3::X;
        let basis = calc_contact_basis(normal);

        // the normal is close to x-axis, so the new y-axis is the former y-axis.
        assert_eq!(normal, basis.mul_vec3(DVec3::X));
        assert_eq!(DVec3::Y, basis.mul_vec3(DVec3::Y));
        assert_eq!(DVec3::Z, basis.mul_vec3(DVec3::Z));
    }

    #[test]
    fn test_calc_contact_velocity() {
        let normal = DVec3::Z;
        let basis = calc_contact_basis(normal);
        let contact_point = DVec3::new(2.0, 3.0, 2.0);
        let position = DVec3::new(2.0, 2.0, 2.0);
        let relative_position = contact_point - position;
        let ang_vel = DVec3::ZERO;
        let vel = DVec3::new(0.0, 0.0, 5.0);

        let result = calc_contact_velocity(
            basis,
            relative_position,
            ang_vel,
            vel,
        );

        assert_eq!(DVec3::new(5.0, 0.0, 0.0), result);
    }
}
