//! The menu UI part of the calculator

use std::rc::Rc;

use druid::{Env, Menu, MenuItem, WindowId};
use rust_i18n::*;

use crate::{CalcState, Theme};

pub struct CalcMenu;

impl CalcMenu {
    pub fn build_ui(_window: Option<WindowId>, _data: &CalcState, _env: &Env) -> Menu<CalcState> {
        make_menu(_window, _data, _env)
    }
}

/// Creates menu on top of the main window.
fn make_menu(_window: Option<WindowId>, _data: &CalcState, _env: &Env) -> Menu<CalcState> {
    Menu::empty()
        .entry(
            Menu::new(t!("menu.file"))
                .entry(
                    Menu::new(t!("theme"))
                        .entry(make_theme_button(Theme::Dark))
                        .entry(make_theme_button(Theme::Light))
                        .entry(make_theme_button(Theme::System)),
                )
                .entry(
                    Menu::new(t!("language"))
                        .entry(make_language_button("cz".to_owned()))
                        .entry(make_language_button("en".to_owned()))
                        .entry(make_language_button("jp".to_owned()))
                        .entry(make_language_button("sk".to_owned()))
                        .entry(make_language_button("vi".to_owned())),
                ),
        )
        .entry(MenuItem::new(t!("menu.edit")))
        .entry(MenuItem::new(t!("menu.help")))
}

fn make_theme_button(theme: Theme) -> MenuItem<CalcState> {
    MenuItem::new(t!(&theme.to_string().to_lowercase()))
        .selected_if(move |data: &CalcState, _env| data.get_theme(false) == theme)
        .on_activate(move |_ctx, data: &mut CalcState, _env| data.set_theme(theme))
}

fn make_language_button(lang: String) -> MenuItem<CalcState> {
    let lang_rc = Rc::new(lang);
    let lang_clone = lang_rc.clone();
    MenuItem::new(t!(lang_rc.as_str()))
        .on_activate(move |_ctx, data: &mut CalcState, _env| data.set_language(&lang_clone))
        .selected_if(move |data, _env| data.get_language() == lang_rc.as_ref())
}
