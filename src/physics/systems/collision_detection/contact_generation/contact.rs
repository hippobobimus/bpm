use bevy::math::DVec3;

#[derive(Debug)]
pub struct Contact {
    pub normal: DVec3,
    pub penetration: f64,
    pub point: DVec3,
}
