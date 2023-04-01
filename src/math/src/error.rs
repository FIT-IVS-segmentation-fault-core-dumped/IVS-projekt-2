/// Error
///
///
use thiserror::Error;

#[derive(Debug, Error)]
/// Error type
pub enum Error {
    #[error("Division zero")]
    /// Division zero
    DivisionZero,

    #[error("Factorial of negative number")]
    /// Factorial of negative number
    FactorialNegative,
}
