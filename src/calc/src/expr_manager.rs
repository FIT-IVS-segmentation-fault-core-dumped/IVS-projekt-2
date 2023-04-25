//! Expression manager logic
#[cfg(test)]
mod tests;

use crate::*;

type Btn = PressedButton;

/// Will put continuous underscore to previous character in string.
pub const CURSOR_CHAR: char = '\u{02f0}';

trait ToExpr {
    fn to_expr(&self) -> Option<ExprItem>;
}

impl ToExpr for Opt {
    #[rustfmt::skip]
    fn to_expr(&self) -> Option<ExprItem> {
        Some(match self {
            //                             Disp         Eval     Priority LeftAsoc SkipConv
            Self::Add =>     ExprItem::new("+",         "+",        0,      true,   true),
            Self::Sub =>     ExprItem::new("-",         "-",        0,      true,   true),
            Self::Mul =>     ExprItem::new("⋅",         "*",        1,      true,   true),
            Self::Div =>     ExprItem::new("÷",         "/",        1,      true,   true),
            Self::Sin =>     ExprItem::new("sin ",      "sin",      3,      false,  false),
            Self::Cos =>     ExprItem::new("cos ",      "cos",      3,      false,  false),
            Self::Tg =>      ExprItem::new("tg ",       "tg",       3,      false,  false),
            Self::Cotg =>    ExprItem::new("cotg ",     "cotg",     3,      false,  false),
            Self::Arcsin =>  ExprItem::new("sin⁻¹ ",    "arcsin",   3,      false,  false),
            Self::Arccos =>  ExprItem::new("cos⁻¹ ",    "arccos",   3,      false,  false),
            Self::Arctg =>   ExprItem::new("tg⁻¹ ",     "arctg",    3,      false,  false),
            Self::Arccotg => ExprItem::new("cotg⁻¹ ",   "arccotg",  3,      false,  false),
            Self::Log =>     ExprItem::new("log ",      "log10",    3,      false,  false),
            Self::LogN =>    ExprItem::new("logₙ ",     "log",      2,      true,   false),
            Self::Ln =>      ExprItem::new("ln ",       "ln",       3,      false,  false),
            Self::Sqrt =>    ExprItem::new("√",         "sqrt",     3,      false,  false),
            Self::Root =>    ExprItem::new("ⁿ√",        "root",     2,      true,   false),
            Self::Root3 =>   ExprItem::new("³√",        "root",     3,      false,  false),
            Self::Pow =>     ExprItem::new("^",         "^",        3,      true,   true),
            Self::Pow2 =>    ExprItem::new("²",         "^2",       3,      true,   true),
            Self::Abs =>     ExprItem::new("abs ",      "abs",      3,      false,  false),
            Self::Comb =>    ExprItem::new("C",         "comb",     2,      true,   false),
            Self::Fact =>    ExprItem::new("!",         "!",        3,      true,   true),
            Self::Mod =>     ExprItem::new("mod",       "mod",      2,      true,   true),
        })
    }
}

impl ToExpr for PressedButton {
    #[rustfmt::skip]
    fn to_expr(&self) -> Option<ExprItem> {
        Some(match self {
            Self::Num(num) => {
                // Convert number to digit.
                let s = char::from_digit(*num as u32, 16)?
                    .to_ascii_uppercase()
                    .to_string();
                ExprItem::new(&s, &s, 0, true, true)
            }
            Self::BinOpt(opt) | Self::UnaryOpt(opt) => return opt.to_expr(),
            Self::BracketLeft =>  ExprItem::new("(", "(", 4, true, true),
            Self::BracketRight => ExprItem::new(")", ")", 4, true, true),
            Self::Comma =>        ExprItem::new(",", ".", 0, true, true),  // FIXME: Maybe we should localize this.
            Self::Random =>       ExprItem::new("⚄", "random", 3, true, true),
            Self::Const(name) =>  {
                // Replace known constants with their characters.
                match name.as_str() {
                    "pi" => ExprItem::new("π", "pi()", 0, true, true),
                    "phi" => ExprItem::new("ϕ", "phi()", 0, true, true),
                    _ => ExprItem::new(name, format!("{name}()"), 0, true, true)
                }
            },
            Self::Ans => todo!(),
            _ => return None,
        })
    }
}

/// Holds strings, which are used to convert buttons
/// to display/evaluate strings.
#[derive(Debug, Clone)]
struct ExprItem {
    disp: String,
    eval: String,

