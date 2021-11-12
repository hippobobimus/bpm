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
        // Add plugins for debugging.
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

// Resource to indicate whether the spawn player system has been run.
#[derive(Default)]
struct HasRun(bool);

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

// Spawns a stack of 27 uniform spheres, dropped from above the boundary plane.
// Runs each time the 'Return' key is pressed.
fn spawn_balls(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keys.just_pressed(KeyCode::Return) {
        return;
    }

    let mass = 0.125;
    let radius = 0.5;
    let colors = [
        Color::rgb(236.0/255.0, 54.0/255.0, 141.0/255.0),
        Color::rgb(81.0/255.0, 229.0/255.0, 1.0),
        Color::rgb(1.0, 1.0, 89.0/255.0),
    ];
    let (rows, levels, cols) = (3, 3, 3);
    let gap = 1.0;
    let offset = (rows / 2) as f64 * gap;
    let drop_height = 15.0;

    for i in 0..rows {
        for j in 0..levels {
            for k in 0..cols {
                let color = colors[i % colors.len()];
                let translation = DVec3::new(
                    i as f64 * gap - offset,
                    j as f64 * gap + radius + drop_height,
                    k as f64 * gap - offset
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
    }
}

// Spawns a physical boundary plane to form the 'floor'.
fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = Color::rgb(0.0, 1.0, 0.0);
    let size = 50.0;
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
