//! The buttons UI part of the calculator

pub struct ButtonsUI;

use core::fmt;
use druid::widget::{Either, EnvScope, Flex, Padding, Painter};
use druid::{theme, Color, Insets, Key, Rect, RenderContext, WidgetExt, RoundedRectRadii};
use druid::{widget::Label, Widget};
use math::number::Radix;

use crate::{CalcState, FuncPad, Opt, PressedButton, Theme};

const BUTTON_PADDING: f64 = 1.0;
const BUTTON_BORDER_RADIUS: f64 = 3.0;
const BUTTON_TEXT_SIZE: f64 = 16.0;

const TAB_BOTTOM_MARGIN: f64 = 5.0;
const TAB_PADDING: Insets = Insets::uniform_xy(8.0, 0.0);
const TAB_UNDERLINE_SIZE: f64 = 4.0;

const TAB_TEXT_COLOR: Key<Color> = Key::<Color>::new("calc.active_textcolor");
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
            .with_flex_child(build_func_pad(), 1.0)
            .with_flex_child(build_num_pad(), 1.0)
    }
}

fn build_func_pad() -> impl Widget<CalcState> {
    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_flex_child(function_pad_tab(FuncPad::Main), 1.0)
                .with_flex_child(function_pad_tab(FuncPad::Func), 1.0),
            1.0,
        )
        .with_spacer(TAB_BOTTOM_MARGIN)
        .with_flex_child(
            Either::new(
                |data, _env| data.get_function_pad() == FuncPad::Func,
                make_func_btns(),
                make_main_btns(),
            ),
            5.0,
        )
}

fn build_num_pad() -> impl Widget<CalcState> {
    Flex::column()
        .with_flex_child(make_radix_tabs(), 1.0)
        .with_spacer(TAB_BOTTOM_MARGIN)
        .with_flex_child(
            Flex::row()
                .with_flex_child(make_button("←", Btn::MoveLeft, BtnType::Operation), 1.)
                .with_flex_child(make_button("→", Btn::MoveRight, BtnType::Operation), 1.)
                .with_flex_child(make_button("C", Btn::Clear, BtnType::Operation), 1.)
                .with_flex_child(make_button("⌫", Btn::Delete, BtnType::Operation), 1.),
            1.0,
        )
        .with_flex_child(
            Flex::row()
                .with_flex_child(make_number_keyboard(), 3.0)
                .with_flex_child(
                    Flex::column()
                        .with_flex_child(
                            make_button("÷", Btn::BinOpt(Opt::Div), BtnType::Operation),
                            1.,
                        )
                        .with_flex_child(
                            make_button("⨯", Btn::BinOpt(Opt::Mul), BtnType::Operation),
                            1.,
                        )
                        .with_flex_child(
                            make_button("+", Btn::BinOpt(Opt::Add), BtnType::Operation),
                            1.,
                        )
                        .with_flex_child(
                            make_button("-", Btn::BinOpt(Opt::Sub), BtnType::Operation),
                            1.0,
                        ),
                    1.0,
                ),
            4.0,
        )
}

