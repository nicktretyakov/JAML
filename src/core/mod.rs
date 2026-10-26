//! Core abstractions – Dependency Inversion Principle.
//! All higher-level modules depend on these traits, not concrete implementations.

pub mod error;
pub mod traits;

pub use error::*;
pub use traits::*;
