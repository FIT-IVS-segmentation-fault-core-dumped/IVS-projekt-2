use calc::CalcState;
use druid::{
    widget::Flex, AppLauncher, Env, LocalizedString, Menu, MenuItem, Size, Widget, WindowDesc,
    WindowId,
};

const WINDOW_SIZE: Size = Size::new(400.0, 400.0);
const MIN_WINDOW_SIZE: Size = Size::new(400.0, 400.0);

fn build_root_widget() -> impl Widget<CalcState> {
    Flex::column()
}

fn make_menu(_window: Option<WindowId>, _data: &CalcState, _env: &Env) -> Menu<CalcState> {
    Menu::empty()
        .entry(MenuItem::new(LocalizedString::new("File")))
        .entry(MenuItem::new(LocalizedString::new("Edit")))
        .entry(MenuItem::new(LocalizedString::new("Help")))
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title(LocalizedString::new("Title"))
        .window_size(WINDOW_SIZE)
        .with_min_size(MIN_WINDOW_SIZE)
        .menu(|window, data, env| make_menu(window, data, env));

    AppLauncher::with_window(main_window)
        .launch(CalcState::new())
        .expect("Failed to launch application");
}
