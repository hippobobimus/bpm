use bevy::prelude::*;
//use rand::prelude::*;

use crate::{
    components::*,
    //constants,
};

// Plugin

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system());
    }
}

// Systems

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 1.0;

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius, subdivisions: 10 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, radius, 0.0),
        ..Default::default()
    })
    .insert(KeyboardControlled)
    .insert(Player)
    .insert_bundle(PhysicsBundle {
        mass: Mass::new(10.0),
        ..Default::default()
    });
}

// TODO port old code
//pub fn setup_initial_entities(world: &mut World) {
//    let circle_qty = 0;
//    let polygon_qty = 0;
//
//    let mut spawner = Spawner::new(world);
//
//    // spawn player.
//    spawner.spawn_player(0.0, 0.0, 10.0, 10.0);
//
//    // spawn world boundaries.
//    spawner.spawn_boundaries();
//
//    // spawn random circles.
//    let mut rng = thread_rng();
//    for _ in 0..circle_qty {
//        let radius = rng.gen_range(1.0..20.0);
//        let x = rng.gen_range(constants::FMIN_X + radius..constants::FMAX_X - radius);
//        let y = rng.gen_range(constants::FMIN_Y + radius..constants::FMAX_Y - radius);
//        let mass = radius * radius * radius * 0.01;
//        spawner.spawn_circle(x, y, radius, mass);
//    }
//
//    // spawn polygons
//    // TODO currently just a generic polygon.
//    let vertices = vec![
//        vector![-20.0, 10.0],
//        vector![0.0, 20.0],
//        vector![20.0, 0.0],
//        vector![10.0, -20.0],
//        vector![-10.0, -10.0]
//    ];
//
//    for _ in 0..polygon_qty {
//        spawner.spawn_polygon(50.0, 50.0, 100.0, vertices.clone());
//    }
//}
//
//struct Spawner<'a> {
//    world: &'a mut World,
//}
//
//impl<'a> Spawner<'a> {
//    pub fn new(world: &'a mut World) -> Self {
//        Self { world }
//    }
//
//    fn spawn_circle(&mut self, x_pos: f64, y_pos: f64, radius: f64, mass: f64) {
//        let mut rng = thread_rng();
//        self.world.create_entity()
//                  .with(Position { vector: vector![x_pos, y_pos] })
//                  // physics
//                  .with(Forces::default())
//                  .with(Mass { value: mass, inverse: 1.0 / mass })
//                  .with(Velocity {
//                      vector: vector![rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)] })
//                  .with(CircleCollider::new(radius))
//                  // rendering
//                  .with(RenderableCircle::new(radius))
//                  .with(RenderColour::new(255, 210, 0))
//                  .build();
//    }
//
//    fn spawn_polygon(&mut self, x_pos: f64, y_pos: f64, mass: f64, vertices: Vec<Vector2<f64>>) {
//        let mut rng = thread_rng();
//        self.world.create_entity()
//                  .with(Position { vector: vector![x_pos, y_pos] })
//                  // physics
//                  .with(Forces::default())
//                  .with(Mass { value: mass, inverse: 1.0 / mass })
//                  .with(Velocity { vector: vector![rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)] })
//                  .with(PolygonCollider::new(&vertices))
//                  // rendering
//                  .with(RenderablePolygon::new(&vertices))
//                  .with(RenderColour::new(255, 120, 0))
//                  .build();
//    }
//
//
//    fn spawn_player(&mut self, x_pos: f64, y_pos: f64, radius: f64, mass: f64) {
//        self.world.create_entity()
//                  .with(Player)
//                  .with(KeyboardControlled)
//                  .with(Position::new(vector![x_pos, y_pos]))
//                  // physics
//                  .with(Force::default())
//                  .with(Drag::default())
//                  .with(Gravity::default())
//                  .with(Thrust::default())
//                  //.with(Forces::default())
//                  .with(Mass::new(mass))
//                  .with(Velocity::default())
//                  //.with(CircleCollider::new(radius))
//                  // rendering
//                  .with(RenderableCircle::new(radius))
//                  .with(RenderColour::new(0, 255, 0))
//                  .build();
//    }
//
//    fn spawn_boundaries(&mut self) {
//        let planes = vec![
//            // Top
//            (vector![0.0, 1.0], vector![constants::FMIN_X, constants::FMIN_Y]),
//            // Bottom
//            (vector![0.0, -1.0], vector![constants::FMAX_X, constants::FMAX_Y]),
//            // Left
//            (vector![1.0, 0.0], vector![constants::FMIN_X, constants::FMIN_Y]),
//            // Right
//            (vector![-1.0, 0.0], vector![constants::FMAX_X, constants::FMAX_Y]),
//        ];
//
//        for (n, pos) in planes {
//            self.world.create_entity()
//                      .with(Position { vector: pos })
//                      // physics
//                      .with(Mass { value: f64::INFINITY, inverse: 0.0 })
//                      .with(Velocity { vector: vector![0.0, 0.0] })
//                      .with(BoundaryCollider::new(n))
//                      .build();
//        }
//    }
//}
