//! Engine crate for the Loid framework
//!
//! This crate provides the decision engine functionality for the Loid framework, including:
//! - Decision engine for determining the best path to a solution
//! - Algorithms for selecting the best neurons to activate based on goals and events

/// Version of the engine
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