    // Below properties are used to correctly convert to
    // evaluate string.

    /// Priority of this function used in `ExprManager::to_postfix()` method.
    priority: u32,
    /// Flags if given button is left associative or not.
    /// If the button is not an operation, then this property
    /// is ignored.
    left_asoc: bool,
    /// Skip convertion to function notation, when converting,
    /// to evaluation string. Used in `ExprManager::to_eval_str()` method.
    /// Ignored for non-operation buttons.
    skip_conv: bool,
}

impl ExprItem {
    /// Construct ExprItem from anything convertable to string.
    pub fn new<T, U>(disp: T, eval: U, priority: u32, left_asoc: bool, skip_conv: bool) -> Self
    where
        T: ToString,
        U: ToString,
    {
        Self {
            disp: disp.to_string(),
            eval: eval.to_string(),
            priority,
            left_asoc,
            skip_conv,
        }
    }
}

/// Manages mathematical expression, used in our calculation.
/// We use this, because we intend to have two different
/// strings for displaying and evaluating.
#[derive(Debug, Clone)]
pub struct ExprManager {
    /// Cursor position in the string.
    cursor_pos: usize,
    /// Buttons, which compose the resulting expressoin string.
    btn_stack: Vec<Btn>,
    /// Used for invalidating the expression manager (for druid repaint).
    dirty_flipper: bool,
}

impl Data for ExprManager {
    // Dont compare `btn_stack` for performance reasons.
    fn same(&self, other: &Self) -> bool {
        self.dirty_flipper == other.dirty_flipper && self.cursor_pos == other.cursor_pos
    }
}

impl Default for ExprManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprManager {
    /// Create new instance of expression manager with empty
    /// expression.
    pub const fn new() -> Self {
        Self {
            dirty_flipper: true,
            cursor_pos: 0,
            btn_stack: Vec::new(),
        }
    }

    /// Process pressed button in calculator. This will
    /// edit expression string accordingly.
    ///
    /// # Panics
    /// When `btn` is `PressedButton::Evaluate` as `ExprManager`
    /// cannot compute results and thus should never get this.
    pub fn process_button(&mut self, btn: &PressedButton) {
        match btn {
            Btn::Clear =>  {
                self.btn_stack.clear();
                self.cursor_pos = 0;
            },
            Btn::Delete => {
                if self.cursor_pos != 0 {
                    self.btn_stack.remove(self.cursor_pos.saturating_sub(1));
                    self.move_cursor(true);
                }
            }
            Btn::MoveRight => self.move_cursor(false),
            Btn::MoveLeft => self.move_cursor(true),
            Btn::Evaluate => panic!("Cannot process `PressedButton::Evaluate`."),
            _ => {
                self.btn_stack.insert(self.cursor_pos, btn.clone());
                self.move_cursor(false);
            }
        };
        self.invalidate();
    }

    /// Invalidate the ExprManager, forcing druid to redraw
    /// widgets that are using lenses on CalcState.
    pub fn invalidate(&mut self) {
        self.dirty_flipper = !self.dirty_flipper;
    }

    /// Move cursor to the left or right in the evaluate expression string.
    fn move_cursor(&mut self, left: bool) {
        match left {
            true => self.cursor_pos = self.cursor_pos.saturating_sub(1),
            false => self.cursor_pos += 1
        };
        self.cursor_pos = self.cursor_pos.clamp(0, self.btn_stack.len());
    }

    /// Get string to be displayed to [`DisplayUI`](widgets::display::DisplayUI).
    pub fn get_display_str(&self, with_cursor: bool) -> String {
        // By default, the empty Display string is only cursor.
        if self.btn_stack.is_empty() {
            if with_cursor {
                // Zero-length whitespace, in order to combine the cursor character with something.
                return CURSOR_CHAR.to_string();
            } else {
                return "0".to_string();
            }
        }

        // Append all display strings from the stack.
        let mut disp_str = String::new();
        if self.cursor_pos == 0 && with_cursor {
            disp_str = CURSOR_CHAR.to_string();
        }
        for (i, btn) in self.btn_stack.iter().enumerate() {
            disp_str += &match btn.to_expr() {
                Some(item) => {
                    if i == self.cursor_pos.saturating_sub(1) && self.cursor_pos != 0 {
                        item.disp + &CURSOR_CHAR.to_string()
                    } else {
                        item.disp
                    }
                },
                None => {
                    eprintln!("error: Cannot convert btn to expr. {:?}", btn);
                    continue;
                }
            };
        }

        disp_str
    }

