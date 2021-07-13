use nalgebra::{
    base::Vector2,
    vector,
};
use sdl2::{
    pixels::Color,
};
use specs::prelude::*;
use specs_derive::Component;

use crate::{
    constants,
    direction::Direction,
    shapes::{Circle, Line, Plane, Polygon},
};

// Marker components

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Player;

// Standard components

// Physics

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Mass {
    pub value: f64,
    pub inverse: f64,
}

impl Mass {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            inverse: 1.0 / value,
        }
    }

    pub fn from_inverse(inverse: f64) -> Self {
        Self {
            value: 1.0 / inverse,
            inverse,
        }
    }

    pub fn inverse(&self) -> f64 {
        self.inverse
    }

    pub fn is_infinite(&self) -> bool {
        self.inverse == 0.0
    }

    pub fn is_normal(&self) -> bool {
        self.value.is_normal()
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Position {
    pub vector: Vector2<f64>,
}

impl Position {
    pub fn new(vector: Vector2<f64>) -> Self {
        Self { vector }
    }

    pub fn transform(&mut self, t: &Vector2<f64>) {
        self.vector += t;
    }

    pub fn vector(&self) -> &Vector2<f64> {
        &self.vector
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Velocity {
    pub vector: Vector2<f64>,
}

impl Velocity {
    pub fn new(vector: Vector2<f64>) -> Self {
        Self { vector }
    }

    pub fn scale(&mut self, s: f64) {
        self.vector * s;
    }

    pub fn transform(&mut self, t: &Vector2<f64>) {
        self.vector += t;
    }

    pub fn vector(&self) -> &Vector2<f64> {
        &self.vector
    }
}

// -- Force Accumulator

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Force {
    total: Vector2<f64>,
}

impl Force {
    pub fn new() -> Self {
        Self { 
            total: Default::default(),
        }
    }

    pub fn add_force(&mut self, f: &Vector2<f64>) {
        self.total += f;
    }

    pub fn reset(&mut self) {
        self.total.x = 0.0;
        self.total.y = 0.0;
    }

    pub fn vector(&self) -> &Vector2<f64> {
        &self.total
    }
}

// -- Force generators

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Drag {
    k1: f64,
    k2: f64,
}

impl Drag {
    pub fn new(k1: f64, k2: f64) -> Self {
        Self { k1, k2 }
    }

    pub fn force(&self, v: &Vector2<f64>) -> Vector2<f64> {
        let v_mag = v.magnitude();
        let coeff = self.k1 * v_mag + self.k2 * v_mag.powi(2);

        -coeff * v
    }
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            k1: constants::DEFAULT_K1,
            k2: constants::DEFAULT_K2,
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Gravity {
    g: Vector2<f64>,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            g: constants::DEFAULT_GRAVITY,
        }
    }
}

impl Gravity {
    pub fn new(g: Vector2<f64>) -> Self {
        Self { g }
    }

    pub fn force(&self, m: f64) -> Vector2<f64> {
        m * self.g
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Thrust {
    force: Vector2<f64>,
    magnitude: f64,
}

impl Default for Thrust {
    fn default() -> Self {
        Self {
            force: Default::default(),
            magnitude: constants::DEFAULT_THRUST,
        }
    }
}

impl Thrust {
    pub fn new(magnitude: f64) -> Self {
        Self {
            force: Default::default(),
            magnitude,
        }
    }

    pub fn disengage(&mut self) {
        self.force.x = 0.0;
        self.force.y = 0.0;
    }

    pub fn engage(&mut self, dir: &Direction) {
        self.force = self.magnitude * dir.unit_vector();
    }

    pub fn force(&self) -> &Vector2<f64> {
        &self.force
    }
}

// --Collisions

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collision {
    pub ent_a: Entity,
    pub ent_b: Entity,
    pub normal: Vector2<f64>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CircleCollider {
    circle: Circle,
}

impl CircleCollider {
    pub fn new(radius: f64) -> Self {
        Self { circle: Circle::new(radius) }
    }

    pub fn circle(&self) -> &Circle {
        &self.circle
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PolygonCollider {
    polygon: Polygon,
}

impl PolygonCollider {
    pub fn new(vertices: &Vec<Vector2<f64>>) -> Self {
        Self {
            polygon: Polygon::new(vertices),
        }
    }

    pub fn polygon(&self) -> &Polygon {
        &self.polygon
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct BoundaryCollider {
    pub plane: Plane,
}

impl BoundaryCollider {
    pub fn new(normal: Vector2<f64>) -> Self {
        Self {
            plane: Plane::new(normal),
        }
    }

    pub fn normal(&self) -> &Vector2<f64> {
        self.plane.normal()
    }

    pub fn boundary(&self) -> &Plane {
        &self.plane
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Forces {
    pub propulsion: Vector2<f64>,
    pub drag: Vector2<f64>,
}

impl Forces {
    pub fn new(propulsion: Vector2<f64>, drag: Vector2<f64>) -> Self {
        Self { propulsion, drag }
    }
}

// Rendering

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct RenderableCircle {
    circle: Circle,
}

impl RenderableCircle {
    pub fn new(radius: f64) -> Self {
        Self {
            circle: Circle::new(radius),
        }
    }

    pub fn radius(&self) -> i16 {
        self.circle.radius() as i16
    }
}

// TODO
pub struct RenderableLine {
    line: Line,
}

impl RenderableLine {
    pub fn new(start: Vector2<f64>, end: Vector2<f64>) -> Self {
        Self {
            line: Line::new(start, end),
        }
    }

    pub fn start(&self, position: Vector2<f64>) -> Vector2<i16> {
        let temp = self.line.start() + position;
        vector![temp.x as i16, temp.y as i16]
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct RenderablePolygon {
    polygon: Polygon,
}

impl RenderablePolygon {
    /// Creates a new polygon with the given vertices. The vertices must be given as vectors
    /// relative to the polygon's centre and are assumed to be ordered such that iterating over
    /// them is equivalent to traversing the perimeter of the polygon.
    pub fn new(vertices: &Vec<Vector2<f64>>) -> Self {
        Self {
            polygon: Polygon::new(vertices),
        }
    }

    /// Returns a Vec of integer x-coords of the polygon's vertices.
    pub fn vx(&self, position: Vector2<f64>) -> Vec<i16> {
        self.polygon.vertices().iter().map(|v| (v.x + position.x) as i16).collect()
    }

    /// Returns a Vec of integer y-coords of the polygon's vertices.
    pub fn vy(&self, position: Vector2<f64>) -> Vec<i16> {
        self.polygon.vertices().iter().map(|v| (v.y + position.y) as i16).collect()
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct RenderColour {
    sdl_colour: Color,
}

impl RenderColour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            sdl_colour: Color::RGB(r, g, b),
        }
    }

    pub fn sdl_colour(&self) -> Color {
        self.sdl_colour
    }

    pub fn change_colour(&mut self, r: u8, g: u8, b: u8) {
        self.sdl_colour = Color::RGB(r, g, b);
    }
}

//#[derive(Clone, Component, Debug)]
//#[storage(VecStorage)]
//pub struct Sprite {
//    pub spritesheet: usize,
//    pub region: Rect,
//    pub flip_horizontal: bool,
//}
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct MovementAnimation {
//    pub current_frame: usize,
//    pub frames_accum: FramesAccumulator,
//    pub left_frames: Vec<Sprite>,
//    pub right_frames: Vec<Sprite>,
//    pub up_frames: Vec<Sprite>,
//    pub down_frames: Vec<Sprite>,
//}
//
//impl MovementAnimation {
//    /// Generates movement animation frames from a given spritesheet and initial sprite frame.
//    pub fn new(spritesheet: usize, initial_frame: Rect) -> Self {
//        MovementAnimation {
//            current_frame: 0,
//            frames_accum: FramesAccumulator::new(constants::SPRITE_ANIMATION_FPS),
//            left_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Left),
//            right_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Right),
//            up_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Up),
//            down_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Down),
//        }
//    }
//
//    /// Generates a series of animation frames from a spritesheet corresponding to a given
//    /// direction of travel.
//    fn animation_frames(spritesheet: usize, initial_frame: Rect, direction: Direction)
//                        -> Vec<Sprite> {
//        let (frame_width, frame_height) = initial_frame.size();
//    
//        let mut frames = Vec::new();
//
//        // Different columns in spritesheet represent different directions of travel.
//        let x_offset = Self::spritesheet_col(direction) * frame_width as i32;
//
//        for i in 0..constants::FRAMES_PER_ANIMATION_CYCLE {
//            // advance by one frame in the animation on each loop.
//            let y_offset = frame_height as i32 * Self::spritesheet_row(i);
//
//            let region = Rect::new(initial_frame.x() + x_offset,
//                                   initial_frame.y() + y_offset,
//                                   frame_width,
//                                   frame_height);
//
//            let flip_horizontal = false;
//
//            frames.push(Sprite { spritesheet, region, flip_horizontal });
//        }
//    
//        frames
//    }
//
//    /// Converts a given direction of movement to the row index in the spritesheet containing the
//    /// corresponding initial sprite frame.
//    fn spritesheet_col(direction: Direction) -> i32 {
//        match direction {
//            Direction::Left => 3,
//            Direction::Right => 1,
//            Direction::Up => 2,
//            Direction::Down => 0,
//        }
//    }
//
//    /// Converts the frame index in a movement animation to the corresponding row in the
//    /// spritesheet.
//    fn spritesheet_row(frame_index: i32) -> i32 {
//        match frame_index {
//            0 => 0,
//            1 => 1,
//            2 => 0, // return to standing before stepping with other foot.
//            3 => 2,
//            _ => 0,
//        }
//    }
//}
