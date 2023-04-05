use std::rc::Rc;

use druid::{Data, Lens};
use math::number::Radix;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "Calculator";

#[rustfmt::skip]
pub enum Opt {
    Sum, Sub, Mul, Div,
    Sin, Cos, Tg, Cotg,
    Arcsin, Arccos, Arctg, Arccotg,
    Log, LogN, Sqrt, Root, Pow,
    Abs, Comb, Perm, Ln
}

pub enum PressedButton {
    Num(u8),
    BinOpt(Opt),
    Const(String),
    Clear,
    Delete,
    MoveRight,
    MoveLeft,
    Evaluate,
    BracketLeft,
    BracketRight,
    Comma,
    Ans,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Data, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Serialize, Deserialize, Lens, Copy, Clone, Data)]
pub struct CalcConfig {
    theme: Theme,
}

impl Default for CalcConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct CalcState {
    displayed_text: String,
    radix: Rc<Radix>,
    config: CalcConfig,
}

impl CalcState {
    pub fn new() -> Self {
        let config = confy::load(APP_NAME, None).unwrap_or_default();

        Self {
            displayed_text: String::new(),
            radix: Rc::new(Radix::Dec),
            config,
        }
    }

    pub fn process_button(&self, _button: PressedButton) {
        todo!();
    }

    pub fn store_config_data(&self) {
        confy::store(APP_NAME, None, self.config).unwrap();
    }

    pub fn get_theme(&self) -> Theme {
        self.config.theme
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.config.theme = theme;
    }

    pub fn set_radix(&mut self, radix: Radix) {
        self.radix = Rc::new(radix);
    }
}
