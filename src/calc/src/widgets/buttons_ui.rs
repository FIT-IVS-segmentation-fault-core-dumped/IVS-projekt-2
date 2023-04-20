//! The buttons UI part of the calculator

pub struct ButtonsUI;

use core::fmt;
use druid::widget::{Either, EnvScope, Flex, Padding, Painter};
use druid::{theme, Color, Insets, Key, Rect, RenderContext, RoundedRectRadii, WidgetExt};
use druid::{widget::Label, Widget};
use math::number::Radix;

use crate::{CalcState, FuncPad, Opt, PressedButton, Theme};

const BUTTON_PADDING: f64 = 1.0;
const BUTTON_BORDER_RADIUS: f64 = 3.0;
const BUTTON_TEXT_SIZE: f64 = 16.0;

const TAB_BOTTOM_MARGIN: f64 = 5.0;
const TAB_PADDING: Insets = Insets::uniform_xy(8.0, 0.0);
const TAB_UNDERLINE_SIZE: f64 = 4.0;
const TAB_TEXT_COLOR: Key<Color> = Key::<Color>::new("calc.tab_textcolor");
const TAB_ACTIVE_TEXT_COLOR_DARK: Color = Color::grey8(230);
const TAB_ACTIVE_TEXT_COLOR_LIGHT: Color = Color::grey8(10);
const TAB_ACTIVE_COLOR: &Color = &Color::rgb8(189, 197, 242);
const TAB_HOVER_COLOR: &Color = &Color::grey8(120);

const TAB_TEXT_SIZE: f64 = 14.0;

type Btn = PressedButton;

#[derive(Debug)]
enum BtnType {
    Digit,
    Operation,
    Function,
}

impl fmt::Display for BtnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BtnType::Digit => write!(f, "digit_btn"),
            BtnType::Operation => write!(f, "operation_btn"),
            BtnType::Function => write!(f, "func_btn"),
        }
    }
}

impl ButtonsUI {
    pub fn build_ui() -> impl Widget<CalcState> {
        Flex::row()
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
            .with_flex_child(make_func_part(), 1.)
            .with_flex_child(make_num_part(), 1.)
    }
}

fn make_func_part() -> impl Widget<CalcState> {
    let tabs = Flex::row()
        .with_flex_child(function_tab(FuncPad::Main), 1.0)
        .with_flex_child(function_tab(FuncPad::Func), 1.0)
        .with_flex_child(function_tab(FuncPad::Const), 1.0);

    let buttons = ViewSwitcher::new(
        |data: &CalcState, _env| data.get_function_pad(),
        |selector, _data, _env| match selector {
            FuncPad::Main => Box::new(make_main_btns()),
            FuncPad::Func => Box::new(make_func_btns()),
        },
    );

    Flex::column()
        .with_flex_child(tabs, 1.0)
        .with_spacer(TAB_BOTTOM_MARGIN)
        .with_flex_child(buttons, 5.0)
}

fn make_num_part() -> impl Widget<CalcState> {
    let tabs = Flex::row()
        .with_flex_child(generic_button("←", Btn::MoveLeft, BtnType::Operation), 1.)
        .with_flex_child(generic_button("→", Btn::MoveRight, BtnType::Operation), 1.)
        .with_flex_child(generic_button("C", Btn::Clear, BtnType::Operation), 1.)
        .with_flex_child(generic_button("⌫", Btn::Delete, BtnType::Operation), 1.);

    let operations = Flex::column()
        .with_flex_child(
            generic_button("÷", Btn::BinOpt(Opt::Div), BtnType::Operation),
            1.,
        )
        .with_flex_child(
            generic_button("⨯", Btn::BinOpt(Opt::Mul), BtnType::Operation),
            1.,
        )
        .with_flex_child(
            generic_button("+", Btn::BinOpt(Opt::Sum), BtnType::Operation),
            1.,
        )
        .with_flex_child(
            generic_button("-", Btn::BinOpt(Opt::Sub), BtnType::Operation),
            1.,
        );

    Flex::column()
        .with_flex_child(make_radix_tabs(), 1.)
        .with_spacer(TAB_BOTTOM_MARGIN)
        .with_flex_child(tabs, 1.)
        .with_flex_child(
            Flex::row()
                .with_flex_child(make_number_keyboard(), 3.)
                .with_flex_child(operations, 1.),
            4.,
        )
}

