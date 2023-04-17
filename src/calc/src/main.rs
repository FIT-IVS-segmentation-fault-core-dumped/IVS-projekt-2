mod environment;

use calc::translate;
use calc::widgets::menu::CalcMenu;
use calc::{widgets::ButtonsUI, CalcState, Theme};
use druid::{
    theme,
    widget::{Container, EnvScope, Flex},
    AppLauncher, Size, Widget, WindowDesc,
};
use environment::*;
use rust_i18n::t;

/// Initial size of the window, when the app starts.
const WINDOW_SIZE: Size = Size::new(400.0, 400.0);
const MIN_WINDOW_SIZE: Size = Size::new(400.0, 400.0);

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
                .main_axis_alignment(druid::widget::MainAxisAlignment::End)
                .with_flex_child(Flex::column(), 1.0)
                // .with_spacer(10.0)
                .with_flex_child(ButtonsUI::build_ui(), 3.0),
        )
        .background(theme::WINDOW_BACKGROUND_COLOR),
    )
}

fn main() {
    // Load stored calc state.
    let calc_state = CalcState::new(calc::available_locales());
    // Set initial locale from config.
    rust_i18n::set_locale(calc_state.language());

    // Create the main window with given window parameters.
    let main_window = WindowDesc::new(build_root_widget())
        .title(t!("title"))
        .window_size(WINDOW_SIZE)
        .with_min_size(MIN_WINDOW_SIZE)
        .menu(CalcMenu::build_ui);

    // Launch the main app using calc_state to define behaviour.
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
        .launch(calc_state)
    {
        eprintln!("error: Failed to launch main application. {}", platform_err);
        std::process::exit(1);
    }
}
