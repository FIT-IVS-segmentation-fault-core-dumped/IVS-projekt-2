//! Lexical analyzer
//!
//! ```
//! # use math::token::{Token, Bracket, Operator, Scanner};
//! # use math::Number;
//! # fn main() -> math::Result<()> {
//! let s = "pi()1.1 + 3 - .5e()root(sqrt(8), 5!)";
//! let mut scanner = Scanner::new(s);
//!
//! assert_eq!(scanner.next_token()?, Some(Token::Id(String::from("pi"))));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenLeft)));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenRight)));
//! assert_eq!(scanner.next_token()?, Some(Token::Number(Number::new(11, 10)?)));
//! assert_eq!(scanner.next_token()?, Some(Token::Operator(Operator::Plus)));
//! assert_eq!(scanner.next_token()?, Some(Token::Number(Number::from(3))));
//! assert_eq!(scanner.next_token()?, Some(Token::Operator(Operator::Minus)));
//! assert_eq!(scanner.next_token()?, Some(Token::Number(Number::new(1, 2)?)));
//! assert_eq!(scanner.next_token()?, Some(Token::Id(String::from("e"))));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenLeft)));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenRight)));
//! assert_eq!(scanner.next_token()?, Some(Token::Id(String::from("root"))));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenLeft)));
//! assert_eq!(scanner.next_token()?, Some(Token::Id(String::from("sqrt"))));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenLeft)));
//! assert_eq!(scanner.next_token()?, Some(Token::Number(Number::from(8))));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenRight)));
//! assert_eq!(scanner.next_token()?, Some(Token::Comma));
//! assert_eq!(scanner.next_token()?, Some(Token::Number(Number::from(5))));
//! assert_eq!(scanner.next_token()?, Some(Token::FactorialSign));
//! assert_eq!(scanner.next_token()?, Some(Token::Bracket(Bracket::ParenRight)));
//! assert_eq!(scanner.next_token()?, None);
//! # Ok(())
//! # }
//! ```
//!

use crate::error::Error;
use crate::number::Number;
use crate::Result;
use num::BigUint;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Representation of a bracket
pub enum Bracket {
    /// (
    ParenLeft,
    /// )
    ParenRight,
    /// |
    VerticalLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Representation of a mathematical operator
pub enum Operator {
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Multiply,
    /// /
    Divide,
    /// ^
    Power,
    /// Modulo,
    Modulo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Representation of a token
pub enum Token {
    /// Number
    Number(Number),
    /// Bracket `(`, `)`, `|`
    Bracket(Bracket),
    /// !
    FactorialSign,
    /// ,
    Comma,
    /// Operator `+`, `-`, `*`, `/`, `^`
    Operator(Operator),
    /// Idenfifier
    Id(String),
}

#[derive(Debug, Default, PartialEq)]
enum State {
    #[default]
    Start,
    FactorialSign,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    LeftPar,
    RightPar,
    VerticalLine,
    Comma,
    Identifier(String),
    NumberStart,
    Number {
        radix: u32,
        num: BigUint,
    },
    FractionStart,
    Fraction {
        radix: u32,
        num: BigUint,
        fract_cnt: u32,
    },
}

impl State {
    fn into_token(self) -> Option<Token> {
        let token = match self {
            Self::Add => Token::Operator(Operator::Plus),
            Self::Sub => Token::Operator(Operator::Minus),
            Self::Mul => Token::Operator(Operator::Multiply),
            Self::Div => Token::Operator(Operator::Divide),
            Self::Pow => Token::Operator(Operator::Power),
            Self::Comma => Token::Comma,
            Self::FactorialSign => Token::FactorialSign,
            Self::LeftPar => Token::Bracket(Bracket::ParenLeft),
            Self::RightPar => Token::Bracket(Bracket::ParenRight),
            Self::VerticalLine => Token::Bracket(Bracket::VerticalLine),
            Self::Identifier(s) => match s.as_str() {
                "mod" => Token::Operator(Operator::Modulo),
                _ => Token::Id(s),
            },
            Self::NumberStart => Token::Number(Number::zero()),
            Self::Number { num, .. } => Token::Number(Number::from(num)),
            Self::Fraction {
                num,
                fract_cnt,
                radix,
            } => {
                let val = Number::new(num, BigUint::from(radix).pow(fract_cnt)).unwrap_or_default();
                Token::Number(val)
            }

            _ => return None,
        };

        Some(token)
    }

