use druid::{
    widget::{Flex, Padding, Scroll},
    Widget,
};

use crate::CalcState;

const WINDOW_PADDING: f64 = 10.;
pub struct AboutWin;

impl AboutWin {
    pub fn build_ui() -> impl Widget<CalcState> {
        Scroll::new(Padding::new(WINDOW_PADDING, Flex::column()))
    }
}
