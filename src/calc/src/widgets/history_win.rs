use std::time::Duration;

use druid::{
    theme,
    widget::{
        Container, Controller, EnvScope, Flex, Label, Padding, Painter, Scroll, ViewSwitcher,
    },
    Color, Env, Event, EventCtx, Key, RenderContext, RoundedRectRadii, TimerToken, Widget,
    WidgetExt,
};
use rust_i18n::t;

use crate::{
    environment::{set_dark_envs, set_light_envs},
    CalcState,
};

const EXPRESSION_LIST_HEIGHT: f64 = 200.;
const PADDING: f64 = 10.;
const CLEAR_BUTTON_HEIGHT: f64 = 25.;
const CLEAR_BUTTON_WIDTH: f64 = 80.;
const CLEAR_BUTTON_PADDING: f64 = 20.;
const CLEAR_BUTTON_TEXT_COLOR: Key<Color> = Key::new("clear_button_color");

// Represents UI of the history window
pub struct HistoryWin;

impl HistoryWin {
    pub fn build_ui() -> impl Widget<CalcState> {
        EnvScope::new(
            |env, data: &CalcState| match data.get_theme(true) {
                crate::Theme::Dark => set_dark_envs(env),
                crate::Theme::Light => set_light_envs(env),
                crate::Theme::System => unreachable!(),
            },
            Container::new(Padding::new(
                PADDING,
                Flex::column()
                    .with_child(build_history())
                    .with_spacer(PADDING)
                    .with_child(make_clear_btn())
                    .with_spacer(PADDING),
            ))
            .background(theme::WINDOW_BACKGROUND_COLOR),
        )
    }
}

// Render all notes from history
fn build_history() -> impl Widget<CalcState> {
    Scroll::new(ViewSwitcher::new(
        |data: &CalcState, _env| data.get_history().get_data().len(),
        |_selector, _data: &CalcState, _env| {
            let mut column = Flex::<CalcState>::column();
            for (expr, res) in _data.get_history().get_data().iter().rev() {
                column.add_child(make_equation(expr, res));
            }

            Box::new(column)
        },
    ))
    .vertical()
    .disable_scrollbars()
    .fix_height(EXPRESSION_LIST_HEIGHT)
}

// Represets one note in the whole history
fn make_equation(expr: &str, res: &str) -> impl Widget<CalcState> {
    EnvScope::new(
        |env, _data| {
            env.set(druid::theme::SCROLLBAR_PAD, 5.);
            env.set(druid::theme::SCROLLBAR_WIDTH, 20.);
            env.set(druid::theme::SCROLLBAR_MIN_SIZE, 240.);
            env.set(druid::theme::SCROLLBAR_MAX_OPACITY, 0.);
            env.set(druid::theme::SCROLLBAR_COLOR, Color::TRANSPARENT);
        },
        Scroll::new(
            Label::new(format!("{} = {}", expr, res))
                .with_text_size(18.)
                .with_text_alignment(druid::TextAlignment::Start),
        )
        .horizontal(),
    )
}

fn make_clear_btn() -> impl Widget<CalcState> {
    #[cfg(target_os = "linux")]
    let label = Label::new(t!("clear"))
        .with_text_color(CLEAR_BUTTON_TEXT_COLOR)
        .with_text_alignment(druid::TextAlignment::Center)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        .center()
        .background(get_btn_painter())
        .controller(ConfirmController::new())
        .on_click(|_, _, _| {})
        .fix_size(CLEAR_BUTTON_WIDTH, CLEAR_BUTTON_HEIGHT);

    #[cfg(target_os = "windows")]
    let label = Label::new(t!("clear"))
        .with_text_color(CLEAR_BUTTON_TEXT_COLOR)
        .with_text_alignment(druid::TextAlignment::Center)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        .background(get_btn_painter())
        .controller(ConfirmController::new())
        .on_click(|_, _, _| {})
        .fix_size(CLEAR_BUTTON_WIDTH, CLEAR_BUTTON_HEIGHT);

    EnvScope::new(
        |env, data: &CalcState| match data.get_history().confiming_deletition {
            true => env.set(CLEAR_BUTTON_TEXT_COLOR, Color::RED),
            false => env.set(CLEAR_BUTTON_TEXT_COLOR, env.get(druid::theme::TEXT_COLOR)),
        },
        label,
    )
}

enum ConfirmControllerState {
    Idle,
    Waiting(TimerToken),
}

// Clear all history data when clear button is pressed two times in 1 second
struct ConfirmController {
    state: ConfirmControllerState,
}

impl ConfirmController {
    fn new() -> Self {
        Self {
            state: ConfirmControllerState::Idle,
        }
    }
}

impl<W: Widget<CalcState>> Controller<CalcState, W> for ConfirmController {
    fn event(
        &mut self,
        _child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut CalcState,
        _env: &Env,
    ) {
        match self.state {
            ConfirmControllerState::Idle => {
                // Change state to Waiting after button was pressed and start the timer
                if let Event::MouseDown(_) = event {
                    data.get_mut_history().confiming_deletition = true;
                    let token = ctx.request_timer(Duration::from_secs(1));
                    self.state = ConfirmControllerState::Waiting(token);
                    ctx.request_update();
                }
            }

            ConfirmControllerState::Waiting(token) => {
                // When time has expires, reset state to Idle
                if let Event::Timer(tok) = event {
                    if tok == &token {
                        self.state = ConfirmControllerState::Idle;
                        data.get_mut_history().confiming_deletition = false;
                        ctx.request_update();
                        ctx.set_handled();
                    }
                }

                // Second button press in time interval of 1 second => remove history
                if let Event::MouseDown(_) = event {
                    self.state = ConfirmControllerState::Idle;
                    let mut history = data.get_mut_history();
                    history.confiming_deletition = false;
                    history.clear();
                    data.store_config_data();
                    ctx.request_update();
                }
            }
        }
    }
}

// Button background color painter
fn get_btn_painter() -> Painter<CalcState> {
    Painter::new(|ctx, _data: &CalcState, env| {
        let bound = ctx
            .size()
            .to_rounded_rect(RoundedRectRadii::from_single_radius(CLEAR_BUTTON_PADDING));

        let theme = _data.get_theme(true);
        let background_key = Key::<Color>::new(Box::leak(
            format!("calc.{:?}.operation_btn.background", theme).into_boxed_str(),
        ));

        let hot_key = Key::<Color>::new(Box::leak(
            format!("calc.{:?}.operation_btn.hover", theme).into_boxed_str(),
        ));
        let active_key = Key::<Color>::new(Box::leak(
            format!("calc.{:?}.operation_btn.active", theme).into_boxed_str(),
        ));
        ctx.fill(bound, &env.get(background_key));

        if ctx.is_hot() {
            ctx.fill(bound, &env.get(hot_key));
        }
        if ctx.is_active() {
            ctx.fill(bound, &env.get(active_key));
        }
    })
}
