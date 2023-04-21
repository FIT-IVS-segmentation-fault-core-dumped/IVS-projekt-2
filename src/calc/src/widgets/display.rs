//! The display UI part of the calculator

use crate::CalcState;
use druid::widget::{Align, Container, Flex};
use druid::{theme, Color, Env, UnitPoint, WidgetExt};
use druid::{widget::Label, Widget};
use math::number::Radix;

pub const ACTIVE_RADIX_COLOR: Color = Color::GREEN;
pub const ACTIVE_TRIG_UNITS_COLOR: Color = Color::YELLOW;
pub const ERROR_MSG_COLOR: Color = Color::RED;

/// Widget displaying, **status of calculator**,
/// **current expression** and the **computed result**.
pub struct DisplayUI;
impl DisplayUI {
    /// Create the DisplayUI widget.
    pub fn build_ui() -> impl Widget<CalcState> {
        get_display()
    }
}

fn get_display() -> impl Widget<CalcState> {
    // As with druid it is hard to dynamically change color based on CalcState data state,
    // it is hard to change color of label dynamically. To solve this we change the
    // disabled color and use the `disabled_if` method, which atleast gives us two possible
    // states. See the status_row below.
    let gen_env = |color: Color| {
        move |env: &mut Env, _data: &CalcState| env.set(theme::DISABLED_TEXT_COLOR, color)
    };
    let radix_env = gen_env(ACTIVE_RADIX_COLOR);
    let tuni_env = gen_env(ACTIVE_TRIG_UNITS_COLOR);
    let radix_eq = |radix: Radix| move |data: &CalcState, _: &Env| data.radix == radix;
    let tuni_eq = |units: bool| move |data: &CalcState, _: &Env| data.degrees == units;

    let status_row = Align::left(
        Flex::row()
            .with_flex_child(
                Label::new("Dec")
                    .with_text_size(10.0)
                    .disabled_if(radix_eq(Radix::Dec))
                    .env_scope(radix_env),
                1.0,
            )
            .with_flex_child(
                Label::new("Hex")
                    .with_text_size(10.0)
                    .disabled_if(radix_eq(Radix::Hex))
                    .env_scope(radix_env),
                1.0,
            )
            .with_flex_child(
                Label::new("Oct")
                    .with_text_size(10.0)
                    .disabled_if(radix_eq(Radix::Oct))
                    .env_scope(radix_env),
                1.0,
            )
            .with_flex_child(
                Label::new("Bin")
                    .with_text_size(10.0)
                    .disabled_if(radix_eq(Radix::Bin))
                    .env_scope(radix_env),
                1.0,
            )
            .with_flex_spacer(1.0)
            .with_flex_child(
                Label::new("Deg")
                    .with_text_size(10.0)
                    .disabled_if(tuni_eq(true))
                    .env_scope(tuni_env),
                1.0,
            )
            .with_flex_child(
                Label::new("Rad")
                    .with_text_size(10.0)
                    .disabled_if(tuni_eq(false))
                    .env_scope(tuni_env),
                1.0,
            ),
    );
    let expr_row = Align::left(
        Label::new(|data: &String, _env: &_| data.clone())
            .with_text_size(32.0)
            .lens(CalcState::displayed_text),
    );
    let result_row = Align::right(
        Label::new(|data: &CalcState, _env: &_| data.result.clone())
            .with_text_size(28.0)
            .disabled_if(|data: &CalcState, _: &Env| data.result_is_err)
            .env_scope(gen_env(ERROR_MSG_COLOR))
            .lens(CalcState::all),
    );

    Container::new(
        Flex::column()
            .with_flex_child(status_row, 1.0)
            .with_flex_child(expr_row, 1.0)
            .with_flex_child(result_row, 2.0)
            .align_vertical(UnitPoint::TOP)
            .padding((10.0, 0.0)),
    )
    .border(Color::GRAY, 2.0)
    .rounded(5.0)
}
