//! Expression manager logic
#[cfg(test)]
mod tests;

use crate::*;

type Btn = PressedButton;

trait ToExpr {
    fn to_expr(&self) -> Option<ExprItem>;
}

impl ToExpr for Opt {
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
            Self::Root3 =>   ExprItem::new("³√",        "root",     3,      false,  false),  // FIXME: How am I gonna handle this?
            Self::Pow =>     ExprItem::new("^",         "^",        3,      false,  true),
            Self::Pow2 =>    ExprItem::new("²",         "^(2)",     3,      true,   true),
            Self::Abs =>     ExprItem::new("abs ",      "abs",      3,      false,  false),
            Self::Comb =>    ExprItem::new("C",         "comb",     2,      true,   false),
            Self::Fact =>    ExprItem::new("!",         "!",        3,      true,   true),
            Self::Mod =>     ExprItem::new("mod",       "mod",      2,      true,   true),
        })
    }
}

impl ToExpr for PressedButton {
    fn to_expr(&self) -> Option<ExprItem> {
        Some(match self {
            Self::Num(num) => {
                let s = match *num >= 10 {
                    true => char::from_digit(*num as u32, 16).unwrap().to_string().to_uppercase(),
                    false => num.to_string()
                };
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

// Holds strings, which are used to convert buttons
// to display/evaluate strings.
#[derive(Debug, Clone)]
struct ExprItem {
    disp: String,
    eval: String,
    // Priority of this function used in `ExprManager::to_postfix()` method.
    priority: u32,
    left_asoc: bool,
    // Skip convertion to function notation, when converting,
    // to evaluation string. Used in `ExprManager::to_eval_str()` method.
    skip_conv: bool,
}

impl ExprItem {
    /// Construct ExprItem from anything convertable to string.
    pub fn new<T, U>(disp: T, eval: U, priority: u32, left_asoc: bool, skip_conv: bool) -> Self 
        where T: ToString,
              U: ToString
    {
        Self {
            disp: disp.to_string(),
            eval: eval.to_string(),
            priority, left_asoc, skip_conv
        }
    }
}

/// Manages mathematical expression, used in our calculation.
/// We use this, because we intend to have two different
/// strings for displaying and evaluating.
#[derive(Debug, Clone)]
pub struct ExprManager {
    // Cursor position in the string.
    cursor_pos: u32,
    // Buttons, which compose the resulting expressoin string.
    btn_stack: Vec<Btn>,
    // Used for invalidating the expression manager (for druid repaint).
    dirty_flipper: bool
}

impl Data for ExprManager {
    // Dont compare `btn_stack` for performance reasons.
    fn same(&self, other: &Self) -> bool {
        self.dirty_flipper == other.dirty_flipper
            && self.cursor_pos == other.cursor_pos
    }
}

impl ExprManager {
    /// Create new instance of expression manager with empty
    /// expression.
    pub fn new() -> Self {
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
            Btn::Clear => self.btn_stack.clear(),
            Btn::Delete => { self.btn_stack.pop(); },
            Btn::MoveRight => self.move_cursor(false),
            Btn::MoveLeft => self.move_cursor(true),
            Btn::Evaluate => panic!("Invalid btn {:?}", btn),
            _ => self.btn_stack.push(btn.clone()),
        };
        self.invalidate();
    }

    /// Invalidate the ExprManager, forcing druid to redraw
    /// widgets that are using lenses on CalcState.
    pub fn invalidate(&mut self) {
        self.dirty_flipper = !self.dirty_flipper;
    }
    
    // Move cursor to the left or right in the evaluate expression string.
    fn move_cursor(&mut self, _left: bool) {
        todo!()
    }

    /// Get string to be displayed to [`DisplayUI`](widgets::display::DisplayUI).
    pub fn get_display_str(&self) -> String {
        // By default, the empty Display string is 0
        if self.btn_stack.is_empty() {
            return "0".to_string();
        }

        // Append all display strings from the stack.
        let mut disp_str = String::new();
        for btn in &self.btn_stack {
            disp_str += &match btn.to_expr() {
                Some(item) => item.disp,
                None => {
                    eprintln!("error: Cannot convert btn to expr. {:?}", btn);
                    continue
                },
            };
        }
        disp_str
    }

    /// Get string to be passed to [`Calculator`](math::Calculator).
    pub fn get_eval_str(&self) -> Result<String, String> {
        if self.btn_stack.is_empty() {
            return Ok("0".to_string());
        }

        let tokens = self.tokenize();
        let postfix = self.to_postfix(&tokens)?;

        Ok(self.to_eval_str(&postfix)?)
    }

    // Pop the operands off the stack, create resulting evaluate string,
    // and push onto the stack.
    fn push_func(&self, eval_stack: &mut Vec<Token>, token: &Token) -> Result<(), String> {
        let stack_size = eval_stack.len();
        if stack_size < token.arity as usize {
            return Err("Not enough operands for token".to_string());
        } else if token.arity == 0 {
            return Ok(())
        }
        
        let to_operand = |tok: &Token| -> String {
            if tok.btn == Btn::Evaluate 
                && tok.item.skip_conv 
                && tok.item.priority < token.item.priority 
                && token.item.skip_conv {
                format!("({})", tok.item.eval)
            } else {
                tok.item.eval.clone()
            }
        };
        let mut eval = String::new();
        match token.arity {
            1 => {
                let operand = to_operand(eval_stack.last().unwrap());
                eval = if token.item.skip_conv {
                    if token.btn == Btn::UnaryOpt(Opt::Fact) {
                        format!("{}!", operand)
                    } else {
                        format!("{}{}", token.item.eval, operand)
                    }
                } else {
                    format!("{}({})", token.item.eval, operand)
                }
            },
            2 => {
                let operand2 = to_operand(&eval_stack.pop().unwrap());
                let operand1 = to_operand(&eval_stack.last().unwrap());
                eval = if token.item.skip_conv {
                    format!("{}{}{}", operand1, token.item.eval, operand2)
                } else {
                    format!("{}({},{})", token.item.eval, operand1, operand2)
                }
            },
            _ => {}
        }
        let top = eval_stack.last_mut().unwrap();
        top.btn = Btn::Evaluate;
        top.item.eval = eval;
        top.item.skip_conv = token.item.skip_conv;
        top.item.priority = token.item.priority;
        top.arity = token.arity;

        Ok(())
    }
    
    // Construct evaluation string for the math library.
    fn to_eval_str(&self, postfix: &Vec<&Token>) -> Result<String, String> {
        if postfix.is_empty() {
            return Ok("".to_string());
        }

        let mut eval_stack: Vec<Token> = Vec::new();
        for token in postfix {
            match token.btn {
                Btn::Num(_) | Btn::Comma | Btn::Const(_) | Btn::Random => eval_stack.push((*token).clone()),
                Btn::UnaryOpt(_) | Btn::BinOpt(_) => { self.push_func(&mut eval_stack, token)? },
                Btn::Ans => todo!(),
                _ => unreachable!(),
            };
        }

        Ok(eval_stack.pop().unwrap().item.eval.clone())
    }

    // Convert tokens to postfix notation.
    fn to_postfix<'a>(&'a self, tokens: &'a Vec<Token>) -> Result<Vec<&'a Token>, String> {
        // Shunting Yard algorithm.
        let mut postfix: Vec<&Token> = Vec::new();
        let mut opt_stack: Vec<&Token> = Vec::new();

        for token in tokens {
            match token.btn {
                Btn::Num(_) | Btn::Comma | Btn::Const(_) => postfix.push(token),
                Btn::BinOpt(_) => {
                    while opt_stack.last().is_some() {
                        let top = opt_stack.last().unwrap();

                        if top.btn == Btn::BracketLeft ||
                            top.item.priority < token.item.priority ||
                            (top.item.priority == token.item.priority && !token.item.left_asoc) {
                                break;
                        }

                        postfix.push(opt_stack.pop().unwrap());
                    }
                    opt_stack.push(token);
                },
                Btn::UnaryOpt(_) | Btn::BracketLeft => opt_stack.push(token),
                Btn::BracketRight => {
                    // Pop all operators until the left bracket from the operator stack.
                    while opt_stack.last().is_some() && opt_stack.last().unwrap().btn != Btn::BracketLeft {
                        postfix.push(opt_stack.pop().unwrap());
                    }
                    // Check if left bracket was found.
                    if opt_stack.last().is_none() || opt_stack.last().unwrap().btn != Btn::BracketLeft {
                        return Err("Failed to match left parenthesis.".to_string());
                    }
                    // Pop the left bracket.
                    opt_stack.pop();
                    // If there is unary operator before the left bracket, then pop it to the
                    // postfix queue.
                    match opt_stack.last() {
                        Some(token) => if let Btn::UnaryOpt(_) = token.btn {
                            postfix.push(opt_stack.pop().unwrap());
                        },
                        None => {}
                    }
                }
                _ => return Err(format!("Invalid token button {:?}", token.btn)),
            };
        }

        // Pop any remaining operators from operator stack onto the postfix queue.
        while opt_stack.last().is_some() {
            postfix.push(opt_stack.pop().unwrap());
        }

        Ok(postfix)
    }

    // Tokenize the `btn_stack`.
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        for btn in &self.btn_stack {
            let btn_expr = match btn.to_expr() {
                Some(e) => e,
                None => continue,
            };

            // Check for implicit multiplication sign
            match btn {
                Btn::Num(_) => if let Some(tok) = tokens.last() {
                    if let Btn::UnaryOpt(Opt::Fact) = tok.btn {
                        tokens.push(Token::new(&Btn::BinOpt(Opt::Mul), Opt::Mul.to_expr().unwrap(), Some(2)))
                    }
                }
                Btn::UnaryOpt(Opt::Fact) => {},
                Btn::UnaryOpt(_) | Btn::Const(_) | Btn::BracketLeft => {
                    if let Some(tok) = tokens.last() {
                        match tok.btn {
                            Btn::Num(_) | Btn::Const(_) | Btn::UnaryOpt(Opt::Fact) => tokens.push(Token::new(&Btn::BinOpt(Opt::Mul), Opt::Mul.to_expr().unwrap(), Some(2))),
                            _ => {}
                        }
                    };
                },
                _ => {}
            }

            match btn {
                Btn::Num(num) => {
                    // Group numbers to tokens.
                    match tokens.last_mut() {
                        Some(tok) => match tok.btn {
                            // If the previous token is number or comma, then we merge them together.
                            Btn::Num(_) | Btn::Comma => tok.item.eval += &num.to_string(),
                            // Previous number is not a number, so create new token for this one.
                            _ => tokens.push(Token::new(btn, btn_expr, None)),
                        }
                        None => tokens.push(Token::new(btn, btn_expr, Some(0))),
                    }
                }
                Btn::Comma => {
                    // Group numbers to tokens.
                    match tokens.last_mut() {
                        Some(tok) => match tok.btn {
                            // If the previous token is number or comma, then we merge them together.
                            Btn::Num(_) | Btn::Comma => tok.item.eval += &btn_expr.eval,
                            // Previous number is not a number, so create new token for this one.
                            _ => tokens.push(Token::new(btn, btn_expr, None)),
                        }
                        None => tokens.push(Token::new(btn, btn_expr, Some(0))),
                    }
                }
                Btn::BinOpt(opt) => {
                    match opt {
                        // Check for arity. If the previous token is operator or left bracket
                        // or this is the first token, then this is unary operation.
                        Opt::Add | Opt::Sub => {
                            let arity = match tokens.last() {
                                Some(tok) => match tok.btn {
                                    Btn::BinOpt(o) | Btn::UnaryOpt(o) => {
                                        if o != Opt::Fact { 1 } else { 2 }
                                    },
                                    Btn::BracketLeft => 1,
                                    _ => 2
                                },
                                None => 1
                            };
                            tokens.push(Token::new(btn, btn_expr, Some(arity)));
                        },
                        _ => tokens.push(Token::new(btn, btn_expr, Some(2))),
                    };
                },
                Btn::UnaryOpt(_) => tokens.push(Token::new(btn, btn_expr, Some(1))),
                _ => tokens.push(Token::new(btn, btn_expr, None)),
            };
        }
        tokens
    }
}

// Reprezents a single token in expression string.
#[derive(Clone, Debug)]
struct Token {
    // When the btn is number, then a single token,
    // could be composed of multiple buttons (such
    // as numbers). In this case we only store the first
    // button and append to ItemExpr::expr string
    // to group numbers together.
    btn: Btn,
    item: ExprItem,
    arity: u32,
}

impl Token {
    pub fn new(btn: &Btn, mut item: ExprItem, arity: Option<u32>) -> Self {
        // The arity of '+' and '-' operators is determined
        // during the tokenization process. When they are
        // unary, we give them the unary priority.
        if arity.is_some() && arity.unwrap() == 1 {
            item.priority = 4;
        }
        Self { btn: btn.to_owned(), item, arity: arity.unwrap_or(0) }
    }
}


