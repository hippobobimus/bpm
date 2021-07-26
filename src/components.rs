//use bevy::{
//    prelude::*,
//    math::{DMat3, DMat4, DQuat, DVec3},
//};
//
//use crate::{
//    constants,
//    //shapes::{Circle, Line, Plane, Polygon},
//};

// Marker components

pub struct KeyboardControlled;
pub struct Player;

// Physics

//#[derive(Bundle, Default)]
//pub struct PhysicsBundle {
//    pub mass: Mass,
//    pub velocity: Velocity,
//    pub force: Force,
//    pub drag: Drag,
//    pub gravity: Gravity,
//    pub thrust: Thrust,
//}
//
//pub struct InertiaTensor {
//    tensor: DMat3,
//    inverse: DMat3,
//}
//
//impl InertiaTensor {
//    pub fn new(inertia_tensor: DMat3) -> Self {
//        Self {
//            tensor: inertia_tensor,
//            inverse: inertia_tensor.inverse(),
//        }
//    }
//
//    pub fn tensor(&self) -> &DMat3 {
//        &self.tensor
//    }
//
//    pub fn inverse(&self) -> &DMat3 {
//        &self.inverse
//    }
//}
//
//pub struct Mass {
//    value: f64,
//    inverse: f64,
//}
//
//impl Default for Mass {
//    fn default() -> Self {
//        Self {
//            value: constants::DEFAULT_MASS,
//            inverse: constants::DEFAULT_INVERSE_MASS,
//        }
//    }
//}
//
//impl Mass {
//    pub fn new(value: f64) -> Self {
//        Self {
//            value,
//            inverse: 1.0 / value,
//        }
//    }
//
//    pub fn from_inverse(inverse: f64) -> Self {
//        Self {
//            value: 1.0 / inverse,
//            inverse,
//        }
//    }
//
//    pub fn inverse(&self) -> f64 {
//        self.inverse
//    }
//
//    pub fn is_infinite(&self) -> bool {
//        self.inverse == 0.0
//    }
//
//    pub fn is_normal(&self) -> bool {
//        self.value.is_normal()
//    }
//
//    pub fn value(&self) -> f64 {
//        self.value
//    }
//}
//
//#[derive(Default)]
//pub struct AngularVelocity {
//    vector: DVec3,
//}
//
//impl AngularVelocity {
//    pub fn new(vector: DVec3) -> Self {
//        Self { vector }
//    }
//
//    pub fn vector(&self) -> &DVec3 {
//        &self.vector
//    }
//}
//
//#[derive(Default)]
//pub struct Velocity {
//    vector: DVec3,
//}
//
//impl Velocity {
//    pub fn new(vector: DVec3) -> Self {
//        Self { vector }
//    }
//
//    pub fn scale(&mut self, s: f64) {
//        self.vector *= s;
//    }
//
//    pub fn add(&mut self, v: DVec3) {
//        self.vector += v;
//    }
//
//    pub fn vector(&self) -> &DVec3 {
//        &self.vector
//    }
//
//    pub fn zero(&mut self) {
//        self.vector = DVec3::ZERO;
//    }
//}
//
//// -- Force Accumulator
//
//#[derive(Debug, Default)]
//pub struct Force {
//    total: DVec3,
//}
//
//impl Force {
//    pub fn new() -> Self {
//        Self::default()
//    }
//
//    pub fn add(&mut self, f: DVec3) {
//        self.total += f;
//    }
//
//    pub fn reset(&mut self) {
//        self.total = DVec3::ZERO;
//    }
//
//    pub fn vector(&self) -> &DVec3 {
//        &self.total
//    }
//}
//
//// -- Force generators
//
//#[derive(Debug)]
//pub struct Drag {
//    k1: f64,
//    k2: f64,
//}
//
//impl Default for Drag {
//    fn default() -> Self {
//        Self {
//            k1: constants::DEFAULT_K1,
//            k2: constants::DEFAULT_K2,
//        }
//    }
//}
//
//impl Drag {
//    pub fn new(k1: f64, k2: f64) -> Self {
//        Self { k1, k2 }
//    }
//
//    pub fn force(&self, velocity: DVec3) -> DVec3 {
//        let v_mag = velocity.length();
//        let coeff = self.k1 * v_mag + self.k2 * v_mag.powi(2);
//
//        -coeff * velocity.normalize_or_zero()
//    }
//}
//
//#[derive(Debug)]
//pub struct Gravity {
//    g: DVec3,
//}
//
//impl Default for Gravity {
//    fn default() -> Self {
//        Self {
//            g: *constants::DEFAULT_GRAVITY,
//        }
//    }
//}
//
//impl Gravity {
//    pub fn new(g: DVec3) -> Self {
//        Self { g }
//    }
//
//    pub fn force(&self, m: f64) -> DVec3 {
//        m * self.g
//    }
//}
//
//pub struct Thrust {
//    force: DVec3,
//    magnitude: f64,
//}
//
//impl Default for Thrust {
//    fn default() -> Self {
//        Self {
//            force: Default::default(),
//            magnitude: constants::DEFAULT_THRUST,
//        }
//    }
//}
//
//impl Thrust {
//    pub fn new(magnitude: f64) -> Self {
//        Self {
//            force: Default::default(),
//            magnitude,
//        }
//    }
//
//    pub fn disengage(&mut self, dir: &DVec3) {
//        self.force -= self.magnitude * dir.normalize();
//    }
//
//    pub fn engage(&mut self, dir: &DVec3) {
//        self.force += self.magnitude * dir.normalize();
//    }
//
//    pub fn force(&self) -> &DVec3 {
//        &self.force
//    }
//}

