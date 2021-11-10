use bevy::{
    prelude::*,
    math::{
        DMat3,
        DQuat,
        DVec3,
    },
};

use crate::{
    constants,
    physics::components::{
        AngularVelocity,
        Contact,
        InertiaTensor,
        Mass,
        PhysTransform,
        Velocity,
    },
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
/// System labels covering collision response sub-systems.
enum CollisionResponseSystems {
    CalculateImpulse,
    ResolvePenetration,
    ClearContacts,
}

/// A SystemSet that calculates and applies a dynamic response to Entitys that are in collision.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(calc_impulse.system()
                     .label(CollisionResponseSystems::CalculateImpulse)
        )
        .with_system(resolve_interpenetration.system()
                     .label(CollisionResponseSystems::ResolvePenetration)
                     .after(CollisionResponseSystems::CalculateImpulse)
        )
        .with_system(remove_contacts.system()
                     .label(CollisionResponseSystems::ClearContacts)
                     .after(CollisionResponseSystems::ResolvePenetration)
        )
}

/// A system that iterates through available collision contacts, updating their motion by
/// calculating and applying appropriate impulses and impulsive torques based on the contact and
/// body parameters.
fn calc_impulse(
    contacts_query: Query<&Contact>,
    mut q: QuerySet<(
        Query<(&AngularVelocity, &InertiaTensor, &Mass, &Velocity)>,
        Query<(&InertiaTensor, &Mass, &mut Velocity, &mut AngularVelocity)>,
    )>,
) {
    for contact in contacts_query.iter() {
        debug!("processing contact: {:?}", contact);

        // transformation matrices used to convert between global coords and contact coords.
        let contact_to_global_transform = calc_contact_basis(contact.normal);
        let global_to_contact_transform = contact_to_global_transform.transpose();

        debug!("contact to global transform {}", contact_to_global_transform);
        debug!("global to contact transform {}", contact_to_global_transform.transpose());

        let mut delta_velocity_per_unit_impulse = 0.0;
        let mut contact_velocity = [DVec3::ZERO, DVec3::ZERO];

        for (i, entity) in contact.entities.iter().enumerate() {
            let (angular_velocity, inertia_tensor, mass, velocity) = q.q0().get(*entity)
                .expect("Invalid contact entity");

            delta_velocity_per_unit_impulse += calc_normal_velocity_per_unit_impulse(
                global_to_contact_transform,
                mass.inverse(),
                inertia_tensor.inverse_global(),
                contact.normal,
                contact.relative_points[i],
            );

            contact_velocity[i] = calc_contact_velocity(
                global_to_contact_transform,
                contact.relative_points[i],
                angular_velocity.vector(),
                velocity.vector(),
            );
        }

        let closing_velocity_contact = contact_velocity[0] - contact_velocity[1];

        debug!("closing velocity in contact coords {}", closing_velocity_contact);
        debug!("delta velocity along normal per unit impulse {}", delta_velocity_per_unit_impulse);

        // Desired change in velocity = -(1 + c) * closing velocity in direction of contact normal.
        // TODO make restitution coeff contact specific.
        let delta_velocity = -(1.0 + constants::RESTITUTION_COEFF) * closing_velocity_contact.x;

        // Frictionless, so impulse is only in direction of contact normal.
        let mut impulse_contact = DVec3::new(
            delta_velocity / delta_velocity_per_unit_impulse,
            0.0,
            0.0,
        );

        debug!("impulse in contact coords before checking sign {}", impulse_contact);

        // Make sure normal impulse is < 0. i.e. it acts in the opposite direction to the contact
        // normal, pushing the 0th body away from the contact.
        impulse_contact.x = impulse_contact.x.min(0.0);
        debug!("impulse in contact coords after {}", impulse_contact);

        // Transform impulse back into global coords.
        let impulse = contact_to_global_transform.mul_vec3(impulse_contact);
        debug!("impulse in global coords {}", impulse);

        // Add impulse and impulsive torque to both bodies, reversing the impulse direction for the
        // second body.
        for ((i, entity), sign) in contact.entities.iter().enumerate().zip([1.0, -1.0].iter()) {
            let impulsive_torque = contact.relative_points[i].cross(*sign * impulse);

            if let Ok((inertia_tensor, mass, mut velocity, mut ang_velocity)) = q.q1_mut().get_mut(*entity) {
                velocity.add(mass.inverse() * sign * impulse);
                ang_velocity.add(inertia_tensor.inverse_global().mul_vec3(impulsive_torque));
            }
        }
    }
}

