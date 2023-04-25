use druid::{
    Command, DelegateCtx, Env, Event, Handled, KeyEvent, Selector, Size, Target, WindowConfig,
    WindowDesc,
};
use rust_i18n::t;

use crate::widgets::about::AboutWin;
use crate::widgets::help::HelpWin;
use crate::widgets::history_win::HistoryWin;
use crate::{CalcState, PressedButton};

use druid::AppDelegate;

const TEXTBOX_FOCUS: Selector<String> = Selector::new("textbox_focus");
const APP_FOCUS: Selector<String> = Selector::new("app_focus");
const SHOW_HISTORY: Selector<String> = Selector::new("show_history");
const SHOW_HELP: Selector<String> = Selector::new("show_help");
const SHOW_ABOUT: Selector<String> = Selector::new("show_about");

const HISTORY_WIN: Size = Size::new(300.0, 300.0);
const HELP_WIN: Size = Size::new(400.0, 400.0);
const ABOUT_WIN: Size = Size::new(300.0, 300.0);

/// Handle the menu bar commands for opening windows
pub struct Delegate;

impl AppDelegate<CalcState> for Delegate {
    #[rustfmt::skip]
    fn window_removed(&mut self, id: druid::WindowId, data: &mut CalcState, _env: &Env, _ctx: &mut DelegateCtx) {
        // close history before the window is removed
        match data.get_history().get_win_id() {
            Some(win_id) => {
                if *win_id == id {
                    data.get_mut_history().close_history();
                }
            }
            None => (),
        }
    }

    #[rustfmt::skip]
    fn command(&mut self, ctx: &mut DelegateCtx, _target: Target, cmd: &Command,data: &mut CalcState,_env: &Env) -> Handled {
        // Display History window
        if let Some(_) = cmd.get(SHOW_HISTORY) {
            if !data.get_history().is_opened() {
                let win_desc = WindowDesc::new(HistoryWin::build_ui())
                    .with_config(WindowConfig::default())
                    .resizable(false)
                    .title(t!("window.history"))
                    .window_size(HISTORY_WIN);

                data.get_mut_history().open_history(win_desc.id.clone());
                ctx.new_window(win_desc);
            }

            Handled::Yes

        // Display Help window
        } else if let Some(_) = cmd.get(SHOW_HELP) {
            let win_desc = WindowDesc::new(HelpWin::build_ui())
                .with_config(WindowConfig::default())
                .resizable(false)
                .title(t!("window.help"))
                .window_size(HELP_WIN);

            ctx.new_window(win_desc);

            Handled::Yes

        // Display About window
        } else if let Some(_) = cmd.get(SHOW_ABOUT) {
            let win_desc = WindowDesc::new(AboutWin::build_ui())
                .with_config(WindowConfig::default())
                .resizable(false)
                .title(t!("window.about"))
                .window_size(ABOUT_WIN);

            ctx.new_window(win_desc);
            Handled::Yes

        // Enable keyboard typing into the display
        } else if let Some(_) = cmd.get(APP_FOCUS) {
            data.set_display_focus(true);
            Handled::Yes

        // Disable keyboard typing into the display - focus is set on any textbox
        } else if let Some(_) = cmd.get(TEXTBOX_FOCUS) {
            data.set_display_focus(false);
            Handled::Yes
        } else {
            Handled::No
        }
    }

    #[rustfmt::skip]
    fn event(&mut self, _ctx: &mut DelegateCtx, win_id: druid::WindowId, event: Event, data: &mut CalcState, _env: &Env) -> Option<Event> {
        if data.get_main_win_id() == win_id {
            if let Event::KeyDown(key) = event.clone() {
                if data.get_display_focus() {
                    handle_keyboard_input(data, key);
                }
            }
        }
        Some(event)
    }
}

/// Handle user keyboard inputs
fn handle_keyboard_input(data: &mut CalcState, key: KeyEvent) {
    match &key.key {
        druid::keyboard_types::Key::ArrowLeft => data.process_button(&PressedButton::MoveLeft),
        druid::keyboard_types::Key::ArrowRight => data.process_button(&PressedButton::MoveRight),
        druid::keyboard_types::Key::Backspace => data.process_button(&PressedButton::Delete),
        druid::keyboard_types::Key::Clear => data.process_button(&PressedButton::Clear),
        druid::keyboard_types::Key::Character(ch) => {
            match ch.chars().next() {
                Some(val) => {
                    let ascii_value = val as u8;

                    // numbers
                    if ascii_value > 47 && ascii_value < 58 {
                        process_numeric_key(data, ascii_value - 48);
                        // A-F characters
                    } else if ascii_value > 96 && ascii_value < 103 {
                        process_numeric_key(data, ascii_value - 87);

                    // Dot and comma
                    } else if ascii_value == 44 || ascii_value == 46 {
                        data.process_button(&PressedButton::Comma)

                    // Left bracket
                    } else if ascii_value == 40 {
                        data.process_button(&PressedButton::BracketLeft)

                    // Right bracket
                    } else if ascii_value == 41 {
                        data.process_button(&PressedButton::BracketRight)
                    }
                }
                None => (),
            }
        }
        _ => (),
    }
}

/// Allow key processing only if the number is valid for the set radix
fn process_numeric_key(data: &mut CalcState, num: u8) {
    match data.radix {
        math::number::Radix::Bin => {
            if num < 2 {
                data.process_button(&PressedButton::Num(num))
            }
        }
        math::number::Radix::Oct => {
            if num < 8 {
                data.process_button(&PressedButton::Num(num))
            }
        }
        math::number::Radix::Dec => {
            if num < 10 {
                data.process_button(&PressedButton::Num(num))
            }
        }
        _ => data.process_button(&PressedButton::Num(num)),
    }
}
