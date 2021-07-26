use bevy::math::DVec3;
use lazy_static::lazy_static;
use sdl2::rect::Rect;

// Screen canvas area
pub static SCREEN_WIDTH: u32 = 800;
pub static SCREEN_HEIGHT: u32 = 600;
pub static FSCREEN_CENTRE_X: f64 = 0.0;
pub static FSCREEN_CENTRE_Y: f64 = 0.0;
pub static FSCREEN_EXTENT_X: f64 = SCREEN_WIDTH as f64 * 0.5;
pub static FSCREEN_EXTENT_Y: f64 = SCREEN_HEIGHT as f64 * 0.5;
pub static FMAX_X: f64 = SCREEN_WIDTH as f64 * 0.5;
pub static FMAX_Y: f64 = SCREEN_HEIGHT as f64 * 0.5;
pub static FMIN_X: f64 = -FMAX_X;
pub static FMIN_Y: f64 = -FMAX_Y;

// Physics
pub static LOW_VELOCITY_THRESHOLD: f64 = 1.0;
// --Damping
pub static DAMPING_FACTOR: f64 = 0.999;
// --Drag
pub static DEFAULT_K1: f64 = 5.0;
pub static DEFAULT_K2: f64 = 5.0;
// --Gravity
lazy_static! {
    pub static ref DEFAULT_GRAVITY: DVec3 = DVec3::new(0.0, 0.0, 0.0); // value of 'g'.
}
// Mass
pub static DEFAULT_MASS: f64 = 10.0;
pub static DEFAULT_INVERSE_MASS: f64 = 0.1;
// --Thrust
pub static DEFAULT_THRUST: f64 = 500.0;

// Animation
pub static SPRITE_ANIMATION_FPS: u32 = 4;
// Spritesheet
lazy_static! {
    pub static ref SPRITESHEET_INITIAL_FRAME: Rect = Rect::new(0, 0, 16, 17);
}
pub static FRAMES_PER_ANIMATION_CYCLE: i32 = 4;