    /// Get string to be passed to [`Calculator`](math::Calculator).
    pub fn get_eval_str(&self) -> Result<String> {
        if self.btn_stack.is_empty() {
            return Ok("0".to_string());
        }

        // We convert button inputs to postfix notation and then assble the evaluation string.
        // In the process we convert oprators from binary to function notation. That is
        // skipped if `ExprItem::skip_conv` is set to `true`.

        // Tokenize the button inputs.
        let mut tokens = self.tokenize();
        // Convert them to postfix.
        let postfix = self.to_postfix(&mut tokens)?;
        // Generate final evaluation string from postfix.
        self.to_eval_str(&postfix)
    }

    /// Pop the operands off the stack, create resulting evaluate string,
    /// and push onto the stack.
    ///
    /// The final pushed Token has `Token::btn` set to `PressedButton::Evaluate`
    /// in order to differenciate between compound tokens and non-operation tokens.
    fn push_func(&self, eval_stack: &mut Vec<Token>, token: &Token) -> Result<()> {
        // Check if there are needed oprands on the stack for given function.
        let stack_size = eval_stack.len();
        if stack_size < token.arity as usize {
            return Err("Not enough operands for token".to_string().into());
        } else if token.arity == 0 {
            return Ok(());
        }

        // Converts token into operand. If the inner token is compound operation (it was already processed
        // by push_func) it is encapsulated in brackets if needed.
        let to_operand = |tok: &Token| -> String {
            if tok.btn == Btn::Evaluate         // Is compound operation
                && tok.item.skip_conv           // Inner operation is in _binary_ notation
                && tok.item.priority < token.item.priority  // Has lower priority than the
                                                            // encapsulating operation
                && token.item.skip_conv
            {
                // Encapsulating operation is in function
                // notation
                // Encapsulate inner operation in brackets.
                format!("({})", tok.item.eval)
            } else {
                // No need to encapsulate inner function, as the result will be unaffected.
                tok.item.eval.clone()
            }
        };

        // Evaluate string, that will be pushed with the new Token onto the stack.
        let mut eval = String::new();

        // Based on the arity, take given number of operands and convert them to
        // function or binary notation. Note that 1 operand is left on the stack,
        // so we can edit its contents instead of pushing new token.
        match token.arity {
            1 => {
                let operand = to_operand(eval_stack.last().unwrap());
                eval = if token.item.skip_conv {
                    if let Btn::UnaryOpt(Opt::Fact | Opt::Pow2) = token.btn {
                        // Unary notation on the right side.
                        format!("{}{}", operand, token.item.eval)
                    } else {
                        // Unary notation on the left side.
                        format!("{}{}", token.item.eval, operand)
                    }
                } else {
                    // Math library doesn't have a function for 3. root
                    if let Btn::UnaryOpt(Opt::Root3) = token.btn {
                        format!("{}(3, {})", token.item.eval, operand)
                    } else {
                        // Unary function notation.
                        format!("{}({})", token.item.eval, operand)
                    }
                }
            }
            2 => {
                let operand2 = to_operand(&eval_stack.pop().unwrap());
                let operand1 = to_operand(eval_stack.last().unwrap());
                eval = if token.item.skip_conv {
                    // Binary notation. If we skip convertion.
                    format!("{}{}{}", operand1, token.item.eval, operand2)
                } else {
                    // Binary function notation.
                    format!("{}({},{})", token.item.eval, operand1, operand2)
                }
            }
            _ => {}
        }

        // Change the top of the stack to the new compound operation (insead
        // of creating new one).
        let top = eval_stack.last_mut().unwrap();
        top.btn = Btn::Evaluate; // Mark as compound operation.
        top.item.eval = eval; // Set the newly generated evaluate string. Note that we don't
                              // care about disp string as it is not used in the convertion
                              // process.
        top.item.skip_conv = token.item.skip_conv;
        top.item.priority = token.item.priority;
        top.arity = token.arity;

        Ok(())
    }   // push_func()

