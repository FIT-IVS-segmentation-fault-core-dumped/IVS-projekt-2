mod environment;
use calc::delegate::Delegate;

use calc::widgets::menu::CalcMenu;
use calc::{
    widgets::{buttons_ui::ButtonsUI, display::DisplayUI},
    CalcState, Theme,
};
use druid::widget::Controller;
use druid::{
    theme,
    widget::{Container, EnvScope, Flex},
    AppLauncher, Size, Widget, WindowDesc,
};
use druid::{Env, Event, EventCtx, WidgetExt};
use environment::*;

/// Initial size of the window, when the app starts.
const WINDOW_SIZE: Size = Size::new(400.0, 400.0);
const MIN_WINDOW_SIZE: Size = Size::new(400.0, 400.0);

const APP_NAME: &str = "FIT calc";

/// After each click set focus on the whole app container in order to handle user keyboard events.
struct AppFocusController;

impl<W: Widget<CalcState>> Controller<CalcState, W> for AppFocusController {
    #[rustfmt::skip]
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut CalcState, env: &Env) {
        if let Event::MouseDown(_) = event {
            ctx.set_focus(ctx.widget_id());
        }
        child.event(ctx, event, data, env)
    }
}

/// Creates the root widget of app. All other widgets are inside this one.
fn build_root_widget() -> impl Widget<CalcState> {
    EnvScope::new(
        |env, data| match data.get_theme(true) {
            Theme::Dark => set_dark_envs(env),
            Theme::Light => set_light_envs(env),
            Theme::System => unreachable!(),
        },
        Container::new(
            Flex::column()
                .with_flex_child(DisplayUI::build_ui(), 1.0)
                .with_flex_child(ButtonsUI::build_ui(), 3.0),
        )
        .background(theme::WINDOW_BACKGROUND_COLOR)
        .controller(AppFocusController),
    )
}

fn main() {
    // Load stored calc state.
    let mut calc_state = CalcState::new(calc::available_locales());
    // Set initial locale from config.
    rust_i18n::set_locale(calc_state.language());

    // Create the main window with given window parameters.
    let main_window = WindowDesc::new(build_root_widget())
        .title(APP_NAME)
        .window_size(WINDOW_SIZE)
        .with_min_size(MIN_WINDOW_SIZE)
        .menu(CalcMenu::build_ui);

    // Launch the main app using calc_state to define behaviour.
    calc_state.set_main_win_id(main_window.id);
    if let Err(platform_err) = AppLauncher::with_window(main_window)
        .configure_env(|env, data| {
            if data.get_theme(true) == Theme::Dark {
                set_dark_envs(env);
            } else {
                set_light_envs(env);
            };

            set_digit_btn_envs(env);
            set_func_btn_envs(env);
            set_operation_btn_envs(env);
        })
        .delegate(Delegate)
        .launch(calc_state)
    {
        eprintln!("error: Failed to launch main application. {}", platform_err);
        std::process::exit(1);
    }
}
