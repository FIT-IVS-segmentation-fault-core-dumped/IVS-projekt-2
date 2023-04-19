use crate::error::Error;
use crate::token::*;
use crate::Variable;
use crate::{Number, Result};
use std::collections::HashMap;

/// The Engine trait
/// This trait contains 2 parts. `evaluate` and `validate_tokens`
/// The `validate_tokens` ensures that the input is valid for the current `Engine`
///
pub trait Engine {
    /// Validate the given token list to ensure that it's executable
    /// This *only* do the semantic check shouldn't perform any heavy operation
    fn validate_tokens(
        &mut self,
        tokens: &[Token],
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
        tokens: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<Number> {
        self.validate_tokens(tokens, variables)?;
        self.evaluate(tokens, variables)
    }
}

enum ShuntingYardOperator {
    Operator(Operator),
    OpenParen,
    Comma,
    Variable(Variable),
}

#[derive(Default)]
/// An modification of the shunting yard algorithm for evaluate infix math notation that allows
/// functions/constants being used
pub struct ShuntingYardEngine {
    operators: Vec<ShuntingYardOperator>,
    operands: Vec<Number>,
}

impl Engine for ShuntingYardEngine {
    fn validate_tokens(
        &mut self,
        tokens: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<()> {
        let mut iter = tokens.iter().peekable();
        let mut arg_counts = Vec::new();

        while let Some(token) = iter.next() {
            match (token, iter.peek()) {
                (Token::Operator(_), None) => return Err(Error::MissingOperand),

                (
                    Token::Operator(_),
                    Some(Token::Operator(
                        Operator::Divide | Operator::Multiply | Operator::Power | Operator::Modulo,
                    )),
                ) => return Err(Error::MissingOperand),

                (Token::Number(_), Some(Token::Number(_))) => return Err(Error::MissingOperator),

                (Token::Id(id), next) => {
                    if next != Some(&&Token::Bracket(Bracket::ParenLeft)) {
                        return Err(Error::InvalidToken);
                    }

                    let Some(var) = variables.get(id) else {
                        return Err(Error::InvalidToken);
                    };

                    iter.next();

                    let argc = var.argc();
                    let next = iter.peek();

                    if argc == 0 {
                        if next != Some(&&Token::Bracket(Bracket::ParenRight)) {
                            return Err(Error::InvalidArguments);
                        }

                        iter.next();
                        continue;
                    }

                    arg_counts.push(argc);
                }

                (Token::Bracket(Bracket::ParenLeft), _) => {
                    arg_counts.push(1);
                }

                (Token::Bracket(Bracket::ParenRight), _) => {
                    let Some(argc) = arg_counts.pop() else {
                        return Err(Error::InvalidToken);
                    };

                    if argc != 1 {
                        return Err(Error::InvalidArguments);
                    }
                }

                (Token::Comma, _) => {
                    let Some(argc) = arg_counts.last_mut() else {
                        return Err(Error::InvalidArguments);
                    };

                    if *argc == 0 {
                        return Err(Error::InvalidArguments);
                    }

                    *argc -= 1;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn evaluate(
        &mut self,
        tokens: &[Token],
        variables: &HashMap<String, Variable>,
    ) -> Result<Number> {
        self.operators.clear();
        self.operands.clear();

        let mut iter = tokens.iter().peekable();
        let mut last_token = None;
        let mut negate_operand = false;

        while let Some(token) = iter.next() {
            match token {
                Token::Number(val) => {
                    let mut num = val.clone();
                    if negate_operand {
                        num = num.mul(-1)?;
                        negate_operand = false;
                    }

                    self.store_operand(num);
                }
                Token::Operator(op) => {
                    let mut op = *op;
                    // Combine all the `+` and `-` signs together
                    while let Some(Token::Operator(next_op)) = iter.peek() {
                        op = match (op, next_op) {
                            (Operator::Plus, Operator::Minus) => Operator::Minus,
                            (Operator::Minus, Operator::Plus) => Operator::Minus,
                            (Operator::Minus, Operator::Minus) => Operator::Plus,
                            (Operator::Plus, Operator::Plus) => Operator::Plus,
                            _ => break,
                        };

                        iter.next();
                    }

                    // Handle the `+` `-` sign of a number
                    if matches!(
                        (last_token, op, iter.peek()),
                        (
                            None | Some(
                                &Token::Comma
                                    | &Token::Bracket(Bracket::ParenLeft)
                                    | &Token::Operator(
                                        Operator::Multiply
                                            | Operator::Divide
                                            | Operator::Power
                                            | Operator::Modulo
                                    )
                            ),
                            Operator::Plus | Operator::Minus,
                            Some(Token::Number(_)),
                        )
                    ) {
                        negate_operand = op == Operator::Minus;
                        continue;
                    }

                    self.operator_handle(op)?;
                }
                Token::FactorialSign => {
                    let num = self.operands.pop().unwrap().factorial()?;
                    self.store_operand(num);
                }

                Token::Bracket(Bracket::ParenLeft) => {
                    self.operators.push(ShuntingYardOperator::OpenParen);
                }
                Token::Bracket(Bracket::ParenRight) => self.closing_bracket_handle()?,
                Token::Bracket(Bracket::VerticalLine) => todo!(),
                Token::Id(id) => {
                    let var = variables.get(id).cloned().unwrap();
                    self.operators.push(ShuntingYardOperator::Variable(var));
                }
                Token::Comma => {
                    if let Some(val) = self.finalize()? {
                        self.store_operand(val);
                    }
                    self.operators.push(ShuntingYardOperator::Comma);
                }
            }

            // Handle the hidden multiply sign in algebraic notation
            if let Some(next_token) = iter.peek() {
                if token != *next_token {
                    let left = matches!(
                        token,
                        Token::Number(_)
                            | Token::FactorialSign
                            | Token::Bracket(Bracket::ParenRight)
                    );

                    let right = matches!(
                        next_token,
                        Token::Number(_) | Token::Id(_) | Token::Bracket(Bracket::ParenLeft)
                    );

                    if left && right {
                        self.operator_handle(Operator::Multiply)?;
                    }
                }
            }

            last_token.replace(token);
        }

        self.finalize()?
            .or_else(|| self.operands.pop())
            .ok_or(Error::MissingOperand)
    }
}

fn operator_precedence(op: Operator) -> u8 {
    match op {
        Operator::Plus | Operator::Minus => 0,
        Operator::Multiply | Operator::Divide => 1,
        Operator::Power | Operator::Modulo => 2,
    }
}

fn evaluate_expr(lhs: Number, rhs: Number, op: Operator) -> Result<Number> {
    match op {
        Operator::Plus => lhs.add(rhs),
        Operator::Minus => lhs.sub(rhs),
        Operator::Multiply => lhs.mul(rhs),
        Operator::Divide => lhs.div(rhs),
        Operator::Power => lhs.power(rhs),
        Operator::Modulo => lhs.modulo(rhs),
    }
}

impl ShuntingYardEngine {
    fn store_operand(&mut self, val: Number) {
        self.operands.push(val);
    }

    fn operator_handle(&mut self, op: Operator) -> Result<()> {
        let current_precedence = operator_precedence(op);

        while let Some(ShuntingYardOperator::Operator(last_op)) = self.operators.last() {
            let last_precedence = operator_precedence(*last_op);
            if current_precedence > last_precedence {
                break;
            }

            let rhs = self.operands.pop().unwrap();
            let lhs = self
                .operands
                .pop()
                .or_else(|| matches!(last_op, Operator::Plus | Operator::Minus).then(Number::zero))
                .ok_or(Error::MissingOperand)?;
            self.store_operand(evaluate_expr(lhs, rhs, *last_op)?);
            self.operators.pop();
        }

        self.operators.push(ShuntingYardOperator::Operator(op));
        Ok(())
    }

    fn closing_bracket_handle(&mut self) -> Result<()> {
        if let Some(num) = self.finalize()? {
            self.store_operand(num);
        }

        if let Some(ShuntingYardOperator::Variable(var)) = self.operators.last() {
            let argc = var.argc();
            let mut argv = Vec::with_capacity(argc as usize);

            for _ in 0..argc {
                argv.insert(0, self.operands.pop().unwrap());
            }

            let val = var.calc(&argv)?;
            self.operators.pop();
            self.store_operand(val);
        }

        Ok(())
    }

    fn finalize(&mut self) -> Result<Option<Number>> {
        let mut res = None;

        while let Some(operator) = self.operators.pop() {
            let ShuntingYardOperator::Operator(op) = operator else {
                break;
            };

            let rhs = res.clone().or_else(|| self.operands.pop()).unwrap();
            let lhs = self
                .operands
                .pop()
                .or_else(|| matches!(op, Operator::Plus | Operator::Minus).then(Number::zero))
                .ok_or(Error::MissingOperand)?;
            res.replace(evaluate_expr(lhs, rhs, op)?);
        }

        Ok(res)
    }
}
