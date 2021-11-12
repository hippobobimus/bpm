use bevy::{
    prelude::*,
    diagnostic::{
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
    math::{
        DQuat,
        DVec3,
    },
};
use rand::prelude::*;

use bpm::{
    // uncomment for optional debugging overlay //debug::prelude::*,
    physics::prelude::*,
    user_interaction::prelude::*,
};

static SCREEN_WIDTH: f32 = 800.0;
static SCREEN_HEIGHT: f32 = 400.0;

pub fn main() -> Result<(), String> {
    App::build()
        .add_plugins(DefaultPlugins)
        // Add Bevy plugins for debugging.
        .add_plugin(LogDiagnosticsPlugin {
            debug: true,
            wait_duration: std::time::Duration::ZERO,
            ..Default::default()
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Add the bpm plugins.
        .add_plugin(UserInteractionPlugin)
        .add_plugin(PhysicsPlugin)
        // uncomment for optional debugging overlay //.add_plugin(BpmDebugPlugin)
        // Add anti-aliasing and a window.
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Two Spheres Example".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        // Add custom resource.
        .init_resource::<HasRun>()
        // Systems to configure cameras/lighting, spawn a variety of shapes and the 'player'.
        .add_startup_system(setup_camera_and_light.system())
        .add_startup_system(spawn_plane.system())
        .add_system(spawn_player.system())
        .add_system(spawn_balls.system())
        // Systems to control exiting.
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(bevy::window::exit_on_window_close_system.system())
        .run();

    Ok(())
}

// Resource to indicate whether the spawn player and spawn balls systems have been run.
#[derive(Default)]
struct HasRun(bool, bool);

// Systems

fn setup_camera_and_light(
    mut commands: Commands,
) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-10.0, 15.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(20.0, 40.0, 20.0),
        light: Light {
            intensity: 10_000.0,
            range: 1500.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

// Spawns a sphere that can be controlled with the keyboard.
// Runs when the 'Return' key is pressed and runs only once.
//
// 'Up/Down/Left/Right' or 'h/j/k/l' = movement in the x-z plane.
// 'Spacebar' or 'w' = movement up in the positive y direction.
// 's' = movement down in the positive y direction.
fn spawn_player(
    mut has_run: ResMut<HasRun>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if has_run.0 || !keys.just_pressed(KeyCode::Return) {
        return;
    }
    has_run.0 = true;

    let mass = 1.0;
    let radius = 1.0;
    let color = Color::rgb(1.0, 0.0, 0.0);
    let translation = DVec3::new(0.0, radius + 10.0, 0.0);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: radius as f32, subdivisions: 10 })),
        material: materials.add(color.into()),
        transform: Transform::from_translation(translation.as_f32()),
        ..Default::default()
    })
    .insert(KeyboardControlled)
    .insert(Player)
    .insert_bundle(PhysicsColliderBundle::sphere(
            mass,
            radius,
            PhysTransform::from_translation(translation),
    ));
}

// Spawns a quantity of randomly sized spheres with a mass proportional to their volume, at random
// positions within a certain spread of the x-z plane and drops them at a height within a certain
// range of the y-axis.
// Runs when the 'Return' key is pressed and runs only once.
fn spawn_balls(
    mut has_run: ResMut<HasRun>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if has_run.1 || !keys.just_pressed(KeyCode::Return) {
        return;
    }
    has_run.1 = true;

    let mut rng = thread_rng();
    let colors = [
        Color::rgb(236.0/255.0, 54.0/255.0, 141.0/255.0),
        Color::rgb(81.0/255.0, 229.0/255.0, 1.0),
        Color::rgb(1.0, 1.0, 89.0/255.0),
    ];
    let min_radius = 0.5;
    let max_radius = 0.75;
    let min_drop_height = 10.0;
    let max_drop_height = 30.0;
    let x_z_spread = -20.0..20.0;
    let qty = 1000;

    for i in 0..qty {
        let color = colors[i % colors.len()];
        let radius: f64 = rng.gen_range(min_radius..max_radius);
        let mass = radius.powi(3);
        let translation = DVec3::new(
            rng.gen_range(x_z_spread.clone()),
            rng.gen_range(radius + min_drop_height..radius + max_drop_height),
            rng.gen_range(x_z_spread.clone()),
        );

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: radius as f32,
                subdivisions: 10 })),
            material: materials.add(color.into()),
            transform: Transform::from_translation(translation.as_f32()),
            ..Default::default()
        })
        .insert_bundle(PhysicsColliderBundle::sphere(
                mass,
                radius,
                PhysTransform::from_translation(translation),
        ));
    }
}

// Spawns a physical boundary plane to form the 'floor'.
fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = Color::rgb(0.0, 1.0, 0.0);
    let size = 100.0;
    let rotation = DQuat::IDENTITY;
    let translation = DVec3::ZERO;

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size })),
        material: materials.add(color.into()),
        transform: Transform {
            rotation: rotation.as_f32(),
            translation: translation.as_f32(),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert_bundle(PhysicsBoundaryBundle::new(
            PhysTransform::from_rotation_translation(rotation, translation),
    ));
}
