use bevy::math::DVec3;
use downcast_rs::{
    Downcast,
    impl_downcast,
};

use std::any::Any;

pub trait Collider: std::fmt::Debug + Downcast + Send + Sync {
//    fn is_shape<T: Collider>(&self) -> bool {
//        //self.is::<T>()
//    }
}
impl_downcast!(Collider);

//impl dyn Collider {
//    fn is_shape<T: Collider>(&self) -> bool {
//        self.is::<T>()
//    }
//}
