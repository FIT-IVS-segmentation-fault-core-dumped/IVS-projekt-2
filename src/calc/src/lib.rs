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

#[derive(Serialize, Deserialize, Lens, Clone, Data)]
pub struct CalcConfig {
    theme: Theme,
    pub language: String,
}

impl Default for CalcConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "en".to_owned(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct CalcState {
    displayed_text: String,
    radix: Rc<Radix>,
    available_languages: Rc<Vec<String>>,
    pub config: CalcConfig,
}

impl CalcState {
    pub fn new(languages: &'static [&'static str]) -> Self {
        let config = confy::load(APP_NAME, None).unwrap_or_default();

        Self {
            displayed_text: String::new(),
            radix: Rc::new(Radix::Dec),
            available_languages: Rc::new(languages.iter().map(|&s| String::from(s)).collect()),
            config,
        }
    }

    pub fn process_button(&self, _button: PressedButton) {
        todo!();
    }

    pub fn get_eval_str(&self) -> String {
        todo!()
    }

    pub fn store_config_data(&self) {
        confy::store(APP_NAME, None, &self.config).unwrap();
    }

    pub fn get_theme(&self) -> Theme {
        self.config.theme
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.config.theme = theme;
    }

    pub fn set_language(&mut self, language: &str) {
        assert!(self.available_languages.contains(&language.to_string()));
        rust_i18n::set_locale(language);
        self.config.language = String::from(language);
    }

    pub fn set_radix(&mut self, radix: Radix) {
        self.radix = Rc::new(radix);
    }
}
