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

    #[error("Logarithm to a negative base")]
    /// Logarithm to a negative base
    LogNegativeBase,

    #[error("Zero nth root")]
    /// Zero nth root
    ZeroNthRoot,

    #[error("Negative root")]
    /// Negative root
    NegativeRoot,

    #[error("Number is outside of range")]
    /// Number is outside of range
    OutOfRange,
}
