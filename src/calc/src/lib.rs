//! Calculator application
//!
//! This file contains definition of calculator state,
//! which defines functionality of our app.

pub mod widgets;
use druid::{Data, Lens};
use math::number::Radix;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

const APP_NAME: &str = "Calculator";

/// Operations on the calculator.
#[rustfmt::skip]
pub enum Opt {
    Sum, Sub, Mul, Div,
    Sin, Cos, Tg, Cotg,
    Arcsin, Arccos, Arctg, Arccotg,
    Log, LogN, Ln, Sqrt, Root, Pow,
    Abs, Comb, Fact
}

/// Used to map button presses to functionality.
/// Now if we want to implement alternative ways
/// of using our calculator, we just need to
/// pass this enum as action to the pressed button.
pub enum PressedButton {
    /// Numpad 0-9 or A-F (10 - 15)
    Num(u8),
    BinOpt(Opt),
    UnaryOpt(Opt),
    /// Constant with given name.
    /// Name is an entry to the table of constants, which will
    /// contain its value. This gives us the ability, to define
    /// their own constants (variables).
    Const(String),
    /// Clear the entire display.
    Clear,
    /// Delete only the item before cursor.
    Delete,
    /// Move cursor right.
    MoveRight,
    /// Move cursor left.
    MoveLeft,
    /// Compute and display result.
    Evaluate,
    BracketLeft,
    BracketRight,
    Comma,
    Ans,
}

/// Color theme of the app.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Data, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

/// Holds application configuration, which is saved on the disk.
/// This is loaded at each start of the application.
#[derive(Serialize, Deserialize, Lens, Clone, Data)]
pub struct CalcConfig {
    theme: Theme,
    language: String,
}

/// Defines what will the initial config hold (when no config is found on the disk).
impl Default for CalcConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "en".to_owned(),
        }
    }
}

/// Holds shared data for all the widgets in our application.
/// Each widget has access to instance of this struct.
#[derive(Clone, Lens)]
pub struct CalcState {
    /// Holds text, which is used to display the final string on the display widget.
    /// or converted to evaluate string for the math library.
    #[lens(ignore)]
    inner_expr: String,
    /// Displayed base of the computed result.
    radix: Radix,
    /// Contains all available languages at runtime.
    /// This is loaded from rust-i18n and as such has to be constructed in new() method.
    available_languages: Rc<Vec<String>>,
    /// Confing deserialized from disk using *confy* crate.
    config: CalcConfig,
    /// Parser of mathematical expressions.
    #[lens(ignore)]
    calc: Rc<math::Calculator>,
    /// Last computed result, which is displayed on the display.
    #[lens(ignore)]
    result: String,
}

/// Contains dummy structs for custom druid::Lens implementations.
pub mod calcstate_lenses {
    #[allow(non_camel_case_types)]
    pub struct inner_expr;
    #[allow(non_camel_case_types)]
    pub struct result;
}

/// Lens will convert inner_expr to displayed_string.
impl Lens<CalcState, String> for calcstate_lenses::inner_expr {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &CalcState, f: F) -> V {
        f(&data.get_display_str())
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut CalcState, f: F) -> V {
        f(&mut data.inner_expr)
    }
}

/// In the future, if we want to compute result realtime
/// when typing, then we could do it here.
impl Lens<CalcState, String> for calcstate_lenses::result {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &CalcState, f: F) -> V {
        f(&data.result)
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut CalcState, f: F) -> V {
        f(&mut data.result)
    }
}

impl Data for CalcState {
    fn same(&self, other: &Self) -> bool {
        self.inner_expr == other.inner_expr
            && self.radix == other.radix
            && self.config.same(&other.config)
    }
}

impl CalcState {
    #[allow(non_upper_case_globals)]
    pub const displayed_text: calcstate_lenses::inner_expr = calcstate_lenses::inner_expr;
    #[allow(non_upper_case_globals)]
    pub const result: calcstate_lenses::result = calcstate_lenses::result;

    /// Creates new instance of CalcState. This will load config from disk.
    ///
    /// * `languages` - Array of available languages loaded from rust-i18n.
    pub fn new(languages: &[&str]) -> Self {
        let config = confy::load(APP_NAME, None).unwrap_or_default();

        Self {
            inner_expr: String::from("0"),
            radix: Radix::Dec,
            // Convert array of string slices to vector of strings.
            available_languages: Rc::new(languages.iter().map(|&s| String::from(s)).collect()),
            config,
            calc: Rc::new(math::Calculator::new()),
            result: String::new()
        }
    }

    /// Readonly getter for language.
    pub fn language(&self) -> &String {
        &self.config.language
    }

    /// Handle button event. We will construct the displayed string using this method.
    pub fn process_button(&self, _button: PressedButton) {
        todo!();
    }

    /// Convert CalcState::inner_expr to *evaluate string*, which
    /// can then be passed to the `math::evaluate` function.
    pub fn get_eval_str(&self) -> String {
        todo!()
    }

    /// Convert CalcState::inner_expr to *display string*, which will 
    /// be actually displayed on the calculator display.
    pub fn get_display_str(&self) -> String {
        // TODO: replace functions with their special characters.
        self.inner_expr.clone()
    }

    /// Store CalcState::config on the disk using *confy* create.
    pub fn store_config_data(&self) -> Result<(), confy::ConfyError> {
        confy::store(APP_NAME, None, &self.config)
    }

    /// Get currently active theme.
    pub fn get_theme(&self) -> Theme {
        self.config.theme
    }

    /// Change theme of application. This will be saved at exit.
    pub fn set_theme(&mut self, theme: Theme) {
        self.config.theme = theme;
    }

    /// Change language of the app. This will be saved at exit.
    ///
    /// * `language` - Must be in the array of languages passed to `CalcState::new()`
    pub fn set_language(&mut self, language: &str) {
        assert!(self.available_languages.contains(&language.to_string()));
        rust_i18n::set_locale(language);
        self.config.language = String::from(language);
    }

    /// Change numeric base of the calculated results.
    pub fn set_radix(&mut self, radix: Radix) {
        self.radix = radix;
    }
}

#[cfg(test)]
mod calcstate_tests;
