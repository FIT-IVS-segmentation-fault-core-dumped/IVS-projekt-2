//! The buttons UI part of the calculator

pub struct ButtonsUI;

use crate::{CalcState, Constants, FunctionTabs, Opt, PressedButton, Theme};
use core::fmt;
use druid::commands::CLOSE_WINDOW;
use druid::kurbo::Circle;
use druid::widget::{Controller, Either, EnvScope, Flex, Padding, Painter, TextBox, ViewSwitcher};
use druid::{
    theme, Color, Env, Event, EventCtx, Insets, Key, LensExt, LifeCycle, LifeCycleCtx, Menu,
    MenuItem, MouseButton, Point, Rect, RenderContext, RoundedRectRadii, Selector, Size,
    TimerToken, UnitPoint, WidgetExt, WindowConfig, WindowId, WindowLevel, WindowSizePolicy,
};
use druid::{widget::Label, Widget};
use math::number::Radix;
use rust_i18n::t;
use std::rc::Rc;
use std::time::{Duration, Instant};

const BUTTON_PADDING: f64 = 1.0;
const BUTTON_BORDER_RADIUS: f64 = 3.0;
const BUTTON_TEXT_SIZE: f64 = 16.0;

const ADD_CONST_BUTTON_TEXT_SIZE: f64 = 22.0;

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

/// Encapsulation of the keyboard user interface
impl ButtonsUI {
    /// Render all buttons used to control the display
    pub fn build_ui() -> impl Widget<CalcState> {
        Flex::row()
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
            .with_flex_child(make_func_part(), 1.)
            .with_flex_child(make_num_part(), 1.)
    }
}

// Button types that determines color of different buttons
enum BtnType {
    Digit,
    Operation,
    Function,
}

// String representation of `BtnType`
impl fmt::Display for BtnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BtnType::Digit => write!(f, "digit_btn"),
            BtnType::Operation => write!(f, "operation_btn"),
            BtnType::Function => write!(f, "func_btn"),
        }
    }
}

// Controller that displays a context menu when right-clicked. `index` represents position of a particular
// constant in the vector of the user defined constants.
struct ShowContextMenu {
    index: usize,
}

