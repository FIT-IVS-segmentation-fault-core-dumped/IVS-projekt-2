//! Calculator application
//!
//! This file contains definition of calculator state,
//! which defines functionality of our app.
pub mod widgets;
pub mod expr_manager;
use druid::{Data, Lens};
use expr_manager::ExprManager;
use math::number::Radix;
use serde::{Deserialize, Serialize};
use std::{fmt, rc::Rc, cell::RefCell};

const APP_NAME: &str = "Calculator";

// Initialize locales in "locales" directory.
rust_i18n::i18n!("locales");

/// Operations on the calculator.
#[rustfmt::skip]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Opt {
    Add, Sub, Mul, Div,
    Sin, Cos, Tg, Cotg,
    Arcsin, Arccos, Arctg, Arccotg,
    Log, LogN, Ln, Sqrt, Root, Pow, Pow2, Root3,
    Abs, Comb, Fact, Mod,
    Random
}


/// Used to map button presses to functionality.
/// Now if we want to implement alternative ways
/// of using our calculator, we just need to
/// pass this enum as action to the pressed button.
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FuncPad {
    Main,
    Func,
    // Const,
}

impl fmt::Display for FuncPad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
    expr_man: ExprManager,
    /// Displayed base of the computed result.
    radix: Radix,

    function_pad: FuncPad,
    /// Contains all available languages at runtime.
    /// This is loaded from rust-i18n and as such has to be constructed in new() method.
    available_languages: Rc<Vec<String>>,

    /// Confing deserialized from disk using *confy* crate.
    config: CalcConfig,
    /// Parser of mathematical expressions.
    #[lens(ignore)]
    calc: Rc<RefCell<math::Calculator>>,
    /// Last computed result, which is displayed on the display.
    #[lens(ignore)]
    result: String,
    /// Is `true` if `CalcState::result` is an error message.
    result_is_err: bool,
    /// Use degrees in trigonometric computations, otherwise use radians.
    degrees: bool,
}

/// Contains dummy structs for custom druid::Lens implementations.
pub mod calcstate_lenses {
    #[allow(non_camel_case_types)]
    pub struct displayed_text;
    #[allow(non_camel_case_types)]
    pub struct all;
}

impl Lens<CalcState, String> for calcstate_lenses::displayed_text {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &CalcState, f: F) -> V {
        f(&data.expr_man.get_display_str())
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut CalcState, f: F) -> V {
        let mut dis = data.expr_man.get_display_str();
        f(&mut dis)
    }
}

impl Lens<CalcState, CalcState> for calcstate_lenses::all {
    fn with<V, F: FnOnce(&CalcState) -> V>(&self, data: &CalcState, f: F) -> V {
        f(data)
    }
    fn with_mut<V, F: FnOnce(&mut CalcState) -> V>(&self, data: &mut CalcState, f: F) -> V {
        f(data)
    }
}

impl Data for CalcState {
    fn same(&self, other: &Self) -> bool {
        self.expr_man.same(&other.expr_man)
            && self.radix == other.radix
            && self.function_pad == other.function_pad
            && self.config.same(&other.config)
            && self.result == other.result
    }
}

impl CalcState {
    #[allow(non_upper_case_globals)]
    pub const displayed_text: calcstate_lenses::displayed_text = calcstate_lenses::displayed_text;
    #[allow(non_upper_case_globals)]
    pub const all: calcstate_lenses::all = calcstate_lenses::all;

    /// Creates new instance of CalcState. This will load config from disk.
    ///
    /// * `languages` - Array of available languages loaded from rust-i18n.
    pub fn new(languages: &[&str]) -> Self {
        let config = confy::load(APP_NAME, None).unwrap_or_default();

        Self {
            expr_man: ExprManager::new(),
            radix: Radix::Dec,
            function_pad: FuncPad::Main,
            // Convert array of string slices to vector of strings.
            available_languages: Rc::new(languages.iter().map(|&s| String::from(s)).collect()),
            config,
            calc: Rc::new(RefCell::new(math::Calculator::new())),
            result: String::new(),
            result_is_err: false,
            degrees: true,
        }
    }

    /// Readonly getter for language.
    pub fn language(&self) -> &String {
        &self.config.language
    }

    /// Handle button event from the UI.
    pub fn process_button(&mut self, button: &PressedButton) {
        match button {
            PressedButton::Evaluate => {
                // Compute result from evaluate string
                let result = self.calc.borrow_mut().evaluate(&self.expr_man.get_eval_str());

                // Set resulting variable according to the resulting value.
                (self.result, self.result_is_err) = match result {
                    Err(e) => (format!("{:?}", e), true),
                    Ok(num) => (num.to_string(self.radix, 5), false)
                };
            },
            PressedButton::Clear => {
                self.expr_man.process_button(button);
                if self.result_is_err {
                    self.result.clear();
                    self.result_is_err = false;
                }
            }
            // Relay other buttons to the expression manager.
            other => self.expr_man.process_button(other)
        };
    }

    /// Convert CalcState::inner_expr to *evaluate string*, which
    /// can then be passed to the `math::evaluate` function.
    pub fn get_eval_str(&self) -> String {
        self.expr_man.get_eval_str()
    }

    /// Convert CalcState::inner_expr to *display string*, which will 
    /// be actually displayed on the calculator display.
    pub fn get_display_str(&self) -> String {
        self.expr_man.get_display_str()
    }

    /// Store CalcState::config on the disk using *confy* create.
    pub fn store_config_data(&self) {
        let res = confy::store(APP_NAME, None, &self.config);
        if let Err(why) = res {
            eprintln!("Got Error:\n{:#?}", why);
        }
    }

    /// Get currently active theme. If `detect_system` is set to true, function will detect set
    /// system theme and return either Dark or Light.
    pub fn get_theme(&self, detect_system: bool) -> Theme {
        if detect_system && self.config.theme == Theme::System {
            let mode = dark_light::detect();
            match mode {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
                dark_light::Mode::Default => Theme::Dark,
            }
        } else {
            self.config.theme
        }
    }

    /// Change theme of application. This will be saved at exit.
    pub fn set_theme(&mut self, theme: Theme) {
        self.config.theme = theme;
        self.store_config_data();
    }

    /// Change language of the app. This will be saved at exit.
    ///
    /// * `language` - Must be in the array of languages passed to `CalcState::new()`
    pub fn set_language(&mut self, language: &str) {
        assert!(self.available_languages.contains(&language.to_string()));
        rust_i18n::set_locale(language);
        self.config.language = String::from(language);
        self.store_config_data();
    }

    /// Get current app language
    pub fn get_language(&self) -> &str {
        &self.config.language
    }

    /// Get numeric base
    pub fn get_radix(&self) -> Radix {
        self.radix
    }

    /// Change numeric base of the calculated results.
    pub fn set_radix(&mut self, radix: Radix) {
        self.radix = radix;
    }

    /// Get function keyboard
    pub fn get_function_pad(&self) -> FuncPad {
        self.function_pad
    }

    /// Change numeric base of the calculated results.
    pub fn set_function_pad(&mut self, function_pad: FuncPad) {
        self.function_pad = function_pad;
    }
}

#[cfg(test)]
mod calcstate_tests;
