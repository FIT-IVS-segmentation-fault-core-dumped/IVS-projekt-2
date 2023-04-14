//! Custom druid widgets for calculator
//!

use druid::{Widget, widget::Label, WidgetExt};
use crate::CalcState;

pub fn get_display() -> impl Widget<CalcState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
    .with_text_size(32.0)
    .lens(CalcState::displayed_text)
    .padding(5.0);

    display
}