    /// Construct final evaluation string from the tokens in postfix order.
    fn to_eval_str(&self, postfix: &Vec<&Token>) -> Result<String> {
        if postfix.is_empty() {
            // This should never happen, as it is already handled in `get_eval_str()`.
            // But just to know, if something has gone wrong.
            panic!("Postfix is empty.");
        }

        // Holds the evaluated compound tokens.
        let mut eval_stack: Vec<Token> = Vec::new();

        // Evaluate tokens. Push the non-operation tokens onto the stack
        // and then convert them to operations based on the operation tokens.
        // The resulting compound token will be left on the top.
        for token in postfix {
            match token.btn {
                // Non-operation tokens. Just push them onto the stack.
                Btn::Num(_) | Btn::Comma | Btn::Const(_) | Btn::Random => {
                    eval_stack.push((*token).clone())
                }
                // Operation tokens. This will pop the non-operation tokens (number depends on `token.arity`)
                // and create a compound token on the top of the stack.
                Btn::UnaryOpt(_) | Btn::BinOpt(_) => self.push_func(&mut eval_stack, token)?,
                // TODO: Handle ans
                Btn::Ans => todo!(),
                // There should only be operation and non-operation tokens on the stack.
                _ => panic!("Invalid token on the evaluate stack. {:?}", token),
            };
        }

        // Pop the resulting compound token and get its eval string.
        // We can safely unwrap it, because the `postfix` is never empty.
        Ok(eval_stack.pop().unwrap().item.eval)
    }

    /// Convert tokens to postfix notation.
    fn to_postfix<'a>(&'a self, tokens: &'a mut [Token]) -> Result<Vec<&'a Token>> {
        // Shunting Yard algorithm. Based on pseudo-code
        // from [wiki](https://en.wikipedia.org/wiki/Shunting_yard_algorithm).

        // The token queue.
        let mut postfix: Vec<&Token> = Vec::new();
        // The option stack for operation tokens.
        let mut opt_stack: Vec<&Token> = Vec::new();

        for token in tokens.iter_mut() {
            match token.btn {
                Btn::Num(_) | Btn::Comma | Btn::Const(_) => postfix.push(token),
                Btn::Random => {
                    token.btn = Btn::Const("random".to_string());
                    token.arity = 0;
                    token.item = token.btn.to_expr().unwrap();
                    postfix.push(token);
                }
                Btn::BinOpt(_) => {
                    while opt_stack.last().is_some() {
                        let top = opt_stack.last().unwrap();

                        if top.btn == Btn::BracketLeft
                            || top.item.priority < token.item.priority
                            || (top.item.priority == token.item.priority && !token.item.left_asoc)
                        {
                            break;
                        }

                        postfix.push(opt_stack.pop().unwrap());
                    }
                    opt_stack.push(token);
                }
                Btn::UnaryOpt(_) | Btn::BracketLeft => opt_stack.push(token),
                Btn::BracketRight => {
                    // Pop all operators until the left bracket from the operator stack.
                    while opt_stack.last().is_some()
                        && opt_stack.last().unwrap().btn != Btn::BracketLeft
                    {
                        postfix.push(opt_stack.pop().unwrap());
                    }
                    // Check if left bracket was found.
                    if opt_stack.last().is_none()
                        || opt_stack.last().unwrap().btn != Btn::BracketLeft
                    {
                        return Err("Failed to match left parenthesis.".to_string().into());
                    }
                    // Pop the left bracket.
                    opt_stack.pop();
                    // If there is unary operator before the left bracket, then pop it to the
                    // postfix queue.
                    if let Some(token) = opt_stack.last() {
                        if let Btn::UnaryOpt(_) = token.btn {
                            postfix.push(opt_stack.pop().unwrap());
                        }
                    }
                }
                _ => return Err(format!("Invalid token button {:?}", token.btn).into()),
            };
        }

        // Pop any remaining operators from operator stack onto the postfix queue.
        while opt_stack.last().is_some() {
            postfix.push(opt_stack.pop().unwrap());
        }

        Ok(postfix)
    }

