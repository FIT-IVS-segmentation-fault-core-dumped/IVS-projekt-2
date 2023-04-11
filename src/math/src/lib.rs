//! # The Math Library
//! Provide a `Number` type with mathematical operations
//! As well as engine to quickly evaluate infix math

#![deny(missing_docs)]

/// Error type
pub mod error;
/// Number type
pub mod number;

/// Token parsing
pub mod token;

/// Engine to perform math evaluation
pub mod engine;

use std::collections::HashMap;

pub use engine::Engine;
pub use number::Number;

/// Result type for this library
pub type Result<T> = std::result::Result<T, error::Error>;

/// Defined variable
pub enum Variable {
    /// A constant, same as a function without parameter
    Constant(Number),

    /// A function
    Function {
        /// Number of parameters
        argc: u8,
        /// Pointer to the function itself
        ptr: fn(&[Number]) -> Result<Number>,
    },
}

/// Calculator struct
pub struct Calculator {
    tokens: Vec<token::Token>,
    engine: Box<dyn Engine>,
    variables: HashMap<String, Variable>,
}

impl Calculator {
    /// Create a new instance
    pub fn new() -> Self {
        let mut res = Self {
            tokens: Vec::new(),
            variables: HashMap::new(),
            engine: Box::new(engine::ShuntingYardEngine::default()) as Box<_>,
        };

        res.add_basic_function();

        res
    }

    fn add_basic_function(&mut self) {
        // TODO
    }

    /// Add new constant or update existing one to the calculator
    /// Name is *case-insensitive*
    /// Naming of the constant shouldn't match with any built-in functions like `sqrt`, `log`, etc
    /// ```
    /// # use math::Calculator;
    /// # use math::Number;
    /// let mut calculator = Calculator::new();
    /// assert_eq!(calculator.add_constant("pi", Number::from(3)), true);
    /// assert_eq!(calculator.add_constant("sqrt", Number::random()), false);
    /// ```
    pub fn add_constant(&mut self, name: &str, num: Number) -> bool {
        let name = name.to_lowercase();
        if matches!(self.variables.get(&name), Some(Variable::Function { .. })) {
            return false;
        }

        self.variables.insert(name, Variable::Constant(num));
        true
    }

    /// Set the engine of the `Calculator`
    /// ```ignore
    /// # use math::Calculator;
    /// let mut calculator = Calculator::new();
    /// let postfix_engine = MyPostFixEngine::new();
    /// calculator.set_engine(postfix_engine);
    /// assert_eq!(calculator.evaluate("1 2 3 * -"), Ok(Number::from(-5)));
    /// ```
    pub fn set_engine(&mut self, engine: impl Engine + 'static) {
        self.engine = Box::new(engine) as Box<_>;
    }

    /// Evaluate a math expression using the given `Engine` (default is the infix math `ShuntingYardEngine`)
    /// If the evaluation success, the constant `ANS` will be stored/updated into the variables list of the calculator
    pub fn evaluate(&mut self, s: &str) -> Result<Number> {
        self.tokens.clear();
        let mut scanner = token::Scanner::new(s);

        while let Some(token) = scanner.next_token()? {
            self.tokens.push(token);
        }

        let ans = self
            .engine
            .execute(self.tokens.as_slice(), &self.variables)?;

        self.add_constant("ans", ans.clone());
        Ok(ans)
    }
}

/// High level function
/// for use in the long run, it is recommendded to create (and hold) an instance of `Calculator`
/// struct itself, as it may reserves the allocation spaces for future evaluation process
pub fn evaluate(s: &str) -> Result<Number> {
    Calculator::new().evaluate(s)
}
