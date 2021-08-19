mod contact_generation;
mod processor;

pub use processor::{
    get_system_set,
    initialize,
};
pub use contact_generation::dispatcher::{
    generate_primative_contacts,
    generate_boundary_contacts,
};
