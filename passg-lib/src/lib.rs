//! This library provides a convenient way to generate pseudorandom passwords
//! according to some given constraints.
//!
//! # Example
//! ```
//! use passg::prelude::*;
//! let generator = GeneratorBuilder::default().build();
//! let password  = generator.generate();
//! ```

pub mod charsets;
pub mod errors;
pub mod generator;

/// A prelude you can use to easily get started
pub mod prelude {
    pub use super::charsets::{Alpha, CollatingSeq, Digit, Special};
    pub use super::errors::Error;
    pub use super::generator::{Generator, GeneratorBuilder};
}