impl ShowContextMenu {
    fn new(index: usize) -> Self {
        Self { index }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for ShowContextMenu {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::MouseDown(mouse_event) = event {
            if mouse_event.button == MouseButton::Right {
                let index = Rc::new(self.index);
                let menu: Menu<CalcState> =
                    Menu::empty().entry(MenuItem::new(t!("context_menu.remove")).on_activate(
                        move |_ctx, data: &mut CalcState, _env| {
                            data.remove_constant(*index);
                        },
                    ));
                ctx.show_context_menu(menu, mouse_event.window_pos);
            }
        }
        child.event(ctx, event, data, env)
    }
}

// Widget that makes layout from function tabs and one of the function keyboard (Main, Func, Const)
fn make_func_part() -> impl Widget<CalcState> {
    let tabs = Flex::row()
        .with_flex_child(function_tab(FunctionTabs::Main), 1.)
        .with_flex_child(function_tab(FunctionTabs::Func), 1.)
        .with_flex_child(function_tab(FunctionTabs::Const), 1.);

    let buttons = ViewSwitcher::new(
        |data: &CalcState, _env| data.get_function_pad(),
        |selector, _data, _env| match selector {
            FunctionTabs::Main => Box::new(make_main_btns()),
            FunctionTabs::Func => Box::new(make_func_btns()),
            FunctionTabs::Const => Box::new(make_const_btns()),
        },
    );

    Flex::column()
        .with_flex_child(tabs, 1.)
        .with_spacer(TAB_BOTTOM_MARGIN)
        .with_flex_child(buttons, 5.)
}

// Make constants buttons with a field for adding new constants
fn make_const_btns() -> impl Widget<CalcState> {
    Flex::column()
        .with_flex_child(make_const_grid(), 4.)
        .with_flex_child(make_add_const_field(), 1.)
}

// Dynamic grid that change its size based on number of defined constants
fn make_const_grid() -> impl Widget<CalcState> {
    ViewSwitcher::new(
        move |data: &CalcState, _env| data.get_constants().keys.len(),
        move |selector, _data, _env| {
            let mut flex = Flex::column();

            // Row that is always drawn with unremovable buttons for constants 'e' and 'π'.
            let default_row = Flex::row()
                .with_flex_child(
                    generic_button("e", Btn::Const("e".to_owned()), BtnType::Function)
                        .controller(TooltipController::new("2.71".to_string())),
                    1.,
                )
                .with_flex_child(
                    generic_button("π", Btn::Const("pi".to_owned()), BtnType::Function)
                        .controller(TooltipController::new("3.14".to_string())),
                    1.,
                )
                .with_flex_child(make_const_button(0), 1.);

            flex.add_flex_child(default_row, 1.);

            // Other rows with user defined constants. The number of rows depends on the number of constants.
            let line_count = (selector + 1) / 3;
            for i in 0..line_count {
                let mut row = Flex::row();

                for j in 0..3 {
                    row.add_flex_child(make_const_button(i * 3 + j + 1), 1.);
                }
                flex.add_flex_child(row, 1.);
            }

            // Fill the available space so that the minimum number of rows of grid is 4
            match line_count {
                0 => flex.add_flex_spacer(3.),
                1 => flex.add_flex_spacer(2.),
                2 => flex.add_flex_spacer(1.),
                _ => flex.add_flex_spacer(0.),
            }

            Box::new(flex)
        },
    )
}

enum TooltipState {
    Showing(WindowId),
    Waiting {
        last_move: Instant,
        timer_expire: Instant,
        token: TimerToken,
        position_in_window_coordinates: Point,
    },
    Fresh,
}

// A Controller responsible for listening to mouse hovers and launching tooltip windows.
// https://github.com/linebender/druid/blob/master/druid/examples/sub_window.rs
struct TooltipController {
    tip: String,
    state: TooltipState,
}

impl TooltipController {
    pub fn new(tip: impl Into<String>) -> Self {
        TooltipController {
            tip: tip.into(),
            state: TooltipState::Fresh,
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for TooltipController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let wait_duration = Duration::from_millis(300);
        let resched_dur = Duration::from_millis(50);
        let cursor_size = Size::new(10., 25.);
        let now = Instant::now();
        let new_state = match &self.state {
            TooltipState::Fresh => match event {
                Event::MouseMove(me) if ctx.is_hot() => Some(TooltipState::Waiting {
                    last_move: now,
                    timer_expire: now + wait_duration,
                    token: ctx.request_timer(wait_duration),
                    position_in_window_coordinates: me.window_pos,
                }),
                _ => None,
            },
            TooltipState::Waiting {
                last_move,
                timer_expire,
                token,
                position_in_window_coordinates,
            } => match event {
                Event::MouseMove(me) if ctx.is_hot() => {
                    let (cur_token, cur_expire) = if *timer_expire - now < resched_dur {
                        (ctx.request_timer(wait_duration), now + wait_duration)
                    } else {
                        (*token, *timer_expire)
                    };
                    Some(TooltipState::Waiting {
                        last_move: now,
                        timer_expire: cur_expire,
                        token: cur_token,
                        position_in_window_coordinates: me.window_pos,
                    })
                }
                Event::Timer(tok) if tok == token => {
                    let deadline = *last_move + wait_duration;
                    ctx.set_handled();
                    if deadline > now {
                        let wait_for = deadline - now;
                        Some(TooltipState::Waiting {
                            last_move: *last_move,
                            timer_expire: deadline,
                            token: ctx.request_timer(wait_for),
                            position_in_window_coordinates: *position_in_window_coordinates,
                        })
                    } else {
                        let tooltip_position_in_window_coordinates =
                            (position_in_window_coordinates.to_vec2() + cursor_size.to_vec2())
                                .to_point();
                        let win_id = ctx.new_sub_window(
                            WindowConfig::default()
                                .show_titlebar(false)
                                .window_size_policy(WindowSizePolicy::Content)
                                .set_level(WindowLevel::Tooltip(ctx.window().clone()))
                                .set_position(tooltip_position_in_window_coordinates),
                            Label::<()>::new(self.tip.clone())
                                .background(env.get(theme::WINDOW_BACKGROUND_COLOR)),
                            (),
                            env.clone(),
                        );
                        Some(TooltipState::Showing(win_id))
                    }
                }
                _ => None,
            },
            TooltipState::Showing(win_id) => match event {
                Event::MouseMove(me) if !ctx.is_hot() => {
                    ctx.submit_command(CLOSE_WINDOW.to(*win_id));
                    Some(TooltipState::Waiting {
                        last_move: now,
                        timer_expire: now + wait_duration,
                        token: ctx.request_timer(wait_duration),
                        position_in_window_coordinates: me.window_pos,
                    })
                }
                _ => None,
            },
        };

        if let Some(state) = new_state {
            self.state = state;
        }

        if !ctx.is_handled() {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env,
    ) {
        if let LifeCycle::HotChanged(false) = event {
            if let TooltipState::Showing(win_id) = self.state {
                ctx.submit_command(CLOSE_WINDOW.to(win_id));
            }
            self.state = TooltipState::Fresh;
        }
        child.lifecycle(ctx, event, data, env)
    }
}

// Make a button that represents a constant at the given `index`.
// If there is not any constant at the given `index`, the empty widget is returned.
fn make_const_button(index: usize) -> impl Widget<CalcState> {
    ViewSwitcher::new(
        move |data: &CalcState, _env| data.get_constants().keys.get(index).is_none(),
        move |selector, _data, _env| match selector {
            true => Box::new(Label::new("")),
            false => {
                let key = _data.get_constants().keys.get(index).unwrap();
                let value = _data.get_constants().values.get(index).unwrap();

                Box::new(
                    generic_button(key, Btn::Const(key.to_string()), BtnType::Function)
                        .controller(TooltipController::new(value.to_string()))
                        .controller(ShowContextMenu::new(index)),
                )
            }
        },
    )
}

// Prevents text in `TextBox` from exceeding limit `max_length` characters
struct LengthController {
    max_length: usize,
}

impl LengthController {
    fn new(max_length: usize) -> Self {
        Self { max_length }
    }
}

impl<W: Widget<CalcState>> Controller<CalcState, W> for LengthController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut CalcState,
        env: &Env,
    ) {
        if let Event::KeyDown(key_event) = event {
            if matches!(
                key_event.key,
                druid::keyboard_types::Key::ArrowLeft
                    | druid::keyboard_types::Key::ArrowRight
                    | druid::keyboard_types::Key::ArrowUp
                    | druid::keyboard_types::Key::ArrowDown
                    | druid::keyboard_types::Key::Delete
                    | druid::keyboard_types::Key::Backspace
            ) {
                // Allow arrow keys, delete key, and backspace key
                child.event(ctx, event, data, env);
            } else {
                // Check if the length of the current text in the textbox
                // is greater than or equal to the maximum allowed length
                if data.get_constants().key_str.len() >= self.max_length {
                    ctx.set_handled();
                    return;
                }
            }
        }
        child.event(ctx, event, data, env);
    }
}

// Group of widgets that enable user to add own constants to the application
fn make_add_const_field() -> impl Widget<CalcState> {
    let mut flex = Flex::row();
    let key_field = TextBox::new()
        .with_text_alignment(druid::TextAlignment::Center)
        .with_placeholder(t!("constants.name"))
        .padding(5.)
        .expand_height()
        .align_vertical(UnitPoint::CENTER)
        .lens(CalcState::constants.then(Constants::key_str))
        .controller(LengthController::new(3));

    let value_field = TextBox::new()
        .with_placeholder(t!("constants.value"))
        .expand_height()
        .padding(5.)
        .expand_width()
        .align_vertical(UnitPoint::CENTER)
        .lens(CalcState::constants.then(Constants::value_str));

    flex.add_flex_child(key_field, 1.5);
    flex.add_child(Label::new("="));
    flex.add_flex_child(value_field, 3.);
    flex.add_child(make_add_const_btn());
    flex
}

fn make_add_const_btn() -> impl Widget<CalcState> {
    Label::new("+")
        .with_text_size(ADD_CONST_BUTTON_TEXT_SIZE)
        .padding(BUTTON_PADDING)
        .background(Painter::new(|ctx, data: &CalcState, env| {
            let width = ctx.size().width;
            let bounds = Circle::new(
                ((width / 2. - 1.), ctx.size().height / 2. + 2.),
                width / 2. - 1.,
            );
            let background_key = Key::<Color>::new(Box::leak(
                format!(
                    "calc.{:?}.{}.background",
                    data.get_theme(true),
                    BtnType::Function
                )
                .into_boxed_str(),
            ));
            if ctx.is_hot() {
                ctx.fill(bounds, &env.get(background_key));
            }
        }))
        .on_click(|_ctx, data: &mut CalcState, _env| {
            let consts = data.get_constants();
            let value: f64;
            let key: String;

            match validate_value(&consts.value_str) {
                Ok(val) => value = val,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }

            match validate_key(&consts.key_str) {
                Ok(k) => key = k,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }

            let is_added = data.add_constant(key, value);
            if is_added {
                data.constants.key_str.clear();
                data.constants.value_str.clear();
                _ctx.request_focus();
            } else {
                eprintln!("Constant already exist");
            }
        })
}

// Check if `value` is a number
fn validate_value(value: &String) -> Result<f64, String> {
    match value.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err(t!("errors.string_to_int_error")),
    }
}