// Buttons 0-9 + ANS + ,
fn make_num_btns() -> impl Widget<CalcState> {
    let first_row = Flex::row()
        .with_flex_child(
            make_button("7", Btn::Num(7), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            make_button("8", Btn::Num(8), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            make_button("9", Btn::Num(9), BtnType::Digit).disabled_if(|data, _env| {
                data.get_radix() == Radix::Bin || data.get_radix() == Radix::Oct
            }),
            1.,
        );

    let second_row = Flex::row()
        .with_flex_child(
            make_button("4", Btn::Num(4), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            make_button("5", Btn::Num(5), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            make_button("6", Btn::Num(6), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        );

    let third_row = Flex::row()
        .with_flex_child(make_button("1", Btn::Num(1), BtnType::Digit), 1.)
        .with_flex_child(
            make_button("2", Btn::Num(2), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        )
        .with_flex_child(
            make_button("3", Btn::Num(3), BtnType::Digit)
                .disabled_if(|data, _env| data.get_radix() == Radix::Bin),
            1.,
        );

    let forth_row = Flex::row()
        .with_flex_child(make_button(",", Btn::Comma, BtnType::Operation), 1.)
        .with_flex_child(make_button("0", Btn::Num(0), BtnType::Digit), 1.)
        .with_flex_child(make_button("=", Btn::Evaluate, BtnType::Operation), 1.);

    Flex::column()
        .main_axis_alignment(druid::widget::MainAxisAlignment::End)
        .with_flex_child(first_row, 1.0)
        .with_flex_child(second_row, 1.0)
        .with_flex_child(third_row, 1.0)
        .with_flex_child(forth_row, 1.0)
}

// Make either decimal or hexadecimal keyboard
fn make_number_keyboard() -> impl Widget<CalcState> {
    let first_row = Flex::row()
        .with_flex_child(make_button("D", Btn::Num(13), BtnType::Digit), 1.)
        .with_flex_child(make_button("E", Btn::Num(14), BtnType::Digit), 1.)
        .with_flex_child(make_button("F", Btn::Num(15), BtnType::Digit), 1.);

    let second_row = Flex::row()
        .with_flex_child(make_button("A", Btn::Num(10), BtnType::Digit), 1.)
        .with_flex_child(make_button("B", Btn::Num(11), BtnType::Digit), 1.)
        .with_flex_child(make_button("C", Btn::Num(12), BtnType::Digit), 1.);

    Either::new(
        |data, _env| data.get_radix() == Radix::Hex,
        Flex::column()
            .with_flex_child(first_row, 1.0)
            .with_flex_child(second_row, 1.0)
            .with_flex_child(make_num_btns(), 4.0),
        make_num_btns(),
    )
}

fn make_main_btns() -> impl Widget<CalcState> {
    let first_row = Flex::row()
        .with_flex_child(
            make_button("√", Btn::UnaryOpt(Opt::Sqrt), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("aⁿ", Btn::BinOpt(Opt::Pow), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("e", Btn::Const("e".to_owned()), BtnType::Function),
            1.,
        );

    let second_row = Flex::row()
        .with_flex_child(
            make_button("ⁿ√", Btn::BinOpt(Opt::Root), BtnType::Function),
            1.,
        )
        .with_flex_child(make_button("a²", Btn::Num(9), BtnType::Function), 1.)
        .with_flex_child(
            make_button("π", Btn::Const("π".to_owned()), BtnType::Function),
            1.,
        );

    let third_row = Flex::row()
        .with_flex_child(
            make_button("n!", Btn::UnaryOpt(Opt::Fact), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("|a|", Btn::UnaryOpt(Opt::Abs), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("Const", Btn::UnaryOpt(Opt::Abs), BtnType::Function),
            1.0,
        );

    let forth_row = Flex::row()
        .with_flex_child(
            make_button("sin", Btn::UnaryOpt(Opt::Sin), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("cos", Btn::UnaryOpt(Opt::Cos), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("ln", Btn::UnaryOpt(Opt::Ln), BtnType::Function),
            1.,
        );

    let fifth_row = Flex::row()
        .with_flex_child(make_button("(", Btn::BracketLeft, BtnType::Function), 1.)
        .with_flex_child(make_button(")", Btn::BracketRight, BtnType::Function), 1.)
        .with_flex_child(make_button("ANS", Btn::Ans, BtnType::Function), 1.);

    Flex::column()
        .with_flex_child(first_row, 1.0)
        .with_flex_child(second_row, 1.0)
        .with_flex_child(third_row, 1.0)
        .with_flex_child(forth_row, 1.0)
        .with_flex_child(fifth_row, 1.0)
}

fn make_func_btns() -> impl Widget<CalcState> {
    let first_row = Flex::row()
        .with_flex_child(
            make_button("sin", Btn::UnaryOpt(Opt::Sin), BtnType::Function),
            1.0,
        )
        .with_flex_child(
            make_button("cos", Btn::UnaryOpt(Opt::Cos), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("ln", Btn::UnaryOpt(Opt::Ln), BtnType::Function),
            1.,
        );

    let second_row = Flex::row()
        .with_flex_child(
            make_button("arcsin", Btn::UnaryOpt(Opt::Arcsin), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("arccos", Btn::UnaryOpt(Opt::Arccos), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("log", Btn::UnaryOpt(Opt::Log), BtnType::Function),
            1.,
        );

    let third_row = Flex::row()
        .with_flex_child(
            make_button("tg", Btn::UnaryOpt(Opt::Tg), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("cotg", Btn::UnaryOpt(Opt::Cotg), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("logₙ", Btn::BinOpt(Opt::LogN), BtnType::Function),
            1.,
        );

    let forth_row = Flex::row()
        .with_flex_child(
            make_button("arctg", Btn::UnaryOpt(Opt::Arctg), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("arccotg", Btn::UnaryOpt(Opt::Arccotg), BtnType::Function),
            1.,
        )
        .with_flex_child(
            make_button("mod", Btn::BinOpt(Opt::Mod), BtnType::Function),
            1.,
        );

    let fifth_row = Flex::row()
        .with_flex_child(make_button("nCr", Btn::BracketLeft, BtnType::Function), 1.)
        .with_flex_child(make_button("(", Btn::BracketLeft, BtnType::Function), 1.)
        .with_flex_child(make_button(")", Btn::BracketRight, BtnType::Function), 1.);

    Flex::column()
        .with_flex_child(first_row, 1.0)
        .with_flex_child(second_row, 1.0)
        .with_flex_child(third_row, 1.0)
        .with_flex_child(forth_row, 1.0)
        .with_flex_child(fifth_row, 1.0)
}

fn make_button(text: &str, button: Btn, button_type: BtnType) -> impl Widget<CalcState> {
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
        .with_flex_child(radix_tab(Radix::Dec), 1.0)
        .with_flex_child(radix_tab(Radix::Hex), 1.0)
        .with_flex_child(radix_tab(Radix::Oct), 1.0)
        .with_flex_child(radix_tab(Radix::Bin), 1.0)
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

fn function_pad_tab(text: FuncPad) -> impl Widget<CalcState> {
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
