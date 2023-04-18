//! Expression manager logic
use crate::*;

/// Manages mathematical expression, used in our calculation.
/// We use this, because we intend to have two different
/// strings for displaying and evaluating.
#[derive(Debug, Clone, Data)]
pub struct ExprManager {
    // Inner expressoin, which is used to convert to display
    // string or evaluate string.
    expr: String,
    // Cursor position in the string.
    cursor_pos: u32,
}

impl ExprManager {
    /// Create new instance of expression manager with empty
    /// expression.
    pub fn new() -> Self {
        Self {
            expr: "".to_string(),
            cursor_pos: 0,
        }
    }

    /// Process pressed button in calculator. This will
    /// edit expression string accordingly.
    pub fn process_button(&self, btn: &PressedButton) {
        todo!();
    }

    /// Get string to be displayed to [`DisplayUI`](widgets::display::DisplayUI).
    pub fn get_display_str(&self) -> String {
        if self.expr.is_empty() { "0".to_string() } else { self.expr.clone() }
    }

    /// Get string to be passed to [`Calculator`](math::Calculator).
    pub fn get_eval_str(&self) -> String {
        todo!();
    }
}
