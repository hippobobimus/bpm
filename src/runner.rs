use lazy_static::lazy_static;
use sdl2::{
    event::Event,
    EventPump,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};
use specs::prelude::*;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    time::Duration,
};
use crate::{
    animator::Animator,
    commands::MovementCommand,
    components::*,
    constants,
    direction::Direction,
    entity,
    keyboard::Keyboard,
    physics::Physics,
    renderer,
};

// A mapping that associates keycodes to the cardinal movement directions.
lazy_static! {
    static ref MOVEMENT_KEYS_MAP: HashMap<Keycode, Direction> = {
        let mut map = HashMap::new();
        map.insert(Keycode::Left, Direction::Left);
        map.insert(Keycode::Right, Direction::Right);
        map.insert(Keycode::Up, Direction::Up);
        map.insert(Keycode::Down, Direction::Down);
        map
    };
}

/// Returns true if the game loop should be exited.
fn process_events(world: &World, event_pump: &mut EventPump) -> bool {
        let mut movement_command = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },
                Event::KeyDown { keycode: Some(k), repeat: false, .. } if MOVEMENT_KEYS_MAP.contains_key(&k) => {
                    let dir = MOVEMENT_KEYS_MAP.get(&k).unwrap(); // already verified contains

                    movement_command = Some(MovementCommand::Move(*dir));
                },
                Event::KeyUp { keycode: Some(k), repeat: false, .. } if MOVEMENT_KEYS_MAP.contains_key(&k) => {
                    //let dir = MOVEMENT_KEYS_MAP.get(&k).unwrap();

                    movement_command = Some(MovementCommand::Stop);
                    //player.deactivate_direction(*dir);
                },
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        false
}

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
    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);

    // entities
    let player_spritesheet = 0;
    let player_origin_frame = Rect::new(177, 0, 14, 24);

    let player_animation = MovementAnimation {
        current_frame: 0,
        left_frames: entity::sprite_animation_frames(player_spritesheet, player_origin_frame, Direction::Left),
        right_frames: entity::sprite_animation_frames(player_spritesheet, player_origin_frame, Direction::Right),
        up_frames: entity::sprite_animation_frames(player_spritesheet, player_origin_frame, Direction::Up),
        down_frames: entity::sprite_animation_frames(player_spritesheet, player_origin_frame, Direction::Down),
    };

    world.create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: constants::DEFAULT_PLAYER_DIRECTION,
            active_directions: HashSet::new(),
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    // game loop
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Event processing
        let exit = process_events(&world, &mut event_pump);
        if exit {
            break 'running
        };

        // Update
        //player.update();
        i = (i + 1) % 255; // update bckgrnd color
        dispatcher.dispatch(&world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 -i), &textures, world.system_data())?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
