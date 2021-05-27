use sdl2::{
    image::{InitFlag, LoadTexture},
    pixels::Color,
    rect::{Point, Rect},
};
use specs::prelude::*;

use std::{
    collections::HashSet,
    time::Duration,
};

use crate::{
    animator::Animator,
    resources::MovementCommandStack,
    components::*,
    constants,
    event_processor,
    keyboard::Keyboard,
    physics::Physics,
    renderer,
};

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("DinoScore", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/char_spritesheet.png")?,
    ];

    let mut world = World::new();

    // dispatcher 
    let mut dispatcher = DispatcherBuilder::new()
        .with(Keyboard, "Keyboard", &[])
        // depend on keyboard setting velocity before position and animation
        .with(Physics, "Physics", &["Keyboard"])
        .with(Animator, "Animator", &["Keyboard"])
        .build();

    dispatcher.setup(&mut world);

    // Not threadsafe, so rendering system must be independently setup and not added to the dispatcher
    renderer::SystemData::setup(&mut world);

    // initialise resources
    let movement_command_queue: MovementCommandStack = MovementCommandStack::new();
    world.insert(movement_command_queue);

    // entities
    let player_spritesheet = 0;
    let player_initial_frame = Rect::new(177, 0, 14, 24);
    let player_animation = MovementAnimation::new(player_spritesheet, player_initial_frame);

    world.create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: constants::DEFAULT_PLAYER_DIRECTION,
            active_directions: HashSet::new(),
        })
        .with(player_animation.right_frames[0].clone()) // Sprite
        .with(player_animation)
        .build();

    // game loop
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Event processing
        let exit = event_processor::process_events(&world, &mut event_pump);
        if exit {
            break 'running
        };

        // Update
        i = (i + 1) % 255; // update bckgrnd color
        dispatcher.dispatch(&world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 -i), &textures, world.system_data())?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
