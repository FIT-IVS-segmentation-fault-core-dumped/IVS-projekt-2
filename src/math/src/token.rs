use crate::error::Error;
use crate::number::Number;
use crate::Result;
use num::BigInt;
use std::mem;

#[derive(Debug, Clone, Copy)]
/// Representation of a bracket
pub enum Bracket {
    /// (
    ParenLeft,
    /// )
    ParenRight,
    /// |
    VerticalLine,
}

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone)]
/// Representation of a token
pub enum Token {
    /// Number
    Number(Number),
    /// Bracket `(`, `)`, `|`
    Bracket(Bracket),
    /// !
    FactorialSign,
    /// Operator `+`, `-`, `*`, `/`, `^`
    Operator(Operator),
    /// Idenfifier
    Id(String),
}

#[derive(Default)]
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
    Identifier(String),
    NumberStart,
    Number {
        radix: u32,
        num: BigInt,
    },
    FractionStart,
    Fraction {
        radix: u32,
        num: BigInt,
        fract: BigInt,
    },
}

impl State {
    fn to_token(self) -> Option<Token> {
        let token = match self {
            Self::Add => Token::Operator(Operator::Plus),
            Self::Sub => Token::Operator(Operator::Minus),
            Self::Mul => Token::Operator(Operator::Multiply),
            Self::Div => Token::Operator(Operator::Divide),
            Self::Pow => Token::Operator(Operator::Power),
            Self::FactorialSign => Token::FactorialSign,
            Self::LeftPar => Token::Bracket(Bracket::ParenLeft),
            Self::RightPar => Token::Bracket(Bracket::ParenRight),
            Self::VerticalLine => Token::Bracket(Bracket::VerticalLine),
            Self::Identifier(s) => Token::Id(s),
            Self::NumberStart => Token::Number(Number::zero()),
            Self::Number { num, .. } => Token::Number(Number::from(num)),
            Self::Fraction { num, fract, .. } => {
                let num = Number::from(num);
                let mut tmp = fract.clone();
                let mut fract_cnt = 0;

                while tmp != num::zero() {
                    tmp >>= 0;
                    fract_cnt += 1;
                }

                let fract = Number::new(fract, 10u128.pow(fract_cnt)).unwrap_or_default();
                Token::Number(num.add(fract).unwrap_or_default())
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
                    '0' => State::NumberStart,
                    '1'..='9' => State::Number {
                        radix: 10,
                        num: BigInt::from(ch.to_digit(10).unwrap()),
                    },
                    '.' => State::FractionStart,
                    'a'..='z' | 'A'..='Z' => State::Identifier(ch.to_string()),

                    _ => return Err(Error::UnsupportedToken(0)),
                };

                Some(next_state)
            }

            Self::Identifier(ref mut s) => 'id: {
                if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9') {
                    break 'id Some(State::Start);
                }

                s.push(ch);
                None
            }
            Self::NumberStart => match ch {
                '.' => Some(Self::Fraction {
                    radix: 10,
                    num: Default::default(),
                    fract: Default::default(),
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
                    num: BigInt::from(ch.to_digit(10).unwrap()),
                }),
                _ => Some(Self::Start),
            },

            Self::Number { radix, ref mut num } => 'number: {
                if ch == '.' {
                    break 'number Some(Self::Fraction {
                        radix: *radix,
                        num: mem::take(num),
                        fract: Default::default(),
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
                let Some(val) = ch.to_digit(10) else {
                    return Err(Error::UnsupportedToken(0));
                };

                Some(Self::Fraction {
                    radix: 10,
                    num: num::zero(),
                    fract: BigInt::from(val),
                })
            }

            Self::Fraction {
                radix,
                ref mut fract,
                ..
            } => 'fraction: {
                let Some(val) = ch.to_digit(*radix) else {
                    break 'fraction Some(Self::Start);
                };

                *fract *= *radix;
                *fract += val;

                None
            }
        };

        Ok(next)
    }
}

/// Scan tokens from string
pub struct Scanner<'a> {
    iter: std::str::Chars<'a>,
    state: State,
    cnt: usize,
}

impl<'a> Scanner<'a> {
    /// Create a new scanner to scan the given `s`
    pub fn new(s: &'a str) -> Self {
        Self {
            iter: s.chars(),
            state: State::Start,
            cnt: 0,
        }
    }

    /// Scan the given string into list of `Token`
    pub fn scan(&mut self) -> Result<Vec<Token>> {
        let mut res = Vec::new();

        while let Some(token) = self.step()? {
            res.push(token);
        }

        Ok(res)
    }

    fn step(&mut self) -> Result<Option<Token>> {
        let ch = match self.iter.next() {
            Some(ch) => ch,
            None if matches!(self.state, State::Start) => return Ok(None),
            None => {
                return mem::take(&mut self.state)
                    .to_token()
                    .ok_or(Error::UnsupportedToken(self.cnt))
                    .map(Some)
            }
        };
        self.cnt += 1;

        let Some(mut state) = self.state.next_state(ch).map_err(|_| Error::UnsupportedToken(self.cnt))? else {
            return Ok(None);
        };

        mem::swap(&mut state, &mut self.state);

        if let State::Start = self.state {
            self.iter.next_back();
            Ok(state.to_token())
        } else {
            Ok(None)
        }
    }
}
