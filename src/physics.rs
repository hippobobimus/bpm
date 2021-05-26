use specs::prelude::*;

use crate::{
    components::*,
    direction::Direction,
};

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    // possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        for (pos, vel) in (&mut data.0, &data.1).join() {
            let dir = get_active_direction(vel).unwrap();
            //if !vel.active_directions.contains(&dir) {
            //    continue;
            //};

            //match vel.direction {
            match dir {
                Direction::Left => {
                    pos.0 = pos.0.offset(-vel.speed, 0);
                },
                Direction::Right => {
                    pos.0 = pos.0.offset(vel.speed, 0);
                },
                Direction::Up => {
                    pos.0 = pos.0.offset(0, -vel.speed);
                },
                Direction::Down => {
                    pos.0 = pos.0.offset(0, vel.speed);
                },
            }
        }
    }

}
    fn get_active_direction(vel: &Velocity) -> Option<Direction> {
        Some(vel.direction)
    }
//    fn update_velocity(&mut self) {
//        self.update_direction();
//        self.speed = if self.is_direction_active(&self.direction) &&
//            !self.is_direction_active(&self.direction.get_opposite()) {
//            PLAYER_MOVEMENT_SPEED
//        } else {
//            0
//        }
//    }
//
//    /// Changes the current direction of the entity to the next active direction in the queue.
//    /// If there are no active directions in the queue then the direction is unchanged.
//    fn update_direction(&mut self) {
//        while let Some(dir) = self.directions_queue.front() {
//            if self.is_direction_active(dir) {
//                self.direction = *dir;
//                return;
//            } else {
//                self.directions_queue.pop_front();
//            }
//        }
//    }
//
//    fn update_position(&mut self) {
//        self.position = self.position.offset(self.speed * self.direction.dx(), self.speed * self.direction.dy());
//    }
