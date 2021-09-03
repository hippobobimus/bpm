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
};

/// A SystemSet that applies a dynamic response to Entitys that are in collision.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(calc_impulse.system()
                     .label("calculate response")
        )
        .with_system(apply_impulse.system()
                     .label("apply response")
                     .after("calculate response")
        )
}


// SYSTEM: Apply calculated impulse to update velocity of entities involved in the collision.
fn apply_impulse(
    mut commands: Commands,
    mut query: Query<(Entity, &Impulse, &ImpulsiveTorque, &InertiaTensor, &Mass, &mut Velocity, &mut AngularVelocity)>,
) {
    for (ent, impulse, impulsive_torque, inertia_tensor, mass, mut velocity, mut ang_velocity) in query.iter_mut() {
        println!("Impulse: {}", impulse.0);
        println!("Impulsive torque: {}", impulsive_torque.0);

        velocity.add(mass.inverse() * impulse.0);
        // TODO limit large values.
        //ang_velocity.add(inertia_tensor.inverse_global().mul_vec3(impulsive_torque.0));

        commands.entity(ent).remove::<Impulse>();
        commands.entity(ent).remove::<ImpulsiveTorque>();
        // TODO create a bundle and then use remove_bundle
    }
}

// SYSTEM: Calculate impulse and impulsive torque to be applied to each colliding entity.
fn calc_impulse(
    mut commands: Commands,
    contact_query: Query<(Entity, &Contact)>,
    mut q: QuerySet<(
        Query<(&AngularVelocity, &InertiaTensor, &Mass, &PhysTransform, &Velocity)>,
        Query<(&mut PhysTransform, &mut Transform)>,
    )>,
) {
    for (contact_entity, contact) in contact_query.iter() {
        println!("contact: {:?}", contact);
        // transformation matrix to go from global coords to contact coords.
        let contact_transform = calc_contact_basis(contact.normal);

        let mut delta_vel_per_unit_impulse = 0.0;
        let mut closing_vel_global = DVec3::ZERO;

        let mut relative_contact_position = [DVec3::ZERO, DVec3::ZERO];
        if let Some(entity) = contact.entities[0] {
            let (angular_velocity, inertia_tensor, mass, phys_transform, velocity) = q.q0().get(entity)
                .expect("Invalid contact entity");

            relative_contact_position[0] = contact.point - phys_transform.translation();

            delta_vel_per_unit_impulse += calc_velocity_per_unit_impulse(
                contact_transform,
                mass.inverse(),
                inertia_tensor.inverse_global(),
                contact.normal,
                relative_contact_position[0],
            );

            closing_vel_global += calc_contact_velocity(
                contact_transform,
                relative_contact_position[0],
                angular_velocity.vector(),
                velocity.vector(),
            );
        }
        if let Some(entity) = contact.entities[1] {
            let (angular_velocity, inertia_tensor, mass, phys_transform, velocity) = q.q0().get(entity)
                .expect("Invalid contact entity");

            relative_contact_position[1] = contact.point - phys_transform.translation();

            delta_vel_per_unit_impulse += calc_velocity_per_unit_impulse(
                contact_transform,
                mass.inverse(),
                inertia_tensor.inverse_global(),
                contact.normal,
                relative_contact_position[1],
            );

            // subtract second value
            closing_vel_global -= calc_contact_velocity(
                contact_transform,
                relative_contact_position[1],
                angular_velocity.vector(),
                velocity.vector(),
            );
        }

        // transform closing velocity into contact coords.
        let closing_vel_contact = contact_transform.mul_vec3(closing_vel_global);

        // desired change in velocity = -(1 + c) * closing velocity in direction of contact normal.
        let restitution = 0.4; // TODO make this contact specific.
        let delta_vel = -(1.0 + restitution) * closing_vel_contact.x;

        // frictionless, so impulse is only in direction of contact normal.
        let mut impulse_contact = DVec3::new(
            delta_vel / delta_vel_per_unit_impulse,
            0.0,
            0.0,
        );

        println!("impulse contact before {}", impulse_contact);
        // Make sure normal impulse is > 0.
        impulse_contact.x = impulse_contact.x.min(0.0);
        println!("impulse contact after {}", impulse_contact);

        // transform impulse back into global coords. The transform is just a rotation, so we can
        // use the transpose instead of the inverse to speed up calculation.
        let impulse = contact_transform.transpose().mul_vec3(impulse_contact);
        println!("impulse global after {}", impulse);

        // add impulse and impulsive torque to both bodies, with the direction of the impulse
        // reversed on the 2nd body.
        if let Some(entity) = contact.entities[0] {
            commands.entity(entity).insert(Impulse(impulse));
            let impulsive_torque = relative_contact_position[0].cross(impulse);
            commands.entity(entity).insert(ImpulsiveTorque(impulsive_torque));
        }
        if let Some(entity) = contact.entities[1] {
            commands.entity(entity).insert(Impulse(-impulse));
            let impulsive_torque = relative_contact_position[1].cross(-impulse);
            commands.entity(entity).insert(ImpulsiveTorque(impulsive_torque));
        }

        // Resolve interpenetration.
        if let Some(entity) = contact.entities[0] {
            let (mut phys_transform, mut transform) = q.q1_mut().get_mut(entity).expect("Entity does not exist!");

            phys_transform.translation -= contact.normal * contact.penetration * 0.5;
            transform.translation = phys_transform.translation.as_f32();
        }
        if let Some(entity) = contact.entities[1] {
            let (mut phys_transform, mut transform) = q.q1_mut().get_mut(entity).expect("Entity does not exist!");

            phys_transform.translation += contact.normal * contact.penetration * 0.5;
            transform.translation = phys_transform.translation.as_f32();
        }

        // finally remove contact
        commands.entity(contact_entity).despawn();
    }
}

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

// local contact coords
fn calc_velocity_per_unit_impulse(
    contact_transform: DMat3,
    inverse_mass: f64,
    inverse_inertia_tensor: DMat3,
    normal: DVec3,
    relative_contact_position: DVec3,
) -> f64 {
    // linear velocity change per unit impulse
    let linear_component = inverse_mass;

    // angular velocity change per unit impulse
    let torque = relative_contact_position.cross(normal);
    let angular_velocity = inverse_inertia_tensor.mul_vec3(torque);
    let velocity_global = angular_velocity.cross(relative_contact_position);
    // transform to contact coords
    let velocity = contact_transform.mul_vec3(velocity_global);
    let angular_component = velocity.x;


    // Overall
    angular_component + linear_component
}

// local contact coords
fn calc_contact_velocity(
    contact_transform: DMat3,
    relative_contact_position: DVec3,
    angular_velocity: DVec3,
    velocity: DVec3,
) -> DVec3 {
    // global coords
    // angular component
    let mut result = angular_velocity.cross(relative_contact_position);
    // linear component
    result += velocity;

    // convert to contact coords
    result = contact_transform.mul_vec3(result);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_contact_velocity() {
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
}
