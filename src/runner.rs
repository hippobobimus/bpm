use bevy::{
    prelude::*,
    diagnostic::{
        //FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
};

use crate::{
    constants,
    spawner::SpawnerPlugin,
    keyboard::KeyboardPlugin,
    physics::PhysicsPlugin,
};

pub fn run() -> Result<(), String> {
    App::build()
        // plugins
        .add_plugins(DefaultPlugins)
        // --Debugging
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        // --Custom
        .add_plugin(KeyboardPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(SpawnerPlugin)

        // resources
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Physics Simulation".to_string(),
            width: constants::SCREEN_WIDTH as f32,
            height: constants::SCREEN_HEIGHT as f32,
            resizable: false,
            ..Default::default()
        })
        // events

        // startup systems
        .add_startup_system(setup.system())
        // systems
        // systems used during development
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(bevy::window::exit_on_window_close_system.system())
        .run();

    Ok(())
}

// Systems

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Planes
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_x(0.5 * std::f32::consts::PI),
            Vec3::new(0.0, 100.0, -100.0),
        )),
        ..Default::default()
    });

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