    /// Tokenize the `btn_stack`.
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        for btn in &self.btn_stack {
            let Some(btn_expr) = btn.to_expr() else {
                continue;
            };

            // Check for implicit multiplication sign. And add it if found.
            // We need to explicitly handle unary operations (for now only Opt::Fact),
            // that are on the right side of the operand.
            match btn {
                // Case: "<righ unary><number>" --> "5!2" ~ "5!*2"
                Btn::Num(_) => {
                    if let Some(tok) = tokens.last() {
                        if let Btn::UnaryOpt(Opt::Fact | Opt::Pow2) = tok.btn {
                            tokens.push(Token::new(
                                &Btn::BinOpt(Opt::Mul),
                                Opt::Mul.to_expr().unwrap(),
                                Some(2),
                            ))
                        }
                    }
                }
                // Ignore right sided unary operations.
                Btn::UnaryOpt(Opt::Fact | Opt::Pow2) => {}
                // Case: "<num|const|right unary><left unary|const|'('>" --> "5!sqrt3" ~ "5!*sqrt3"
                Btn::UnaryOpt(_) | Btn::Const(_) | Btn::BracketLeft => {
                    if let Some(tok) = tokens.last() {
                        match tok.btn {
                            Btn::Num(_) | Btn::Const(_) | Btn::UnaryOpt(Opt::Fact | Opt::Pow2) => {
                                tokens.push(Token::new(
                                    &Btn::BinOpt(Opt::Mul),
                                    Opt::Mul.to_expr().unwrap(),
                                    Some(2),
                                ))
                            }
                            _ => {}
                        }
                    };
                }
                _ => {}
            }

            match btn {
                // Tokenize numbers. Group numbers next to each other into single token.
                Btn::Num(num) => {
                    match tokens.last_mut() {
                        Some(tok) => match tok.btn {
                            // If the previous token is number or comma, then we merge them together.
                            Btn::Num(_) | Btn::Comma => tok.item.eval += &num.to_string(),
                            // Previous number is not a number, so create new token for this one.
                            _ => tokens.push(Token::new(btn, btn_expr, None)),
                        },
                        None => tokens.push(Token::new(btn, btn_expr, Some(0))),
                    }
                }
                // Tokenize comma. If next to number, then group it into single token.
                Btn::Comma => {
                    match tokens.last_mut() {
                        Some(tok) => match tok.btn {
                            // If the previous token is number or comma, then we merge them together.
                            Btn::Num(_) | Btn::Comma => tok.item.eval += &btn_expr.eval,
                            // Previous number is not a number, so create new token for this one.
                            _ => tokens.push(Token::new(btn, btn_expr, None)),
                        },
                        None => tokens.push(Token::new(btn, btn_expr, Some(0))),
                    }
                }
                // Tokenize binary operation.
                // Check for arity of '+' and '-' operators. These could actually
                // be unary, based on the previous token.
                Btn::BinOpt(Opt::Add | Opt::Sub) => {
                    let arity = match tokens.last() {
                        Some(tok) => match tok.btn {
                            // Case: "1*-3"
                            Btn::BinOpt(_) => 1,
                            // If previous token is right-sided unary operation,
                            // then this is binary, as it has bigger priority.
                            // Case: "sin 5!-3"
                            Btn::UnaryOpt(Opt::Fact | Opt::Pow2) => 2,
                            // Case: "2*(-3)"
                            Btn::BracketLeft => 1,
                            _ => 2,
                        },
                        None => 1,
                    };
                    tokens.push(Token::new(btn, btn_expr, Some(arity)));
                },
                Btn::BinOpt(_) => tokens.push(Token::new(btn, btn_expr, Some(2))),
                Btn::UnaryOpt(_) => tokens.push(Token::new(btn, btn_expr, Some(1))),
                _ => tokens.push(Token::new(btn, btn_expr, None)),
            };
        } // for btn in tokens
        tokens
    } // tokenize()
} // ExprManager

/// Reprezents a single token in expression string.
#[derive(Clone, Debug)]
struct Token {
    /// When the btn is number, then a single token,
    /// could be composed of multiple buttons (such
    /// as numbers). In this case we only store the first
    /// button and append to ItemExpr::expr string
    /// to group numbers together.
    btn: Btn,
    /// Item associated with `btn`
    item: ExprItem,
    /// Arity of the `btn` operation.
    arity: u32,
}

impl Token {
    pub fn new(btn: &Btn, mut item: ExprItem, arity: Option<u32>) -> Self {
        // The arity of '+' and '-' operators is determined
        // during the tokenization process. When they are
        // unary, we give them the unary priority.
        if arity.is_some() && arity.unwrap() == 1 {
            item.priority = 3;
        }
        Self {
            btn: btn.to_owned(),
            item,
            arity: arity.unwrap_or(0),
        }
    }
}

