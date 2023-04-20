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
            Opt::Add =>     ExprItem::new("+", "+"),
            Opt::Sub =>     ExprItem::new("-", "-"),
            Opt::Mul =>     ExprItem::new("⋅", "*"),
            Opt::Div =>     ExprItem::new("÷", "/"),
            Opt::Sin =>     ExprItem::new("sin ", "sin"),
            Opt::Cos =>     ExprItem::new("cos ", "cos"),
            Opt::Tg =>      ExprItem::new("tg ", "tg"),
            Opt::Cotg =>    ExprItem::new("cotg ", "cotg"),
            Opt::Arcsin =>  ExprItem::new("sin⁻¹ ", "arcsin"),
            Opt::Arccos =>  ExprItem::new("cos⁻¹ ", "arccos"),
            Opt::Arctg =>   ExprItem::new("tg⁻¹ ", "arctg"),
            Opt::Arccotg => ExprItem::new("cotg⁻¹ ", "arccotg"),
            Opt::Log =>     ExprItem::new("log ", "log10"),
            Opt::LogN =>    ExprItem::new("logₙ ", "log"),
            Opt::Ln =>      ExprItem::new("ln ", "ln"),
            Opt::Sqrt =>    ExprItem::new("√", "sqrt"),
            Opt::Root =>    ExprItem::new("ⁿ√", "root"),
            Opt::Root3 =>   ExprItem::new("³√", "root"),  // FIXME: How am I gonna handle this?
            Opt::Pow =>     ExprItem::new("^", "^"),
            Opt::Pow2 =>    ExprItem::new("²", "^2"),
            Opt::Abs =>     ExprItem::new("abs ", "abs"),
            Opt::Comb =>    ExprItem::new("C", "comb"),
            Opt::Fact =>    ExprItem::new("!", "!"),
            Opt::Mod =>     ExprItem::new("mod", "mod"),
            Opt::Random =>  ExprItem::new("⚄", "random"),
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
                ExprItem::new(&s, &s)
            }
            Self::BinOpt(opt) | Self::UnaryOpt(opt) => return opt.to_expr(),
            Self::BracketLeft =>  ExprItem::new("(", "("),
            Self::BracketRight => ExprItem::new(")", ")"),
            Self::Comma =>        ExprItem::new(",", "."),  // FIXME: Maybe we should localize this.
            Self::Const(name) =>  {
                // Replace known constants with their characters.
                match name.as_str() {
                    "pi" => ExprItem::new("π", "pi()"),
                    "phi" => ExprItem::new("ϕ", "phi()"),
                    _ => ExprItem::new(name, format!("{name}()"))
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
    eval: String
}

impl ExprItem {
    /// Construct ExprItem from anything convertable to string.
    pub fn new<T, U>(d: T, e: U) -> Self 
        where T: ToString,
              U: ToString
    {
        Self {
            disp: d.to_string(),
            eval: e.to_string(),
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
    /// widgets using lenses on CalcState properties.
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
    pub fn get_eval_str(&self) -> String {
        if self.btn_stack.is_empty() {
            return "0".to_string();
        }

        // Append all evaluation strings from the stack
        // FIXME: Convert some functions to functional notation.
        let mut eval_str = String::new();
        for btn in &self.btn_stack {
            eval_str += &match btn.to_expr() {
                Some(item) => item.eval,
                None => {
                    eprintln!("error: Cannot convert btn to expr. {:?}", btn);
                    continue
                },
            };
        }
        eval_str
    }
}

