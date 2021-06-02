use lazy_static::lazy_static;
use sdl2::rect::Rect;

// Game loop
pub static TIME_STEP: f64 = 1.0 / 20.0;

// Physics
pub static LOW_VELOCITY_THRESHOLD: f64 = 5.0;
pub static PLAYER_PROPULSION_FORCE: f64 = 100.0;
pub static RESISTANCE_COEFFICIENT: f64 = 0.1;

// Animation
pub static TICKS_PER_FRAME: i32 = 5;
// Spritesheet
lazy_static! {
    pub static ref SPRITESHEET_INITIAL_FRAME: Rect = Rect::new(0, 0, 16, 17);
}
pub static FRAMES_PER_ANIMATION: i32 = 4;
