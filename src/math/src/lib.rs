//! # The Math Library
//! Provide a `Number` type with mathematical operations
//! As well as engine to quickly evaluate infix math

#![deny(missing_docs)]

/// Error type
pub mod error;
/// Number type
pub mod number;

pub use number::Number;

/// Result type for this library
pub type Result<T> = std::result::Result<T, error::Error>;

/// Calculator struct
pub struct Calculator;

impl Calculator {
    /// Create a new instance
    pub fn new() -> Self {
        Self
    }
    /// Evaluate the infix math expression
    pub fn evaluate(&mut self, _s: &str) -> Result<Number> {
        todo!();
    }
}

/// High level function
/// for use in the long run, it is recommendded to create (and hold) an instance of `Calculator`
/// struct itself, as it may reserves the allocation spaces for future evaluation process
pub fn evaluate(_s: &str) -> Result<Number> {
    todo!()
}

#[cfg(test)]
mod math_tests;

