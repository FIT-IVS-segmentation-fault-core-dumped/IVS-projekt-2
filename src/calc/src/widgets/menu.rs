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
                        .entry(
                            MenuItem::new(t!("system"))
                                .selected_if(|data: &CalcState, _env| {
                                    data.get_theme(false) == Theme::System
                                })
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_theme(Theme::System)
                                }),
                        )
                        .entry(
                            MenuItem::new(t!("dark"))
                                .selected_if(|data: &CalcState, _env| {
                                    data.get_theme(false) == Theme::Dark
                                })
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_theme(Theme::Dark)
                                }),
                        )
                        .entry(
                            MenuItem::new(t!("light"))
                                .selected_if(|data: &CalcState, _env| {
                                    data.get_theme(false) == Theme::Light
                                })
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_theme(Theme::Light)
                                }),
                        ),
                )
                .entry(
                    Menu::new(t!("language"))
                        .entry(
                            MenuItem::new(t!("czech"))
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_language("cz");
                                })
                                .selected_if(|data, _env| data.get_language() == "cz"),
                        )
                        .entry(
                            MenuItem::new(t!("english"))
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_language("en")
                                })
                                .selected_if(|data, _env| data.get_language() == "en"),
                        )
                        .entry(
                            MenuItem::new(t!("japanese"))
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_language("jp")
                                })
                                .selected_if(|data, _env| data.get_language() == "jp"),
                        )
                        .entry(
                            MenuItem::new(t!("slovak"))
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_language("sk");
                                })
                                .selected_if(|data, _env| data.get_language() == "sk"),
                        )
                        .entry(
                            MenuItem::new(t!("vietnamese"))
                                .on_activate(|_ctx, data: &mut CalcState, _env| {
                                    data.set_language("vi")
                                })
                                .selected_if(|data, _env| data.get_language() == "vi"),
                        ),
                )
                .entry(MenuItem::new("constants")),
        )
        .entry(MenuItem::new(t!("menu.edit")))
        .entry(MenuItem::new(t!("menu.help")))
}
