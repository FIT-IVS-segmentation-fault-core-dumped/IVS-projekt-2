use druid::{
    widget::{Flex, Label, Padding, Scroll},
    Color, FontDescriptor, FontWeight, Widget,
};

use crate::CalcState;

const WINDOW_PADDING: f64 = 10.;
pub struct HelpWin;

impl HelpWin {
    pub fn build_ui() -> impl Widget<CalcState> {
        let mut page =
            Flex::column().cross_axis_alignment(druid::widget::CrossAxisAlignment::Start);

        page.add_child(build_section("Usage"));
        page.add_child(build_text(
            "
        The calculator app has a user-friendly interface that allows you to perform
        mathematical calculations quickly and easily. There are 3 sections in
        the interface, here is basic overview of them:
        ",
        ));

        page.add_child(build_subsection("Display Section"));
        page.add_child(build_text(
            "
        - The Display Section is where you can see your input and output.
        - Use the numpad on the right side to enter your mathematical expression.
        - The result of your calculation will be displayed on the right side of the display",
        ));

        page.add_child(build_subsection("Numpad and Basic Operator Section"));
        page.add_child(build_text(
            "
        - Use the numpad to enter the numbers and operators required for your
           calculation.",
        ));

        page.add_child(build_subsection("Function Button Section:"));
        page.add_child(build_text(
            "
            - Use the tabs to navigate to the Main Tab, Func Tab, or Const Tab.",
        ));

        page.add_child(build_subsection("Parentheses"));
        page.add_child(build_text(
            "
        - They include open parenthesis “(“ and close parenthesis “)”.
           Remember to follow the order of operations and use parentheses as necessary
           to ensure accurate calculations",
        ));

        page.add_child(build_subsection("ANS Button"));
        page.add_child(build_text(
            "
        - Allows you to use the last answer in your current calculation. Click on the
           ANS Button to insert the last answer into your mathematical expression.",
        ));

        page.add_child(build_subsection("Random Button"));
        page.add_child(build_text(
            "
        - Insert a random number into your mathematical expression.",
        ));

        page.add_child(build_section("Display Section"));
        page.add_child(build_text(
            "
            The calculator app comes with a display section that allows you to see
            the current mode (binary, octal,decimal, hex) and angular unit type
            (degree, radiant), user input, and calculation results.",
        ));

        page.add_child(build_subsection("Calculation Result"));
        page.add_child(build_text(
            "
        - The calculation result is located on the right-hand side of the user
           input section.
        - Once you have entered your expression, press the “=” button or hit
           enter on your keyboard to calculate the result. The result will be
           displayed in this section.",
        ));

        page.add_child(build_subsection("Modes"));
        page.add_child(build_text(
            "
        - To change the current mode, click on the corresponding button in the function
           buttons section. The current mode will be displayed at the top of the display
           section.",
        ));

        page.add_child(build_section("\nNumpad and Basic Operators"));
        page.add_child(build_text(
            "
            The calculator app comes with a numpad and basic operator section
            that allows you to input mathematical expressions using basic arithmetic
            operators.",
        ));

        page.add_child(build_subsection("Order of Operations"));
        page.add_child(build_text(
            "
        - The calculator app follows the order of operations when calculating expressions.
           The order of operations is as follows: Parentheses, Exponents, Multiplication
           and Division (from left to right), Addition and Subtraction (from le to right).",
        ));

        page.add_child(build_subsection("Clear and Delete"));
        page.add_child(build_text(
            "
        - The clear button (C) clears the user input section, allowing you to start over
           with a new expression. The delete button removes the character before the
           cursor in the user input section",
        ));

        page.add_child(build_section("\nFunctions"));
        page.add_child(build_text(
            "
            Thee calculator app comes with a Function Button section that allows you to
            access a range of functions to help you perform complex mathematical
            calculations. This section has three different tabs, each with a specific
            purpose, you can change between them by clicking onto it.",
        ));

        page.add_child(build_subsection("Main Tab"));
        page.add_child(build_text(
            "
        - Contains a range of mathematical functions, including parentheses, ANS for
           the last answer, nth root, square root, cubic root, power, exponential,
           absolute value, constants E and PI, factorial, random number, sin, and cos.",
        ));

        page.add_child(build_subsection("Func Tab"));
        page.add_child(build_text(
            "
        - Contains more advanced mathematical functions, including sin, cos, tan,
           cot, arcsin, arccos, arctan, arccot, logarithm, natural logarithm, log10,
           and modulo",
        ));

        page.add_child(build_subsection("Const Tab"));
        page.add_child(build_text(
            "
        - Click on the “Const” button to access this tab. Enter the name and value
           of your constant in the input fields provided, and click “+” to save it.
           Your constant will now be available in the Const Tab for use in your expressions.",
        ));

        Scroll::new(Padding::new(WINDOW_PADDING, page))
    }
}

fn build_section(text: &str) -> impl Widget<CalcState> {
    Label::new(format!("\n{}", text))
        .with_text_color(Color::WHITE)
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_size(18.)
}

fn build_subsection(text: &str) -> impl Widget<CalcState> {
    Label::new(format!("\n  {}", text))
        .with_font(FontDescriptor::default().with_weight(FontWeight::SEMI_BOLD))
        .with_text_size(14.)
}

fn build_text(text: &str) -> impl Widget<CalcState> {
    Label::new(text)
        .with_text_color(Color::grey8(200))
        .with_font(FontDescriptor::default().with_weight(FontWeight::NORMAL))
        .with_text_size(12.)
}
