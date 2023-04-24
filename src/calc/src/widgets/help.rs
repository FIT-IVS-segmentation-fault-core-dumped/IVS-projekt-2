use druid::{
    widget::{Flex, Padding, Scroll},
    Widget,
};

use crate::CalcState;

const WINDOW_PADDING: f64 = 10.;
pub struct HelpWin;

impl HelpWin {
    pub fn build_ui() -> impl Widget<CalcState> {
        let flex = Flex::column();

        Scroll::new(Padding::new(WINDOW_PADDING, flex))
    }
}
