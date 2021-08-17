pub mod collision_detection;
pub mod contact_generation;

pub use collision_detection::*;
pub use contact_generation::dispatcher::{
    generate_primative_contacts,
    generate_boundary_contacts,
};
