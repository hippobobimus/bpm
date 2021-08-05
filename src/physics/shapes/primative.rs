use downcast_rs::{
    Downcast,
    impl_downcast,
};

pub trait CollisionPrimative: std::fmt::Debug + Downcast + Send + Sync {}
impl_downcast!(CollisionPrimative);
