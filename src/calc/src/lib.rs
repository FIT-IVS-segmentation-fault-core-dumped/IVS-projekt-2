//! Calculator application
//!
//! This file contains definition of calculator state,
//! which defines functionality of our app.
pub mod delegate;
pub mod expr_manager;
pub mod history;
pub mod widgets;

use druid::{Application, Data, Lens, WindowId};
use expr_manager::ExprManager;
use history::History;
use math::{number::Radix, Number};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fmt, rc::Rc};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
    Log, LogN, Ln, Sqrt, Root, Pow,
    /// Reprezents `a^2` operation.
    Pow2,
    /// Reprezents `root(3, a)` operation.
    Root3,
    Abs, Comb, Fact, Mod,
}

/// Used to map button presses to functionality.
/// Now if we want to implement alternative ways
/// of using our calculator, we just need to
/// pass this enum as action to the pressed button.
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum PressedButton {
    /// Numpad 0-9 or A-F (10 - 15)
    Num(u8),
    /// Operations, that require 2 operands.
    BinOpt(Opt),
    /// Operations, that require only 1 operand.
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
    /// Floating point.
    Comma,
    /// Last result.
    Ans,
    /// Operation, which generates random number
    /// on each evaluation.
    Random,
}

/// Color theme of the app.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Data, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

/// String representations for `Theme`
impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents tabs that switch between different function keyboards
#[derive(Debug, PartialEq, Clone, Copy, Data)]
pub enum FunctionTabs {
    Main,
    Func,
    Const,
}

/// Holds application configuration, which is saved on the disk.
/// This is loaded at each start of the application.
#[derive(Serialize, Deserialize, Lens, Clone)]
pub struct CalcConfig {
    theme: Theme,
    language: String,
    history: History,
}

/// Defines what will the initial config hold (when no config is found on the disk).
impl Default for CalcConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "en".to_owned(),
            history: History::default(),
        }
    }
}

impl Data for CalcConfig {
    fn same(&self, other: &Self) -> bool {
        self.theme == other.theme
            && self.language == other.language
            && self.history.same(&other.history)
    }
}

/// Holds all user defined constants
#[derive(Lens, Clone)]
pub struct Constants {
    keys: Vec<String>,
    values: Vec<String>,

    pub key_str: String,
    pub value_str: String,
}

impl Constants {
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            key_str: String::new(),
            value_str: String::new(),
        }
    }
}

impl Data for Constants {
    fn same(&self, other: &Self) -> bool {
        self.keys == other.keys && self.values == other.values
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
    /// User defined constants
    constants: Constants,
    /// Switch between fucntional keyboards
    function_tab: FunctionTabs,
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
    /// Result from the last calculation
    ans: f64,
    /// main window id
    main_win_id: WindowId,
    /// If `has_focus` is true it means the app will send user keyboard input to display
    display_focus: bool,
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
        f(&data.expr_man.get_display_str(true))
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut CalcState, f: F) -> V {
        let mut dis = data.expr_man.get_display_str(true);
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
            && self.function_tab == other.function_tab
            && self.config.same(&other.config)
            && self.result == other.result
            && self.constants.same(&other.constants)
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
            function_tab: FunctionTabs::Main,
            constants: Constants::new(),
            // Convert array of string slices to vector of strings.
            available_languages: Rc::new(languages.iter().map(|&s| String::from(s)).collect()),
            config,
            calc: Rc::new(RefCell::new(math::Calculator::new())),
            result: String::new(),
            result_is_err: false,
            degrees: true,
            ans: 0.0,
            display_focus: true,
            main_win_id: WindowId::next(),
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
                let eval_str = match self.expr_man.get_eval_str() {
                    Ok(str) => str,
                    Err(msg) => {
                        eprintln!("error: {}", msg);
                        return;
                    }
                };
                let result = self.calc.borrow_mut().evaluate(&eval_str);

                // Set resulting variable according to the resulting value.
                (self.result, self.result_is_err) = match result {
                    Err(e) => (format!("{:?}", e), true),
                    Ok(num) => (num.to_string(self.radix, 5), false),
                };
            }
            PressedButton::Clear => {
                self.expr_man.process_button(button);
                if self.result_is_err {
                    self.result.clear();
                    self.result_is_err = false;
                }
            }

            // Relay other buttons to the expression manager.
            other => self.expr_man.process_button(other),
        };
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
    pub fn get_function_tab(&self) -> FunctionTabs {
        self.function_tab
    }

    /// Change numeric base of the calculated results.
    pub fn set_function_tab(&mut self, function_tab: FunctionTabs) {
        self.function_tab = function_tab;
    }

    /// Get user defined constants
    pub fn get_constants(&self) -> &Constants {
        &self.constants
    }

    /// Add constant as key-value pair to the math library. If constant name is not valid (sin, cos, log...)
    /// function returns false
    pub fn add_constant(&mut self, key: String, value: String) -> bool {
        let Ok(num) = math::evaluate(&value) else {return false;};

        // let bignum = (value * 100000.) as i128;

        let is_added = self.calc.borrow_mut().add_constant(&key, Number::from(num));

        if is_added {
            self.constants.keys.push(key);
            self.constants.values.push(value);
        }
        is_added
    }

    /// Remove constant from math library as well as from `CalcState` data
    pub fn remove_constant(&mut self, index: usize) {
        self.calc
            .borrow_mut()
            .remove_constant(self.constants.keys[index].as_str());
        self.constants.keys.remove(index);
        self.constants.values.remove(index);
    }

    /// Check if the constant already exists
    pub fn is_new_constant(&self, key: String) -> bool {
        !(self.constants.keys.contains(&key) || key == "e" || key == "pi" || key == "ANS")
    }

    /// Copy the currently displayed result into the system clipboard
    pub fn copy_result(&self) {
        let mut clipboard = Application::global().clipboard();
        clipboard.put_string(self.result.clone());
    }

    /// Copy the currently displayed expression into the system clipboard
    pub fn copy_expression(&self) {
        let mut clipboard = Application::global().clipboard();
        clipboard.put_string(self.expr_man.get_display_str(false));
    }

    /// Get a reference to History struct
    pub fn get_history(&self) -> &History {
        &self.config.history
    }

    /// Get a mutable reference to History struct
    pub fn get_mut_history(&mut self) -> &mut History {
        &mut self.config.history
    }

    /// Save the expression and result to history
    pub fn save_equation(&mut self) {
        self.config
            .history
            .data
            .push((self.expr_man.get_display_str(false), self.result.clone()));
        self.store_config_data();
    }

    /// Set angular unit based on `degrees` on either degrees or radians
    pub fn set_angular_unit(&mut self, degrees: bool) {
        self.degrees = degrees;
    }

    /// Get currently set angular unit (true = degrees, false = radians)
    pub fn get_angular_unit(&self) -> bool {
        self.degrees
    }

    /// Get currently set angular unit (true = degrees, false = radians)
    pub fn get_main_win_id(&self) -> WindowId {
        self.main_win_id
    }

    /// Get currently set angular unit (true = degrees, false = radians)
    pub fn set_main_win_id(&mut self, win_id: WindowId) {
        self.main_win_id = win_id;
    }

    /// Get currently set angular unit (true = degrees, false = radians)
    pub fn get_display_focus(&self) -> bool {
        self.display_focus
    }

    /// Get currently set angular unit (true = degrees, false = radians)
    pub fn set_display_focus(&mut self, has_focus: bool) {
        self.display_focus = has_focus;
    }

    /// Update value of ans. Should be called after each calculation
    pub fn update_ans(&mut self, value: f64) {
        self.ans = value;
    }
}