/// Calculates and applies a translation and rotation to each movable body involved in a collision
/// in order to remove the interpenetration between them.
fn resolve_interpenetration(
    contact_query: Query<&Contact>,
    q1: Query<(&InertiaTensor, &Mass)>,
    mut q2: Query<(&InertiaTensor, &mut PhysTransform)>,
) {
    for contact in contact_query.iter() {
        debug!("contact = {:?}", contact);
        // --- Calculate inertia.
        let mut linear_inertia = vec![];
        let mut angular_inertia = vec![];

        for (i, entity) in contact.entities.iter().enumerate() {
            let (inertia_tensor, mass) =
                q1.get(*entity).expect("Invalid contact entity");

            // Calculate the inertia in the direction of the contact normal.
            linear_inertia.push(mass.inverse());

            let impulsive_torque = contact.relative_points[i].cross(contact.normal);

            angular_inertia.push(inertia_tensor.inverse_global().mul_vec3(impulsive_torque)
                                    .cross(contact.relative_points[i])
                                    .dot(contact.normal));
        }

        let total_inverse_inertia: f64 = 1.0 / (linear_inertia.iter().sum::<f64>()
                                                + angular_inertia.iter().sum::<f64>());
        debug!("total inverse inertia {}", total_inverse_inertia);

        // --- Calculate movement required.
        let mut linear_move_vec = vec![];
        let mut angular_move_vec = vec![];

        for (i, sign) in (0..contact.entities.len()).zip(&[1.0, -1.0]) {
            linear_move_vec.push(sign * contact.penetration * linear_inertia[i] * total_inverse_inertia);
            angular_move_vec.push(sign * contact.penetration * angular_inertia[i] * total_inverse_inertia);
        }
        debug!("linear and ang move before limiting = {:?}, {:?}", linear_move_vec, angular_move_vec);

        // limit angular move to mitigate over-rotation issues.
        for i in 0..contact.entities.len() {
            let limit = constants::ANGULAR_LIMIT * contact.relative_points[i].length();

            if angular_move_vec[i] > limit {
                linear_move_vec[i] += angular_move_vec[i] - limit;
                angular_move_vec[i] = limit;
            } else if angular_move_vec[i] < -limit {
                linear_move_vec[i] += angular_move_vec[i] + limit;
                angular_move_vec[i] = -limit;
            }
        }
        debug!("linear and ang move after limiting = {:?}, {:?}", linear_move_vec, angular_move_vec);

        // --- Apply movement to each body.
        for (i, entity) in contact.entities.iter().enumerate() {
            let (inertia_tensor, mut phys_transform) = q2.get_mut(*entity).expect("Entity does not exist!");

            debug!("pos before = {}", phys_transform.translation);
            phys_transform.translation += linear_move_vec[i] * -contact.normal;
            debug!("pos after = {}", phys_transform.translation);

            // impulsive torque per unit impulse = rel_pos x normal
            // delta_omega per unit impulse = I^-1 * impulsive_torque per unit impulse.
            //
            // rotation per unit move = delta_omega per unit impulse / delta_v per unit impulse
            //
            if (angular_inertia[i].abs() - constants::LOW_ROTATION_THRESHOLD) >= 0.0 {
                let rotation_change = inertia_tensor.inverse_global().mul_vec3(contact.relative_points[i].cross(contact.normal))
                                        * (angular_move_vec[i] / angular_inertia[i]);

                debug!("rotation before = {}", phys_transform.rotation);
                phys_transform.rotation = phys_transform.rotation
                    .mul_quat(DQuat::from_xyzw(rotation_change.x, rotation_change.y, rotation_change.z, 0.0));
                debug!("rotation after = {}", phys_transform.rotation);
            };

        }
    }
}

/// Purges all Contact Entitys from the ECS.
fn remove_contacts(
    mut commands: Commands,
    contact_entities: Query<Entity, With<Contact>>,
) {
    for entity in contact_entities.iter() {
        commands.entity(entity).despawn();
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
    let mut y = if normal.dot(DVec3::X).abs() - normal.dot(DVec3::Y).abs() > 0.000001 {
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

    debug!("contact velocity in global coords = {}", result);

    // convert to contact coords
    result = global_to_contact_transform.mul_vec3(result);

    debug!("contact velocity in contact coords = {}", result);

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
        let basis = calc_contact_basis(normal).transpose();
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
