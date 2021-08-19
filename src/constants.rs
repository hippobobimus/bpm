use bevy::math::DVec3;
use lazy_static::lazy_static;

// Screen canvas area
pub static SCREEN_WIDTH: u32 = 800;
pub static SCREEN_HEIGHT: u32 = 600;

// Oct-Tree
pub static MAX_OCT_TREE_DEPTH: i32 = 5;

// Play area
pub static PLAY_AREA_CENTRE_X: f64 = 0.0;
pub static PLAY_AREA_CENTRE_Y: f64 = 100.0;
pub static PLAY_AREA_CENTRE_Z: f64 = 0.0;
pub static PLAY_AREA_EXTENT_X: f64 = 100.0;
pub static PLAY_AREA_EXTENT_Y: f64 = 100.0;
pub static PLAY_AREA_EXTENT_Z: f64 = 100.0;

// Physics
// --Float precision
pub static LOW_VELOCITY_THRESHOLD: f64 = 1.0;
// --Damping
pub static DAMPING_FACTOR: f64 = 0.999;
pub static ANGULAR_DAMPING_FACTOR: f64 = 0.9;
// --Drag
pub static DEFAULT_K1: f64 = 5.0;
pub static DEFAULT_K2: f64 = 5.0;
// --Gravity
lazy_static! {
    pub static ref DEFAULT_GRAVITY: DVec3 = DVec3::new(0.0, 0.0, 0.0); // value of 'g'.
}
// --Mass
pub static DEFAULT_MASS: f64 = 10.0;
pub static DEFAULT_INVERSE_MASS: f64 = 0.1;
// --Body dimensions
pub static DEFAULT_RADIUS: f64 = 1.0;
// --Thrust
pub static DEFAULT_THRUST: f64 = 1000.0;
