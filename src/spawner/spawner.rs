use bevy::{
    prelude::*,
    math::DVec3,
};
use rand::prelude::*;

use crate::{
    physics::prelude::*,
    user_interaction::{KeyboardControlled, Player},
};

// Plugin

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system())
            .add_startup_system(spawn_cuboids.system())
            .add_startup_system(spawn_fan.system())
            .add_startup_system(spawn_spheres.system());
    }
}

// Systems

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mass = 10.0;
    let radius = 1.0;
    let radius_f64 = 1.0_f64;

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius, subdivisions: 10 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, radius, 0.0),
        ..Default::default()
    })
    .insert(KeyboardControlled)
    .insert(Player)
    .insert_bundle(PhysicsColliderBundle::sphere(
            mass,
            radius_f64,
            PhysTransform::from_xyz(0.0, radius_f64, 0.0),
    ));
}

fn spawn_spheres(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let qty = 10;
    let mass = 10.0;
    let radius: f64 = 1.0;

    let mut rng = thread_rng();
    for _ in 0..qty {
        let x: f64 =  rng.gen_range(-50.0..50.0);
        let y: f64 =  rng.gen_range(radius..20.0);
        let z: f64 =  rng.gen_range(-50.0..50.0);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius as f32,
                subdivisions: 10,
            })),
            material: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
            ..Default::default()
        })
        .insert_bundle(PhysicsColliderBundle::sphere(
            mass,
            radius,
            PhysTransform::from_xyz(x, y, z),
        ));
    }
}

fn spawn_cuboids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let qty = 10;
    let mass = 10.0;
    let extents: DVec3 = DVec3::new(1.0, 1.0, 1.0);

    let mut rng = thread_rng();
    for _ in 0..qty {
        let x: f64 =  rng.gen_range(-50.0..50.0);
        let y: f64 =  rng.gen_range(extents.y..20.0);
        let z: f64 =  rng.gen_range(-50.0..50.0);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                extents.x as f32 * 2.0,
                extents.y as f32 * 2.0,
                extents.z as f32 * 2.0,
            ))),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2).into()),
            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
            ..Default::default()
        })
        .insert_bundle(PhysicsColliderBundle::cuboid(
                mass,
                extents,
                PhysTransform::from_xyz(x, y, z),
        ));
    }
}

fn spawn_fan(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mass = 20.0;
    let extents = DVec3::new(1.0, 10.0, 1.0);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(
            extents.x as f32 * 2.0,
            extents.y as f32 * 2.0,
            extents.z as f32 * 2.0,
        ))),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 10.0, -10.0),
        ..Default::default()
    })
    .insert_bundle(PhysicsColliderBundle::cuboid(
            mass,
            extents,
            PhysTransform::from_xyz(0.0, 10.0, -10.0),
    ))
    .insert(Rotator::new(DVec3::Z, DVec3::new(0.0, 5.0, 0.0), 10.0));
}

// TODO port old code
//
//    fn spawn_boundaries(&mut self) {
//        let planes = vec![
//            // Top
//            (vector![0.0, 1.0], vector![constants::FMIN_X, constants::FMIN_Y]),
//            // Bottom
//            (vector![0.0, -1.0], vector![constants::FMAX_X, constants::FMAX_Y]),
//            // Left
//            (vector![1.0, 0.0], vector![constants::FMIN_X, constants::FMIN_Y]),
//            // Right
//            (vector![-1.0, 0.0], vector![constants::FMAX_X, constants::FMAX_Y]),
//        ];
//
//        for (n, pos) in planes {
//            self.world.create_entity()
//                      .with(Position { vector: pos })
//                      // physics
//                      .with(Mass { value: f64::INFINITY, inverse: 0.0 })
//                      .with(Velocity { vector: vector![0.0, 0.0] })
//                      .with(BoundaryCollider::new(n))
//                      .build();
//        }
//    }
//}
