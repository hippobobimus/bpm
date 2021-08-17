mod collision_detection;
mod contact_generation;

pub use collision_detection::{
    get_system_set,
    initialize,
};
pub use contact_generation::contact::Contact;
pub use contact_generation::dispatcher::{
    generate_primative_contacts,
    generate_boundary_contacts,
};