    fn next_state(&mut self, ch: char) -> Result<Option<State>> {
        let next = match self {
            Self::FactorialSign
            | Self::LeftPar
            | Self::RightPar
            | Self::VerticalLine
            | Self::Comma
            | Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Pow => Some(Self::Start),
            Self::Start => {
                let next_state = match ch {
                    ')' => State::RightPar,
                    '(' => State::LeftPar,
                    '|' => State::VerticalLine,
                    '+' => State::Add,
                    '-' => State::Sub,
                    '*' => State::Mul,
                    '/' => State::Div,
                    '^' => State::Pow,
                    '!' => State::FactorialSign,
                    ',' => State::Comma,
                    '0' => State::NumberStart,
                    ' ' => return Ok(None),
                    '1'..='9' => State::Number {
                        radix: 10,
                        num: BigUint::from(ch.to_digit(10).unwrap()),
                    },
                    '.' => State::FractionStart,
                    'a'..='z' | 'A'..='Z' | '_' => State::Identifier(ch.to_string()),

                    _ => return Err(Error::UnsupportedToken(0)),
                };

                Some(next_state)
            }

            Self::Identifier(ref mut s) => 'id: {
                if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                    break 'id Some(State::Start);
                }

                s.push(ch);
                None
            }
            Self::NumberStart => match ch {
                '.' => Some(Self::Fraction {
                    radix: 10,
                    num: Default::default(),
                    fract_cnt: Default::default(),
                }),
                'b' => Some(Self::Number {
                    radix: 2,
                    num: Default::default(),
                }),
                'o' => Some(Self::Number {
                    radix: 8,
                    num: Default::default(),
                }),
                'x' => Some(Self::Number {
                    radix: 16,
                    num: Default::default(),
                }),
                '0'..='9' => Some(Self::Number {
                    radix: 10,
                    num: BigUint::from(ch.to_digit(10).unwrap()),
                }),
                _ => Some(Self::Start),
            },

            Self::Number { radix, ref mut num } => 'number: {
                if ch == '.' {
                    break 'number Some(Self::Fraction {
                        radix: *radix,
                        num: mem::take(num),
                        fract_cnt: num::zero(),
                    });
                }

                let Some(val) = ch.to_digit(*radix) else {
                    break 'number Some(Self::Start);
                };

                *num *= *radix;
                *num += val;

                None
            }

            Self::FractionStart => {
                let Some(num) = ch.to_digit(10) else {
                    return Err(Error::UnsupportedToken(0));
                };

                Some(Self::Fraction {
                    radix: 10,
                    num: BigUint::from(num),
                    fract_cnt: 1,
                })
            }

            Self::Fraction {
                radix,
                ref mut num,
                ref mut fract_cnt,
                ..
            } => 'fraction: {
                let Some(val) = ch.to_digit(*radix) else {
                    break 'fraction Some(Self::Start);
                };

                *fract_cnt += 1;

                *num *= *radix;
                *num += val;

                None
            }
        };

        Ok(next)
    }
}

enum StepState {
    Token(Token),
    Inprogress,
    End,
}

/// Scan tokens from string
pub struct Scanner<'a> {
    iter: std::str::Chars<'a>,
    state: State,
    cnt: usize,
    buf: Option<char>,
}

impl<'a> Scanner<'a> {
    /// Create a new scanner to scan the given `s`
    pub fn new(s: &'a str) -> Self {
        Self {
            iter: s.chars(),
            state: State::Start,
            cnt: 0,
            buf: None,
        }
    }

    /// Scan for the next token
    pub fn next_token(&mut self) -> Result<Option<Token>> {
        loop {
            match self.step()? {
                StepState::Inprogress => continue,
                StepState::Token(token) => break Ok(Some(token)),
                StepState::End => break Ok(None),
            }
        }
    }

    fn step(&mut self) -> Result<StepState> {
        self.cnt += 1;

        let Some(ch) = self.buf.take().or_else(|| self.iter.next()) else {
            let state = mem::take(&mut self.state);

            if state == State::Start {
                return Ok(StepState::End);
            }

            return state
                .into_token()
                .ok_or(Error::UnsupportedToken(self.cnt))
                .map(StepState::Token)
        };

        let next_state = self
            .state
            .next_state(ch)
            .map_err(|_| Error::UnsupportedToken(self.cnt))?;

        let Some(mut state) = next_state else {
            return Ok(StepState::Inprogress);
        };

        mem::swap(&mut state, &mut self.state);

        if let State::Start = self.state {
            if state == State::Start {
                return Ok(StepState::Inprogress);
            }

            self.buf.replace(ch);

            return state
                .into_token()
                .ok_or(Error::UnsupportedToken(self.cnt))
                .map(StepState::Token);
        }

        Ok(StepState::Inprogress)
    }
}
