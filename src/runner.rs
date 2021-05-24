use lazy_static::lazy_static;
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};
use std::{
    collections::HashMap,
    time::Duration,
};
use crate::{
    direction::Direction,
    entity::Entity,
    renderer,
};

lazy_static! {
    static ref MOVEMENT_KEYS_MAP: HashMap<Keycode, Direction> = {
        let mut map = HashMap::new();
            map.insert(Keycode::Left, Direction::LEFT);
            map.insert(Keycode::Right, Direction::RIGHT);
            map.insert(Keycode::Up, Direction::UP);
            map.insert(Keycode::Down, Direction::DOWN);
            map
    };
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
    let texture = texture_creator.load_texture("assets/main_char_spritesheet.png")?;

    //let mut entity_list = LinkedList::new();

    let mut player = Entity::new(Point::new(0, 0), Rect::new(30, 21, 12, 16));

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Event processing
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(k), repeat: false, .. } if MOVEMENT_KEYS_MAP.contains_key(&k) => {
                    let dir = MOVEMENT_KEYS_MAP.get(&k).unwrap(); // already verified contains

                    player.activate_direction(*dir);
                },
                Event::KeyUp { keycode: Some(k), repeat: false, .. } if MOVEMENT_KEYS_MAP.contains_key(&k) => {
                    let dir = MOVEMENT_KEYS_MAP.get(&k).unwrap();

                    player.deactivate_direction(*dir);
                },
                _ => {}
            }
        }

        // Update
        player.update();
        i = (i + 1) % 255; // update bckgrnd color

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 -i), &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
