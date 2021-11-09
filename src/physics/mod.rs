pub mod components;
mod entity;
mod oct_tree;
pub mod shapes;
mod systems;

// Re-exports
pub use entity::PhysicsColliderBundle;

/// 'use physics::prelude::*;' to import common components, shapes, bundles and plugins.
pub mod prelude {
    #[doc(hidden)]
    pub use super::components::{
        AngularVelocity,
        BoundaryCollider,
        Collider,
        Drag,
        Gravity,
        InertiaTensor,
        Mass,
        PhysTransform,
        Rotator,
        Thrust,
        Velocity,
    };
    pub use super::entity::{
        PhysicsBoundaryBundle,
        PhysicsColliderBundle,
    };
    pub use super::shapes::{
        CollisionPrimative,
        Cuboid,
        Plane,
        Sphere,
    };
    pub use super::PhysicsPlugin;
}

use bevy::prelude::*;

use systems::{
    cache_update,
    collision_detection,
    collision_response,
    force_and_torque,
    integrator,
    transform_sync,
};

/// System label covering all physics systems.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct BpmPhysics;

/// System labels covering physics sub-systems.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum BpmPhysicsSystems {
    ForceAndTorque,
    Integrator,
    CollisionDetection,
    CollisionResponse,
    TransformSync,
    CacheUpdatePrimary,
    CacheUpdateSecondary,
}

/// A Bevy plugin that adds systems to support rigid-body physics, including; force/torque
/// accumulation, integration, collision detection and collision resolution.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Up to and including contact generation, but nothing that depends on those contacts.
        static PRIMARY: &str = "Primary";
        // Systems that utilise contact entities generated during the current frame.
        static SECONDARY: &str = "Secondary";

        app
            .add_stage_after(CoreStage::Update, PRIMARY, SystemStage::parallel())
            .add_stage_after(PRIMARY, SECONDARY, SystemStage::parallel())
            .add_startup_system(
                collision_detection::initialize.system()
            )
            .add_system_set_to_stage(
                PRIMARY,
                force_and_torque::get_system_set()
                    .label(BpmPhysicsSystems::ForceAndTorque)
                    .label(BpmPhysics)
            )
            .add_system_set_to_stage(
                PRIMARY,
                integrator::get_system_set()
                    .label(BpmPhysicsSystems::Integrator)
                    .label(BpmPhysics)
                    .after(BpmPhysicsSystems::ForceAndTorque)
            )
            .add_system_set_to_stage(
                PRIMARY,
                cache_update::get_system_set()
                    .label(BpmPhysicsSystems::CacheUpdatePrimary)
                    .label(BpmPhysics)
                    .after(BpmPhysicsSystems::Integrator)
            )
            .add_system_set_to_stage(
                PRIMARY,
                collision_detection::get_system_set()
                    .label(BpmPhysicsSystems::CollisionDetection)
                    .label(BpmPhysics)
                    .after(BpmPhysicsSystems::CacheUpdatePrimary)
            )
            .add_system_set_to_stage(
                SECONDARY,
                collision_response::get_system_set()
                    .label(BpmPhysicsSystems::CollisionResponse)
                    .label(BpmPhysics)
            )
            .add_system_set_to_stage(
                SECONDARY,
                cache_update::get_system_set()
                    .label(BpmPhysicsSystems::CacheUpdateSecondary)
                    .label(BpmPhysics)
                    .after(BpmPhysicsSystems::CollisionResponse)
            )
            .add_system_set_to_stage(
                SECONDARY,
                transform_sync::get_system_set()
                    .label(BpmPhysicsSystems::TransformSync)
                    .label(BpmPhysics)
                    .after(BpmPhysicsSystems::CollisionResponse)
            );
    }
}
