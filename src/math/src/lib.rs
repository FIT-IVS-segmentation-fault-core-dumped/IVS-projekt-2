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

#[derive(Clone)]
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

impl Variable {
    /// Get the number of argument for this variable to work
    pub fn argc(&self) -> u8 {
        match self {
            Self::Constant(_) => 0,
            Self::Function { argc, .. } => *argc,
        }
    }
    /// Calculate the value of the variable
    pub fn calc(&self, nums: &[Number]) -> Result<Number> {
        match self {
            Self::Constant(v) => Ok(v.clone()),
            Self::Function { ptr, .. } => (ptr)(nums),
        }
    }
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
        res.add_constant("e", Number::e());
        res.add_constant("pi", Number::pi());

        res
    }

    fn add_basic_function(&mut self) {
        let mut add_function = |name: &str, argc, ptr| {
            self.variables
                .insert(name.to_lowercase(), Variable::Function { argc, ptr })
        };

        add_function("root", 2, |nums| nums[1].root(&nums[0]));
        add_function("sqrt", 1, |nums| nums[0].sqrt());
        add_function("ln", 1, |nums| nums[0].ln());
        add_function("log2", 1, |nums| nums[0].log2());
        add_function("log10", 1, |nums| nums[0].log10());
        add_function("log", 2, |nums| nums[1].log(&nums[0]));
        add_function("sin", 1, |nums| nums[0].sin());
        add_function("cos", 1, |nums| nums[0].cos());
        add_function("tg", 1, |nums| nums[0].tg());
        add_function("cotg", 1, |nums| nums[0].cotg());
        add_function("arcsin", 1, |nums| nums[0].arcsin());
        add_function("arccos", 1, |nums| nums[0].arccos());
        add_function("arctg", 1, |nums| nums[0].arctg());
        add_function("arccotg", 1, |nums| nums[0].arccotg());
        add_function("arccotg", 1, |nums| nums[0].arccotg());
        add_function("pow", 2, |nums| nums[0].power(&nums[1]));
        add_function("abs", 1, |nums| nums[0].abs());
        add_function("comb", 2, |nums| Number::combination(&nums[0], &nums[1]));
        add_function("random", 0, |_| Ok(Number::random()));
    }

    /// Add new constant or update existing one to the calculator \
    /// Name is *case-insensitive* \
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

    /// Remove a constant from the list
    ///
    /// ```
    /// # use math::Calculator;
    /// # use math::Number;
    /// let mut calculator = Calculator::new();
    /// let constant = Number::from(177183);
    ///
    /// calculator.add_constant("my_const", constant.clone());
    /// assert_eq!(calculator.evaluate("my_const()"), Ok(constant));
    ///
    /// calculator.remove_constant("my_const");
    /// assert!(calculator.evaluate("my_const()").is_err());
    /// ```
    pub fn remove_constant(&mut self, name: &str) -> Option<Number> {
        let name = name.to_lowercase();
        let val = self.variables.remove(&name)?;

        match val {
            Variable::Constant(num) => Some(num),
            _ => {
                self.variables.insert(name, val);
                None
            }
        }
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

    /// Evaluate a math expression using the given `Engine` (default is the infix math `ShuntingYardEngine`) \
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
