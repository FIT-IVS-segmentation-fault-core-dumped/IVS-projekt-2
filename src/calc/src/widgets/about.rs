use druid::{
    widget::{Flex, Label, Padding, Scroll},
    Widget,
};

use crate::CalcState;

const WINDOW_PADDING: f64 = 10.;
pub struct AboutWin;

impl AboutWin {
    pub fn build_ui() -> impl Widget<CalcState> {
        Scroll::new(Padding::new(
            WINDOW_PADDING,
            Flex::column().with_child(
                Label::new(
                    "
    Calc
        Version 0.1
        ---------------------
        Calc was developed by a team of three students from VUT university as a school
        project.

        Our team members are:

            xkloub03
            xmorav48
            xnguye27

        We have developed this Calculator App with the goal of providing a powerful
        and easy-to-use tool for performing calculations. We have included a variety
        of functions, including trigonometric functions, combination number, nth root,
        square root, logarithm, factorial, and more.

        We hope that you find our Calc App useful for your needs.
        If you have any feedback or suggestions for how we can improve
        the app, please do not hesitate to let us know.

        Thank you for using Calc!

        Best regards,
        IVS team, segmentation fault (core dumped)",
                )
                .with_text_size(12.),
            ),
        ))
    }
}
