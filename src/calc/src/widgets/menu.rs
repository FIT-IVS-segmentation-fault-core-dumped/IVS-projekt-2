//! The menu UI part of the calculator

use std::rc::Rc;

use druid::{Command, Env, Menu, MenuItem, Selector, Target, WindowId};
use rust_i18n::*;

use crate::{CalcState, Theme};

pub struct CalcMenu;

const SHOW_HISTORY: Selector<String> = Selector::new("show_history");
const SHOW_HELP: Selector<String> = Selector::new("show_help");
const SHOW_ABOUT: Selector<String> = Selector::new("show_about");

impl CalcMenu {
    pub fn build_ui(_window: Option<WindowId>, _data: &CalcState, _env: &Env) -> Menu<CalcState> {
        make_menu(_window, _data, _env)
    }
}

/// Creates menu on top of the main window.
fn make_menu(_window: Option<WindowId>, _data: &CalcState, _env: &Env) -> Menu<CalcState> {
    Menu::empty()
        .rebuild_on(|old_data: &CalcState, data, _env| {
            old_data.get_language() != data.get_language()
        })
        .entry(
            Menu::new(t!("menu.options"))
                .entry(
                    Menu::new(t!("options.theme"))
                        .entry(make_theme_button(Theme::Dark))
                        .entry(make_theme_button(Theme::Light))
                        .entry(make_theme_button(Theme::System)),
                )
                .entry(
                    Menu::new(t!("options.language"))
                        .entry(make_language_button("cz".to_owned()))
                        .entry(make_language_button("en".to_owned()))
                        .entry(make_language_button("jp".to_owned()))
                        .entry(make_language_button("sk".to_owned()))
                        .entry(make_language_button("vi".to_owned())),
                )
                .entry(
                    Menu::new(t!("options.angular_unit"))
                        .entry(make_angular_unit_button("radians", false))
                        .entry(make_angular_unit_button("degrees", true)),
                )
                .entry(
                    MenuItem::new(t!("options.record_hist"))
                        .on_activate(|_ctx, data: &mut CalcState, _env| {
                            data.config.history.toggle_recording();
                            data.store_config_data();
                        })
                        .selected_if(|data, _env| data.get_history().recording() == true),
                )
                .entry(MenuItem::new(t!("options.show_hist")).command(Command::new(
                    SHOW_HISTORY,
                    "".to_owned(),
                    Target::Auto,
                ))),
        )
        .entry(
            Menu::new(t!("menu.edit"))
                .entry(MenuItem::new(t!("edit.copy_res")).on_activate(
                    |_ctx, data: &mut CalcState, _env| {
                        data.copy_result();
                    },
                ))
                .entry(MenuItem::new(t!("edit.copy_expr")).on_activate(
                    |_ctx, data: &mut CalcState, _env| {
                        data.copy_expression();
                    },
                )),
        )
        .entry(MenuItem::new(t!("menu.help")).command(Command::new(
            SHOW_HELP,
            "".to_owned(),
            Target::Global,
        )))
        .entry(MenuItem::new(t!("menu.about")).command(Command::new(
            SHOW_ABOUT,
            "".to_owned(),
            Target::Global,
        )))
}

fn make_angular_unit_button(name: &str, is_degree: bool) -> MenuItem<CalcState> {
    let text = format!("angular_units.{}", name);
    MenuItem::new(t!(&text))
        .on_activate(move |_ctx, data: &mut CalcState, _env| {
            data.set_angular_unit(is_degree);
        })
        .selected_if(move |data, _env| data.get_angular_unit() == is_degree)
}

fn make_theme_button(theme: Theme) -> MenuItem<CalcState> {
    let theme_name = format!("themes.{}", theme.to_string().to_lowercase());
    MenuItem::new(t!(&theme_name))
        .selected_if(move |data: &CalcState, _env| data.get_theme(false) == theme)
        .on_activate(move |_ctx, data: &mut CalcState, _env| data.set_theme(theme))
}

fn make_language_button(lang: String) -> MenuItem<CalcState> {
    let lang_rc = Rc::new(lang);
    let lang_clone = lang_rc.clone();
    MenuItem::new(t!("locale_name", locale = lang_rc.as_str()))
        .on_activate(move |_ctx, data: &mut CalcState, _env| data.set_language(&lang_clone))
        .selected_if(move |data, _env| data.get_language() == lang_rc.as_ref())
}
