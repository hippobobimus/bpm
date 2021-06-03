use lazy_static::lazy_static;
use sdl2::rect::Rect;

// Game loop
//pub static TIMESTEP_NS: u32 = 1_000_000_000 / 20; // 20 Hz

// Physics
pub static LOW_VELOCITY_THRESHOLD: f64 = 5.0;
pub static PLAYER_PROPULSION_FORCE: f64 = 100.0;
pub static RESISTANCE_COEFFICIENT: f64 = 0.1;

// Animation
pub static SPRITE_ANIMATION_FPS: u32 = 4;
// Spritesheet
lazy_static! {
    pub static ref SPRITESHEET_INITIAL_FRAME: Rect = Rect::new(0, 0, 16, 17);
}
pub static FRAMES_PER_ANIMATION_CYCLE: i32 = 4;
