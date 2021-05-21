mod entity {
    use crate::direction::Direction;
    use sdl2::rect::{Point, Rect};
    use std::collections::HashSet;
    use std::collections::VecDeque;

const PLAYER_MOVEMENT_SPEED: i32 = 10;

    #[derive(Debug)]
    pub struct Entity {
        direction: Direction,
        directions_queue: VecDeque<Direction>,
        directions_active: HashSet<Direction>,
        pub position: Point,
        speed: i32,
        pub sprite: Rect,
    }

    impl Entity {
        pub fn new(position: Point, sprite: Rect) -> Entity {
            Entity {
                direction: Direction::RIGHT,
                directions_queue: VecDeque::new(),
                directions_active: HashSet::new(),
                position: position,
                speed: 0,
                sprite: sprite,
            }
        }
    
        pub fn update(&mut self) {
            self.update_velocity();
            self.update_position();
        }
    
        fn update_velocity(&mut self) {
            while let Some(dir) = self.directions_queue.front() {
                if self.directions_active.contains(dir) {
                    self.direction = *dir;
                    self.speed = PLAYER_MOVEMENT_SPEED;
                    return;
                } else {
                    self.directions_queue.pop_front();
                }
            }
        
            self.speed = 0;
        }
    
        fn update_position(&mut self) {
            self.position = self.position.offset(self.speed * self.direction.dx(), self.speed * self.direction.dy());
        }
    
        pub fn activate_direction(&mut self, direction: Direction) {
            self.directions_queue.push_front(direction);
            self.directions_active.insert(direction);
        }
    
        pub fn deactivate_direction(&mut self, direction: Direction) {
            self.directions_active.remove(&direction);
        }
    }
}

mod direction {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum Direction {
        LEFT,
        RIGHT,
        UP,
        DOWN,
    }
    
    impl Direction {
        pub fn dx(&self) -> i32 {
            match self {
                Direction::LEFT => -1,
                Direction::RIGHT => 1,
                Direction::UP => 0,
                Direction::DOWN => 0,
            }
        }
        pub fn dy(&self) -> i32 {
            match *self {
                Direction::LEFT => 0,
                Direction::RIGHT =>0,
                Direction::UP => -1,
                Direction::DOWN => 1,
            }
        }
    }
}

mod renderer {
    use sdl2::render::{Texture, WindowCanvas};
    use sdl2::pixels::Color;
    use sdl2::rect::{Point, Rect};
    use crate::entity::Entity;

    pub fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture,
              entity: &Entity) -> Result<(), String> {
        canvas.set_draw_color(color);
        canvas.clear();
    
        let (width, height) = canvas.output_size()?;
    
        //for e in entity_list.iter() {
            let screen_position = entity.position + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, entity.sprite.width(),
                entity.sprite.height());
    
            canvas.copy(texture, entity.sprite, screen_rect)?;
        //}
    
        canvas.present();
    
        Ok(())
    }
}

mod runner {
    use sdl2::pixels::Color;
    use sdl2::event::Event;
    use sdl2::image::{InitFlag, LoadTexture};
    use sdl2::keyboard::Keycode;
    use sdl2::rect::{Point, Rect};
    use std::collections::HashMap;
    use std::time::Duration;
    use crate::entity::Entity;
    use crate::direction::Direction;
    use crate::renderer;

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
    
        let mut movement_keys_map = HashMap::new();
        movement_keys_map.insert(Keycode::Left, Direction::LEFT);
        movement_keys_map.insert(Keycode::Right, Direction::RIGHT);
        movement_keys_map.insert(Keycode::Up, Direction::UP);
        movement_keys_map.insert(Keycode::Down, Direction::DOWN);
    
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
                    Event::KeyDown { keycode: Some(k), repeat: false, .. } if movement_keys_map.contains_key(&k) => {
                        let dir = movement_keys_map.get(&k).unwrap(); // already verified contains
    
                        player.activate_direction(*dir);
                    },
                    Event::KeyUp { keycode: Some(k), repeat: false, .. } if movement_keys_map.contains_key(&k) => {
                        let dir = movement_keys_map.get(&k).unwrap();
    
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
}

fn main() -> Result<(), String> {
    runner::run()?;

    Ok(())
}
