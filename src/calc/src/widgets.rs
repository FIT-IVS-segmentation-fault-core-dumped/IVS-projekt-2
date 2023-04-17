//! Custom druid widgets for calculator
//!
pub mod menu;
pub mod buttons_ui;

use druid::widget::{Align, Container, Flex}; 
use druid::{Color, WidgetExt, UnitPoint, };
use druid::{widget::Label, Widget};
use crate::CalcState;


pub fn get_display() -> impl Widget<CalcState> {
    let status_row = Align::left(
            Flex::row()
                .with_flex_child(Label::new("Bin").with_text_size(10.0), 1.0)
                .with_flex_child(Label::new("Oct").with_text_size(10.0), 1.0)
                .with_flex_child(Label::new("Dec").with_text_size(10.0).with_text_color(Color::GREEN), 1.0)
                .with_flex_child(Label::new("Hex").with_text_size(10.0), 1.0)
                .with_flex_spacer(1.0)
                .with_flex_child(Label::new("Deg").with_text_size(10.0).with_text_color(Color::YELLOW), 1.0)
                .with_flex_child(Label::new("Rad").with_text_size(10.0), 1.0)
        );
    let expr_row = Align::left(
            Label::new(|data: &String, _env: &_| data.clone())
                .with_text_size(32.0)
                .lens(CalcState::displayed_text)
        );
    let result_row = Align::right(
            Label::new(|data: &String, _env: &_| data.clone())
                .with_text_size(28.0)
                .lens(CalcState::result)
        );

    Container::new(
            Flex::column()
                .with_flex_child(status_row, 1.0)
                .with_flex_child(expr_row, 1.0)
                .with_flex_child(result_row, 2.0)
                .align_vertical(UnitPoint::TOP)
                .padding((10.0, 0.0))
        )
        .border(Color::GRAY, 2.0)
        .rounded(5.0)
        .fix_height(100.0)
}