// TODO port old code
// Physics
//
//#[derive(Component, Debug, Default)]
//#[storage(VecStorage)]
//pub struct Position {
//    pub vector: Vector2<f64>,
//}
//
//impl Position {
//    pub fn new(vector: Vector2<f64>) -> Self {
//        Self { vector }
//    }
//
//    pub fn transform(&mut self, t: &Vector2<f64>) {
//        self.vector += t;
//    }
//
//    pub fn vector(&self) -> &Vector2<f64> {
//        &self.vector
//    }
//}
//
//// --Collisions
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct Collision {
//    pub ent_a: Entity,
//    pub ent_b: Entity,
//    pub normal: Vector2<f64>,
//}
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct CircleCollider {
//    circle: Circle,
//}
//
//impl CircleCollider {
//    pub fn new(radius: f64) -> Self {
//        Self { circle: Circle::new(radius) }
//    }
//
//    pub fn circle(&self) -> &Circle {
//        &self.circle
//    }
//}
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct PolygonCollider {
//    polygon: Polygon,
//}
//
//impl PolygonCollider {
//    pub fn new(vertices: &Vec<Vector2<f64>>) -> Self {
//        Self {
//            polygon: Polygon::new(vertices),
//        }
//    }
//
//    pub fn polygon(&self) -> &Polygon {
//        &self.polygon
//    }
//}
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct BoundaryCollider {
//    pub plane: Plane,
//}
//
//impl BoundaryCollider {
//    pub fn new(normal: Vector2<f64>) -> Self {
//        Self {
//            plane: Plane::new(normal),
//        }
//    }
//
//    pub fn normal(&self) -> &Vector2<f64> {
//        self.plane.normal()
//    }
//
//    pub fn boundary(&self) -> &Plane {
//        &self.plane
//    }
//}
//
//#[derive(Component, Debug, Default)]
//#[storage(VecStorage)]
//pub struct Forces {
//    pub propulsion: Vector2<f64>,
//    pub drag: Vector2<f64>,
//}
//
//impl Forces {
//    pub fn new(propulsion: Vector2<f64>, drag: Vector2<f64>) -> Self {
//        Self { propulsion, drag }
//    }
//}
//
//// Rendering
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct RenderableCircle {
//    circle: Circle,
//}
//
//impl RenderableCircle {
//    pub fn new(radius: f64) -> Self {
//        Self {
//            circle: Circle::new(radius),
//        }
//    }
//
//    pub fn radius(&self) -> i16 {
//        self.circle.radius() as i16
//    }
//}
//
//// TODO
//pub struct RenderableLine {
//    line: Line,
//}
//
//impl RenderableLine {
//    pub fn new(start: Vector2<f64>, end: Vector2<f64>) -> Self {
//        Self {
//            line: Line::new(start, end),
//        }
//    }
//
//    pub fn start(&self, position: Vector2<f64>) -> Vector2<i16> {
//        let temp = self.line.start() + position;
//        vector![temp.x as i16, temp.y as i16]
//    }
//}
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct RenderablePolygon {
//    polygon: Polygon,
//}
//
//impl RenderablePolygon {
//    /// Creates a new polygon with the given vertices. The vertices must be given as vectors
//    /// relative to the polygon's centre and are assumed to be ordered such that iterating over
//    /// them is equivalent to traversing the perimeter of the polygon.
//    pub fn new(vertices: &Vec<Vector2<f64>>) -> Self {
//        Self {
//            polygon: Polygon::new(vertices),
//        }
//    }
//
//    /// Returns a Vec of integer x-coords of the polygon's vertices.
//    pub fn vx(&self, position: Vector2<f64>) -> Vec<i16> {
//        self.polygon.vertices().iter().map(|v| (v.x + position.x) as i16).collect()
//    }
//
//    /// Returns a Vec of integer y-coords of the polygon's vertices.
//    pub fn vy(&self, position: Vector2<f64>) -> Vec<i16> {
//        self.polygon.vertices().iter().map(|v| (v.y + position.y) as i16).collect()
//    }
//}
//
//#[derive(Component, Debug)]
//#[storage(VecStorage)]
//pub struct RenderColour {
//    sdl_colour: Color,
//}
//
//impl RenderColour {
//    pub fn new(r: u8, g: u8, b: u8) -> Self {
//        Self {
//            sdl_colour: Color::RGB(r, g, b),
//        }
//    }
//
//    pub fn sdl_colour(&self) -> Color {
//        self.sdl_colour
//    }
//
//    pub fn change_colour(&mut self, r: u8, g: u8, b: u8) {
//        self.sdl_colour = Color::RGB(r, g, b);
//    }
//}
//
////#[derive(Clone, Component, Debug)]
////#[storage(VecStorage)]
////pub struct Sprite {
////    pub spritesheet: usize,
////    pub region: Rect,
////    pub flip_horizontal: bool,
////}
////
////#[derive(Component, Debug)]
////#[storage(VecStorage)]
////pub struct MovementAnimation {
////    pub current_frame: usize,
////    pub frames_accum: FramesAccumulator,
////    pub left_frames: Vec<Sprite>,
////    pub right_frames: Vec<Sprite>,
////    pub up_frames: Vec<Sprite>,
////    pub down_frames: Vec<Sprite>,
////}
////
////impl MovementAnimation {
////    /// Generates movement animation frames from a given spritesheet and initial sprite frame.
////    pub fn new(spritesheet: usize, initial_frame: Rect) -> Self {
////        MovementAnimation {
////            current_frame: 0,
////            frames_accum: FramesAccumulator::new(constants::SPRITE_ANIMATION_FPS),
////            left_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Left),
////            right_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Right),
////            up_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Up),
////            down_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Down),
////        }
////    }
////
////    /// Generates a series of animation frames from a spritesheet corresponding to a given
////    /// direction of travel.
////    fn animation_frames(spritesheet: usize, initial_frame: Rect, direction: Direction)
////                        -> Vec<Sprite> {
////        let (frame_width, frame_height) = initial_frame.size();
////
////        let mut frames = Vec::new();
////
////        // Different columns in spritesheet represent different directions of travel.
////        let x_offset = Self::spritesheet_col(direction) * frame_width as i32;
////
////        for i in 0..constants::FRAMES_PER_ANIMATION_CYCLE {
////            // advance by one frame in the animation on each loop.
////            let y_offset = frame_height as i32 * Self::spritesheet_row(i);
////
////            let region = Rect::new(initial_frame.x() + x_offset,
////                                   initial_frame.y() + y_offset,
////                                   frame_width,
////                                   frame_height);
////
////            let flip_horizontal = false;
////
////            frames.push(Sprite { spritesheet, region, flip_horizontal });
////        }
////
////        frames
////    }
////
////    /// Converts a given direction of movement to the row index in the spritesheet containing the
////    /// corresponding initial sprite frame.
////    fn spritesheet_col(direction: Direction) -> i32 {
////        match direction {
////            Direction::Left => 3,
////            Direction::Right => 1,
////            Direction::Up => 2,
////            Direction::Down => 0,
////        }
////    }
////
////    /// Converts the frame index in a movement animation to the corresponding row in the
////    /// spritesheet.
////    fn spritesheet_row(frame_index: i32) -> i32 {
////        match frame_index {
////            0 => 0,
////            1 => 1,
////            2 => 0, // return to standing before stepping with other foot.
////            3 => 2,
////            _ => 0,
////        }
////    }
////}
