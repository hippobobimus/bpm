use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

use crate::{
    //animator::Animator,
    constants,
    entities::SpawnerPlugin,
    //event_processor,
    //keyboard::Keyboard,
    //physics::{
        //collision_detection::CollisionDetectionSys,
        //collision_response::CollisionResponseSys,
        //forces::ForceSys,
        //integrator::IntegrationSys,
    //},
    //renderer,
    //resources::{DeltaTime, MovementCommandStack},
    //timing::Timing,
};

pub fn run() -> Result<(), String> {
    App::build()
        // plugins
        .add_plugins(DefaultPlugins)
        // --Debugging
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        // --Custom
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
    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 60.0 })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        ..Default::default()
    });

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 60.0, 90.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.0)),
        light: Light {
            range: 150.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

// TODO old code
//pub fn _old_run() -> Result<(), String> {
//    let sdl_context = sdl2::init()?;
//    let video_subsystem = sdl_context.video()?;
//    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
//
//    let window = video_subsystem
//        .window("Physics Simulation", constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT)
//        .position_centered()
//        .build()
//        .map_err(|e| e.to_string())?;
//
//    let mut canvas = window
//        .into_canvas()
//        .build()
//        .map_err(|e| e.to_string())?;
//
//    let texture_creator = canvas.texture_creator();
//    let textures = [
//        texture_creator.load_texture("assets/females/F_01.png")?,
//        texture_creator.load_texture("assets/females/F_02.png")?,
//        texture_creator.load_texture("assets/females/F_03.png")?,
//        texture_creator.load_texture("assets/females/F_04.png")?,
//        texture_creator.load_texture("assets/females/F_05.png")?,
//        texture_creator.load_texture("assets/females/F_06.png")?,
//        texture_creator.load_texture("assets/females/F_07.png")?,
//        texture_creator.load_texture("assets/females/F_08.png")?,
//        texture_creator.load_texture("assets/females/F_09.png")?,
//        texture_creator.load_texture("assets/females/F_10.png")?,
//        texture_creator.load_texture("assets/females/F_11.png")?,
//        texture_creator.load_texture("assets/females/F_12.png")?,
//        texture_creator.load_texture("assets/males/M_01.png")?,
//        texture_creator.load_texture("assets/males/M_02.png")?,
//        texture_creator.load_texture("assets/males/M_03.png")?,
//        texture_creator.load_texture("assets/males/M_04.png")?,
//        texture_creator.load_texture("assets/males/M_05.png")?,
//        texture_creator.load_texture("assets/males/M_06.png")?,
//        texture_creator.load_texture("assets/males/M_07.png")?,
//        texture_creator.load_texture("assets/males/M_08.png")?,
//        texture_creator.load_texture("assets/males/M_09.png")?,
//        texture_creator.load_texture("assets/males/M_10.png")?,
//        texture_creator.load_texture("assets/males/M_11.png")?,
//        texture_creator.load_texture("assets/males/M_12.png")?,
//    ];
//
//    let mut world = specs::prelude::World::new();
//
//    // dispatcher 
//    let mut dispatcher = specs::prelude::DispatcherBuilder::new()
//        .with(Timing, "Timing", &[])
//        .with(Keyboard, "Keyboard", &["Timing"])
//        .with(ForceSys, "Forces", &["Keyboard", "Timing"])
//        .with(IntegrationSys, "Integrator", &["Forces"])
//        .with(CollisionDetectionSys::new(), "CollisionDetection", &["Keyboard", "Timing"])
//        .with(CollisionResponseSys, "CollisionResponse", &["CollisionDetection", "Forces", "Keyboard", "Timing"])
//        .build();
//
////    let mut dispatcher = DispatcherBuilder::new()
////        .with(Timing, "Timing", &[])
////        .with(Keyboard, "Keyboard", &["Timing"])
////        .with(CollisionDetectionSys::new(), "CollisionDetection", &["Keyboard", "Timing"])
////        .with(ForceSys, "ExternalForces", &["CollisionDetection", "Keyboard", "Timing"])
////        .with(CollisionResponseSys, "CollisionResponse", &["CollisionDetection", "ExternalForces", "Keyboard", "Timing"])
////        .with(MovementSys, "Movement", &["CollisionDetection", "CollisionResponse", "ExternalForces", "Keyboard", "Timing"])
////        //.with(Animator, "Animator", &["Keyboard", "Timing"])
////        .build();
//
//    dispatcher.setup(&mut world);
//
//    // Not threadsafe, so rendering system must be independently setup and not added to the dispatcher
//    renderer::SystemData::setup(&mut world);
//
//    // initialise resources
//    let movement_command_queue: MovementCommandStack = MovementCommandStack::new();
//    let delta_time = DeltaTime::new();
//
//    world.insert(movement_command_queue);
//    world.insert(delta_time);
//
//    // entities
//    entities::setup_initial_entities(&mut world);
//
//    // game loop
//    let mut event_pump = sdl_context.event_pump()?;
//    let i = 0;
//    let mut exit;
//    'running: loop {
//        // Event processing
//        exit = event_processor::process_events(&world, &mut event_pump);
//        if exit { break 'running };
//
//        // Update
//        //i = (i + 1) % 255; // update background color
//        let background_colour = Color::RGB(i, 64, 255 -i);
//        dispatcher.dispatch(&world);
//        world.maintain();
//
//        // Render
//        renderer::render(&mut canvas, background_colour, &textures, world.system_data())?;
//    }
//
//    Ok(())
//}
