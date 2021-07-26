use bevy::math::DMat3;

pub struct InertiaTensor {
    tensor: DMat3,
    inverse: DMat3,
}

impl InertiaTensor {
    pub fn new(inertia_tensor: DMat3) -> Self {
        Self {
            tensor: inertia_tensor,
            inverse: inertia_tensor.inverse(),
        }
    }

    pub fn tensor(&self) -> &DMat3 {
        &self.tensor
    }

    pub fn inverse(&self) -> &DMat3 {
        &self.inverse
    }
}
