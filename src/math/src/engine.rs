use crate::error::Error;
use crate::token::Token;
use crate::Variable;
use crate::{Number, Result};
use std::collections::HashMap;

/// The Engine trait
/// This trait contains 2 parts. `evaluate` and `validate_tokens`
/// The `validate_tokens` ensures that the input is valid for the current `Engine`
///
pub trait Engine {
    /// Validate the given token list to ensure that it's executable
    /// This *only* do the syntatical check shouldn't perform any heavy operation
    fn validate_tokens(
        &mut self,
        token: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<()>;

    /// Evaluate the token list
    /// This function will always be call *after* `validate_tokens`, so it don't need to check for
    /// the correctness of Token list
    fn evaluate(
        &mut self,
        tokens: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<Number>;

    /// Call`validate_tokens` then `evaluate` it immediately
    fn execute(
        &mut self,
        token: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<Number> {
        self.validate_tokens(token, variables)?;
        self.evaluate(token, variables)
    }
}

#[derive(Default)]
/// An modification of the shunting yard algorithm for evaluate infix math notation that allows
/// functions/constants being used
pub struct ShuntingYardEngine;

impl Engine for ShuntingYardEngine {
    fn validate_tokens(
        &mut self,
        token: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<()> {
        todo!()
    }

    fn evaluate(
        &mut self,
        tokens: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<Number> {
        todo!()
    }
}
