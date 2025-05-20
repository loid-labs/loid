//! Persistence crate for the Loid framework
//!
//! This crate provides persistence functionality for the Loid framework, including:
//! - Models for neurons, events, goals, and sources
//! - Repository traits for storing and retrieving neurons and goals
//! - Vector database integration for efficient neuron storage
//! - Event queue for storing and retrieving events

/// Version of the persistence crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
