/// Error
///
///
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
/// Error type
pub enum Error {
    #[error("Division zero")]
    /// Division zero
    DivisionZero,

    #[error("Factorial of negative number")]
    /// Factorial of negative number
    FactorialNegative,

    #[error("Log of a negative number")]
    ///
    LogNegativeBase,
}