fn make_num_btns() -> impl Widget<CalcState> {
    let row_789 = Flex::row()
        .with_flex_child(
            generic_button("7", Btn::Num(7), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            generic_button("8", Btn::Num(8), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            generic_button("9", Btn::Num(9), BtnType::Digit).disabled_if(|data, _env| {
                data.get_radix() == Radix::Bin || data.get_radix() == Radix::Oct
            }),
            1.,
        );

    let row_456 = Flex::row()
        .with_flex_child(
            generic_button("4", Btn::Num(4), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            generic_button("5", Btn::Num(5), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            generic_button("6", Btn::Num(6), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        );

    let row_123 = Flex::row()
        .with_flex_child(generic_button("1", Btn::Num(1), BtnType::Digit), 1.)
        .with_flex_child(
            generic_button("2", Btn::Num(2), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            generic_button("3", Btn::Num(3), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        );

    let row_0 = Flex::row()
        .with_flex_child(generic_button(",", Btn::Comma, BtnType::Operation), 1.)
        .with_flex_child(generic_button("0", Btn::Num(0), BtnType::Digit), 1.)
        .with_flex_child(generic_button("=", Btn::Evaluate, BtnType::Operation), 1.);

    Flex::column()
        .main_axis_alignment(druid::widget::MainAxisAlignment::End)
        .with_flex_child(row_789, 1.)
        .with_flex_child(row_456, 1.)
        .with_flex_child(row_123, 1.)
        .with_flex_child(row_0, 1.)
}

// Make either decimal or hexadecimal keyboard
fn make_number_keyboard() -> impl Widget<CalcState> {
    let row_def = Flex::row()
        .with_flex_child(generic_button("D", Btn::Num(13), BtnType::Digit), 1.)
        .with_flex_child(generic_button("E", Btn::Num(14), BtnType::Digit), 1.)
        .with_flex_child(generic_button("F", Btn::Num(15), BtnType::Digit), 1.);

    let row_abc = Flex::row()
        .with_flex_child(generic_button("A", Btn::Num(10), BtnType::Digit), 1.)
        .with_flex_child(generic_button("B", Btn::Num(11), BtnType::Digit), 1.)
        .with_flex_child(generic_button("C", Btn::Num(12), BtnType::Digit), 1.);

    Either::new(
        |data, _env| data.get_radix() == Radix::Hex,
        Flex::column()
            .with_flex_child(row_def, 1.)
            .with_flex_child(row_abc, 1.)
            .with_flex_child(make_num_btns(), 4.),
        make_num_btns(),
    )
}

fn make_main_btns() -> impl Widget<CalcState> {
    let mut first_row = Flex::row();
    function_button(&mut first_row, "√", Btn::UnaryOpt(Opt::Sqrt));
    function_button(&mut first_row, "aⁿ", Btn::BinOpt(Opt::Pow));
    function_button(&mut first_row, "e", Btn::Const("e".to_owned()));

    let mut second_row = Flex::row();
    function_button(&mut second_row, "ⁿ√", Btn::BinOpt(Opt::Root));
    function_button(&mut second_row, "a²", Btn::Pow2);
    function_button(&mut second_row, "π", Btn::Const("π".to_owned()));

    let mut third_row = Flex::row();
    function_button(&mut third_row, "n!", Btn::UnaryOpt(Opt::Fact));
    function_button(&mut third_row, "|a|", Btn::UnaryOpt(Opt::Abs));
    function_button(&mut third_row, "³√", Btn::Root3);

    let mut forth_row = Flex::row();
    function_button(&mut forth_row, "sin", Btn::UnaryOpt(Opt::Sin));
    function_button(&mut forth_row, "cos", Btn::UnaryOpt(Opt::Cos));
    function_button(&mut forth_row, "ln", Btn::UnaryOpt(Opt::Ln));

    let mut fifth_row = Flex::row();
    function_button(&mut fifth_row, "ANS", Btn::Ans);
    function_button(&mut fifth_row, "(", Btn::BracketLeft);
    function_button(&mut fifth_row, ")", Btn::BracketRight);

    Flex::column()
        .with_flex_child(first_row, 1.)
        .with_flex_child(second_row, 1.)
        .with_flex_child(third_row, 1.)
        .with_flex_child(forth_row, 1.)
        .with_flex_child(fifth_row, 1.)
}

fn make_func_btns() -> impl Widget<CalcState> {
    let mut first_row = Flex::row();
    function_button(&mut first_row, "sin", Btn::UnaryOpt(Opt::Sin));
    function_button(&mut first_row, "cos", Btn::UnaryOpt(Opt::Cos));
    function_button(&mut first_row, "ln", Btn::UnaryOpt(Opt::Ln));

    let mut second_row = Flex::row();
    function_button(&mut second_row, "sin⁻¹", Btn::UnaryOpt(Opt::Arcsin));
    function_button(&mut second_row, "cos⁻¹", Btn::UnaryOpt(Opt::Arccos));
    function_button(&mut second_row, "log", Btn::UnaryOpt(Opt::Log));

    let mut third_row = Flex::row();
    function_button(&mut third_row, "tg", Btn::UnaryOpt(Opt::Tg));
    function_button(&mut third_row, "cotg", Btn::UnaryOpt(Opt::Cotg));
    function_button(&mut third_row, "logₙ", Btn::BinOpt(Opt::LogN));

    let mut forth_row = Flex::row();
    function_button(&mut forth_row, "tg⁻¹", Btn::UnaryOpt(Opt::Arctg));
    function_button(&mut forth_row, "cotg⁻¹", Btn::UnaryOpt(Opt::Arccotg));
    function_button(&mut forth_row, "mod", Btn::BinOpt(Opt::Mod));

    let mut fifth_row = Flex::row();
    function_button(&mut fifth_row, "nCr", Btn::BinOpt(Opt::Comb));
    function_button(&mut fifth_row, "(", Btn::BracketLeft);
    function_button(&mut fifth_row, ")", Btn::BracketRight);

    Flex::column()
        .with_flex_child(first_row, 1.)
        .with_flex_child(second_row, 1.)
        .with_flex_child(third_row, 1.)
        .with_flex_child(forth_row, 1.)
        .with_flex_child(fifth_row, 1.)
}

fn function_button(flex: &mut Flex<CalcState>, text: &str, button: Btn) {
    flex.add_flex_child(generic_button(text, button, BtnType::Function), 1.);
}

// Generic button for numbers, functions and operations
fn generic_button(text: &str, button: Btn, button_type: BtnType) -> impl Widget<CalcState> {
    Padding::new(
        BUTTON_PADDING,
        Label::new(text)
            .with_text_size(BUTTON_TEXT_SIZE)
            .center()
            .background(get_button_painter(button_type))
            .expand()
            .on_click(move |_ctx, data: &mut CalcState, _env| data.process_button(&button)),
    )
}

fn make_radix_tabs() -> impl Widget<CalcState> {
    Flex::row()
        .with_flex_child(radix_tab(Radix::Dec), 1.)
        .with_flex_child(radix_tab(Radix::Hex), 1.)
        .with_flex_child(radix_tab(Radix::Oct), 1.)
        .with_flex_child(radix_tab(Radix::Bin), 1.)
}

fn radix_tab(radix: Radix) -> impl Widget<CalcState> {
    EnvScope::new(
        |env, data| {
            if data.get_theme(true) == Theme::Dark {
                env.set(theme::DISABLED_TEXT_COLOR, TAB_ACTIVE_TEXT_COLOR_DARK);
            } else {
                env.set(theme::DISABLED_TEXT_COLOR, TAB_ACTIVE_TEXT_COLOR_LIGHT);
            }
        },
        Padding::new(
            TAB_PADDING,
            Label::new(format!("{:?}", radix))
                .with_text_color(TAB_TEXT_COLOR)
                .with_text_size(TAB_TEXT_SIZE)
                .center()
                .background(get_tab_painter())
                .expand()
                .disabled_if(move |data: &CalcState, _env| data.get_radix() == radix)
                .on_click(move |_ctx, data: &mut CalcState, _env| data.set_radix(radix)),
        ),
    )
}

fn function_tab(text: FuncPad) -> impl Widget<CalcState> {
    EnvScope::new(
        |env, data| {
            if data.get_theme(true) == Theme::Dark {
                env.set(theme::DISABLED_TEXT_COLOR, TAB_ACTIVE_TEXT_COLOR_DARK);
            } else {
                env.set(theme::DISABLED_TEXT_COLOR, TAB_ACTIVE_TEXT_COLOR_LIGHT);
            }
        },
        Padding::new(
            TAB_PADDING,
            Label::new(text.to_string())
                .with_text_color(TAB_TEXT_COLOR)
                .with_text_size(TAB_TEXT_SIZE)
                .center()
                .background(get_tab_painter())
                .expand()
                .disabled_if(move |data: &CalcState, _env| data.get_function_pad() == text)
                .on_click(move |_ctx, data: &mut CalcState, _env| data.set_function_pad(text)),
        ),
    )
}

fn get_tab_painter() -> Painter<CalcState> {
    Painter::new(|ctx, _data: &CalcState, _env| {
        let size = ctx.size();
        let rectangle_height = TAB_UNDERLINE_SIZE;
        let bounds = Rect::new(0.0, size.height, size.width, size.height - rectangle_height)
            .to_rounded_rect(5.0);

        if ctx.is_disabled() {
            ctx.fill(bounds, TAB_ACTIVE_COLOR);
        } else {
            // Outline when hovering
            if ctx.is_hot() {
                ctx.fill(bounds, TAB_HOVER_COLOR);
            }
            if ctx.is_active() {
                ctx.fill(bounds, TAB_HOVER_COLOR);
            }
        }
    })
}

fn get_button_painter(button_type: BtnType) -> Painter<CalcState> {
    Painter::new(move |ctx, data: &CalcState, env| {
        let bounds = ctx
            .size()
            .to_rounded_rect(RoundedRectRadii::from_single_radius(BUTTON_BORDER_RADIUS));

        let theme = data.get_theme(true);
        let background_key = Key::<Color>::new(Box::leak(
            format!("calc.{:?}.{}.background", theme, button_type).into_boxed_str(),
        ));

        ctx.fill(bounds, &env.get(&background_key));

        if !ctx.is_disabled() {
            let active_key = Key::<Color>::new(Box::leak(
                format!("calc.{:?}.{}.active", theme, button_type).into_boxed_str(),
            ));
            let hot_key = Key::<Color>::new(Box::leak(
                format!("calc.{:?}.{}.hover", theme, button_type).into_boxed_str(),
            ));

            // Outline when hovering
            if ctx.is_hot() {
                ctx.fill(bounds, &env.get(&hot_key));
            }
            // Color after button press
            if ctx.is_active() {
                ctx.fill(bounds, &env.get(&active_key));
            }
        }
    })
}