// Check if `key` starts with an aphabetic character
fn validate_key(key: &String) -> Result<String, String> {
    if let Some(first_char) = key.chars().next() {
        match first_char.is_alphabetic() {
            true => Ok(key.to_string()),
            false => Err(format!(
                "Variable name must start with an alphabetic character"
            )),
        }
    } else {
        unreachable!()
    }
}

// Make a layout with radix tabs, numeric and operation buttons
fn make_num_part() -> impl Widget<CalcState> {
    let mut top_row = Flex::row();
    operation_button(&mut top_row, "←", Btn::MoveLeft);
    operation_button(&mut top_row, "→", Btn::MoveRight);
    operation_button(&mut top_row, "C", Btn::Clear);
    operation_button(&mut top_row, "⌫", Btn::Delete);

    let mut operations = Flex::column();
    operation_button(&mut operations, "÷", Btn::BinOpt(Opt::Div));
    operation_button(&mut operations, "⨯", Btn::BinOpt(Opt::Mul));
    operation_button(&mut operations, "+", Btn::BinOpt(Opt::Add));
    operation_button(&mut operations, "-", Btn::BinOpt(Opt::Sub));

    Flex::column()
        .with_flex_child(make_radix_tabs(), 1.)
        .with_spacer(TAB_BOTTOM_MARGIN)
        .with_flex_child(top_row, 1.)
        .with_flex_child(
            Flex::row()
                .with_flex_child(make_number_keyboard(), 3.)
                .with_flex_child(operations, 1.),
            4.,
        )
}

