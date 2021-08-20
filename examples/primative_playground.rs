use bevy::{
    prelude::*,
    diagnostic::{
        //FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
    math::{
        DQuat,
        DVec3,
    },
};
use rand::prelude::*;

use bpm::{
    physics::prelude::*,
    user_interaction::prelude::*,
};

// Bevy rendering uses 32 bit floats, whereas the physics engine uses 64 bit.
static SCREEN_WIDTH: f32 = 800.0;
static SCREEN_HEIGHT: f32 = 400.0;
// cubic player area with equal sides.
static PLAY_AREA_EXTENT: f64 = 50.0;
static PLAY_AREA_EXTENT_F32: f32 = 50.0;
static PLAY_AREA_SIZE_F32: f32 = 100.0;


pub fn main() -> Result<(), String> {
    App::build()
        .add_plugins(DefaultPlugins)
        // Add plugins for debugging.
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Add the bpm plugins.
        .add_plugin(UserInteractionPlugin)
        .add_plugin(PhysicsPlugin)
        // Add anti-aliasing and a window.
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Primative Playground Example".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        // Systems to configure cameras/lighting, spawn a variety of shapes and the 'player'.
        .add_startup_system(setup_camera_and_light.system())
        .add_startup_system(spawn_boundaries.system())
        .add_startup_system(spawn_player.system())
        .add_startup_system(spawn_cuboids.system())
        .add_startup_system(spawn_spheres.system())
        .add_startup_system(spawn_fan.system())
        // systems used during development
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(bevy::window::exit_on_window_close_system.system())
        .run();

    Ok(())
}

// Systems

fn setup_camera_and_light(
    mut commands: Commands,
) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 120.0, 180.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 120.0, 0.0)),
        light: Light {
            intensity: 10_000.0,
            range: 150.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

// spawns the physical boundaries of the play area
fn spawn_boundaries(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = Color::rgb(0.0, 1.0, 0.0);
    let rotation = vec![
        // Floor
        (DQuat::IDENTITY, Quat::IDENTITY),
        // Back wall
        (DQuat::from_rotation_x(0.5 * std::f64::consts::PI),
         Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
    ];
    let translation = vec![
        // Floor
        (DVec3::ZERO, Vec3::ZERO),
        // Back wall
        (DVec3::new(0.0, PLAY_AREA_EXTENT, -PLAY_AREA_EXTENT),
         Vec3::new(0.0, PLAY_AREA_EXTENT_F32, -PLAY_AREA_EXTENT_F32)),
    ];

    for ((rotation, rotation_f32), (translation, translation_f32))
        in rotation.into_iter().zip(translation.into_iter())
    {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: PLAY_AREA_SIZE_F32 })),
            material: materials.add(color.into()),
            transform: Transform {
                rotation: rotation_f32,
                translation: translation_f32,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(PhysicsBoundaryBundle::new(
                PhysTransform::from_rotation_translation(rotation, translation),
        ));
    }
}

// Spawns a sphere that can be controlled with the keyboard. Up, down, left, right for movement in
// the x-z plane and spacebar to go up in the y-direction.
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mass = 10.0;
    let radius = 1.0_f64;
    let color = Color::rgb(1.0, 0.0, 0.0);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: radius as f32, subdivisions: 10 })),
        material: materials.add(color.into()),
        transform: Transform::from_xyz(0.0, radius as f32 + 0.01, 0.0),
        ..Default::default()
    })
    .insert(KeyboardControlled)
    .insert(Player)
    .insert_bundle(PhysicsColliderBundle::sphere(
            mass,
            radius,
            PhysTransform::from_xyz(0.0, radius + 0.01, 0.0),
    ));
}

// spawns a number of spheres in random locations within the play area.
fn spawn_spheres(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let qty = 10;
    let mass = 10.0;
    let radius: f64 = 1.0;
    let color = Color::rgb(1.0, 1.0, 0.35);

    let mut rng = thread_rng();
    for _ in 0..qty {
        // the origin is at the centre of the x-z plane.
        let x: f64 =  rng.gen_range(-PLAY_AREA_EXTENT..PLAY_AREA_EXTENT);
        let y: f64 =  rng.gen_range(radius..2.0 * PLAY_AREA_EXTENT - radius);
        let z: f64 =  rng.gen_range(-PLAY_AREA_EXTENT..PLAY_AREA_EXTENT);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius as f32,
                subdivisions: 10,
            })),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
            ..Default::default()
        })
        // add the physics bundle to allow this sphere to participate in collisions.
        .insert_bundle(PhysicsColliderBundle::sphere(
            mass,
            radius,
            PhysTransform::from_xyz(x, y, z),
        ));
    }
}

// spawns a number of cuboids at random locations within the play area.
fn spawn_cuboids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let qty = 10;
    let mass = 10.0;
    let extents: DVec3 = DVec3::new(1.0, 1.0, 1.0);
    let color = Color::rgb(0.6, 0.0, 0.6);

    let mut rng = thread_rng();
    for _ in 0..qty {
        // the origin is at the centre of the x-z plane.
        let x: f64 =  rng.gen_range(-PLAY_AREA_EXTENT..PLAY_AREA_EXTENT);
        let y: f64 =  rng.gen_range(extents.y..2.0 * PLAY_AREA_EXTENT - extents.y);
        let z: f64 =  rng.gen_range(-PLAY_AREA_EXTENT..PLAY_AREA_EXTENT);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                extents.x as f32 * 2.0,
                extents.y as f32 * 2.0,
                extents.z as f32 * 2.0,
            ))),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
            ..Default::default()
        })
        // add the physics bundle to allow this cuboid to participate in collisions.
        .insert_bundle(PhysicsColliderBundle::cuboid(
                mass,
                extents,
                PhysTransform::from_xyz(x, y, z),
        ));
    }
}

// Spawns a perpetually rotating 'fan' (elongated cuboid).
fn spawn_fan(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mass = 20.0;
    let extents = DVec3::new(1.0, 10.0, 1.0);
    let color = Color::rgb(0.0, 0.0, 1.0);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(
            extents.x as f32 * 2.0,
            extents.y as f32 * 2.0,
            extents.z as f32 * 2.0,
        ))),
        material: materials.add(color.into()),
        transform: Transform::from_xyz(0.0, 10.0, -10.0),
        ..Default::default()
    })
    // add the physics bundle to allow this cuboid to participate in collisions.
    .insert_bundle(PhysicsColliderBundle::cuboid(
            mass,
            extents,
            PhysTransform::from_xyz(0.0, 10.0, -10.0),
    ))
    // add a force/torque generator component to create rotation. Give the axis of rotation, the
    // point on the body where the force is to be applied and the force magnitude.
    .insert(Rotator::new(DVec3::Z, DVec3::new(0.0, 5.0, 0.0), 10.0));
}
