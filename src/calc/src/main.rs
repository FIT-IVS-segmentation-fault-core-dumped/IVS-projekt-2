use calc::CalcState;
use druid::{
    widget::Flex, AppLauncher, Env, Menu, MenuItem, Size, Widget, WindowDesc,
    WindowId,
};
use rust_i18n::t;

const WINDOW_SIZE: Size = Size::new(400.0, 400.0);
const MIN_WINDOW_SIZE: Size = Size::new(400.0, 400.0);

// Initialize locales in "locales" directory.
rust_i18n::i18n!("locales");

fn build_root_widget() -> impl Widget<CalcState> {
    Flex::column()
}

fn make_menu(_window: Option<WindowId>, _data: &CalcState, _env: &Env) -> Menu<CalcState> {
    Menu::empty()
        .entry(MenuItem::new(t!("menu.file")))
        .entry(MenuItem::new(t!("menu.edit")))
        .entry(MenuItem::new(t!("menu.help")))
}

fn main() {
    // Load stored calc state.
    let calc_state = CalcState::new(available_locales());
    // Set initial locale from config.
    rust_i18n::set_locale(&calc_state.config.language);

    let main_window = WindowDesc::new(build_root_widget())
        .title(t!("title"))
        .window_size(WINDOW_SIZE)
        .with_min_size(MIN_WINDOW_SIZE)
        .menu(|window, data, env| make_menu(window, data, env));

    AppLauncher::with_window(main_window)
        .launch(calc_state)
        .expect("Failed to launch application");
}