// Create numeric decimal keyboard
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

// Buttons in the function tab - Main
fn make_main_btns() -> impl Widget<CalcState> {
    let mut first_row = Flex::row();
    function_button(&mut first_row, "√", Btn::UnaryOpt(Opt::Sqrt));
    function_button(&mut first_row, "aⁿ", Btn::BinOpt(Opt::Pow));
    function_button(&mut first_row, "e", Btn::Const("e".to_owned()));

    let mut second_row = Flex::row();
    function_button(&mut second_row, "ⁿ√", Btn::BinOpt(Opt::Root));
    function_button(&mut second_row, "a²", Btn::UnaryOpt(Opt::Pow2));
    function_button(&mut second_row, "π", Btn::Const("π".to_owned()));

    let mut third_row = Flex::row();
    function_button(&mut third_row, "n!", Btn::UnaryOpt(Opt::Fact));
    function_button(&mut third_row, "|a|", Btn::UnaryOpt(Opt::Abs));
    function_button(&mut third_row, "³√", Btn::UnaryOpt(Opt::Root3));

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

// Buttons in the function tab - Func
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

// Add fucntion button to the `flex` widget
fn function_button(flex: &mut Flex<CalcState>, text: &str, button: Btn) {
    flex.add_flex_child(generic_button(text, button, BtnType::Function), 1.);
}

// Add operation button to the `flex` widget
fn operation_button(flex: &mut Flex<CalcState>, text: &str, button: Btn) {
    flex.add_flex_child(generic_button(text, button, BtnType::Operation), 1.);
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

// Represents tab for changing radix
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

// Tab that switchs between different function keyboards
fn function_tab(text: FunctionTabs) -> impl Widget<CalcState> {
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
            Label::new(format!("{:?}", text))
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

// Create a tab background painter
fn get_tab_painter() -> Painter<CalcState> {
    Painter::new(|ctx, _data: &CalcState, _env| {
        let size = ctx.size();
        let rectangle_height = TAB_UNDERLINE_SIZE;
        let bounds = Rect::new(0., size.height, size.width, size.height - rectangle_height)
            .to_rounded_rect(5.);

        if ctx.is_disabled() {
            ctx.fill(bounds, TAB_ACTIVE_COLOR);
        } else {
            // Highlight when hovering
            if ctx.is_hot() {
                ctx.fill(bounds, TAB_HOVER_COLOR);
            }
        }
    })
}

// Create a button background painter
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

            // Highlight when hovering
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
