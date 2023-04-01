use crate::Result;
use std::cmp::Ordering;

/// Represent a number
#[derive(Debug)]
pub struct Number {
    inner: f64,
}

impl<T: Into<f64>> From<T> for Number {
    fn from(v: T) -> Self {
        Self { inner: v.into() }
    }
}

impl AsRef<f64> for Number {
    fn as_ref(&self) -> &f64 {
        &self.inner
    }
}
impl Number {
    /// 0.0
    pub const ZERO: Self = Self { inner: 0.0 };

    /// 1.0
    pub const ONE: Self = Self { inner: 1.0 };

    /// 3.14159...
    pub const PI: Self = Self {
        inner: std::f64::consts::PI,
    };

    /// 2.71828...
    pub const E: Self = Self {
        inner: std::f64::consts::E,
    };

    /// Get the decimal string
    ///
    /// ```
    /// let pi = Number::PI.to_string(6);
    /// let zero = Number::ZERO.to_string(6);
    /// let neg = Number::from(-0.1).to_string(6);
    ///
    /// assert_eq!(zero, "0");
    /// assert_eq!(neg, "-0.1");
    /// assert!(pi, "3.141592"); // default precision is 6 decimal points
    /// ```
    pub fn to_string(&self, precision: u8) -> String {
        todo!()
    }

    /// Get the binary string
    ///
    /// ```
    /// let pi = Number::PI.to_string_binary(6);
    /// let zero = Number::ZERO.to_string_binary(6);
    /// let neg = Number::from(-0.1).to_string_binary(6);
    ///
    /// assert_eq!(zero, "0");
    /// assert_eq!(neg, "-0.1");
    /// assert!(pi.starts_with("3.14159"));
    /// ```
    pub fn to_string_binary(&self, precision: u8) -> String {
        todo!()
    }

    /// Get the octal string
    ///
    /// ```
    /// let pi = Number::PI.to_string_octal(6);
    /// let zero = Number::ZERO.to_string_octal(6);
    /// let neg = Number::from(-0.1).to_string_octal(6);
    ///
    /// assert_eq!(zero, "0");
    /// assert_eq!(neg, "-0.063146");
    /// assert!(pi, "3.11037");
    /// ```
    pub fn to_string_octal(&self, precision: u8) -> String {
        todo!()
    }

    /// Get the hexadecimal string
    ///
    /// ```
    /// let pi = Number::PI.to_string_hex();
    /// let zero = Number::ZERO.to_string_hex();
    /// let neg = Number::from(-0.1).to_string_hex();
    ///
    /// assert_eq!(zero, "0");
    /// assert_eq!(neg, "-0.199999");
    /// assert!(pi.starts_with("3.243F6A"));
    /// ```
    pub fn to_string_hex(&self, precision: u8) -> String {
        todo!()
    }
}

impl Number {
    /// Generate a random number in range of <0, 1>
    ///
    /// ```
    /// assert_ne!(Number::random(), Number::random());
    /// assert_ge!(Number::random(), Number::Zero);
    /// assert_le!(Number::random(), Number::One);
    /// ```
    pub fn random() -> Self {
        todo!()
    }

    /// Add two numbers together
    ///
    /// ```
    /// # use math::Number;
    ///
    /// let a = Number::from(0.1);
    /// let b = Number::from(0.2);
    ///
    /// assert_eq!(a.add(b), Ok(Number::from(0.3)));
    /// assert_eq!(b.add(a), a.add(b));
    /// ```
    pub fn add(&self, other: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Subtract two numbers
    ///
    /// ```
    /// # use math::Number;
    ///
    /// let a = Number::from(0.1);
    /// let b = Number::from(0.2);
    ///
    /// assert_eq!(a.sub(b), Ok(Number::from(-0.1)));
    /// assert_ne!(b.sub(a), Ok(Number::from(0.1)));
    /// ```
    pub fn sub(&self, other: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Multiply two numbers
    ///
    /// ```
    /// # use math::Number;
    ///
    /// let a = Number::from(0.1);
    /// let b = Number::from(0.2);
    ///
    /// assert_eq!(a.mul(b), Ok(Number::from(-0.1)));
    /// assert_ne!(b.mul(a), Ok(Number::from(0.1)));
    /// ```
    pub fn mul(&self, other: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Divide two numbers
    ///
    /// # Error
    /// Return Error::DivisionZero if `other` is 0
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let random = Number::random();
    ///     assert!(random.div(Number::ZERO).is_err());
    ///     assert_eq!(Number::ZERO.div(random)?, Number::ZERO);
    ///     assert_eq!(rand.div(rand)?, Number::ONE);
    ///
    ///     let a = Number::from(3);
    ///     let b = Number::from(1.5);
    ///     let neg_b = Number::from(-1.5);
    ///     assert_eq!(a.div(b)?, Number::from(2));
    ///     assert_eq!(a.div(neg_b)?, Number::from(-2));
    /// #   Ok(())
    /// # }
    /// ```
    pub fn div(&self, other: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// ```
    /// # use math::Number;
    /// assert_eq!(Number::random().pow(Number::ZERO), Ok(Number::ONE));
    /// assert_eq!(Number::from(5).pow(2), Ok(Number::from(25)));
    /// ```
    pub fn power(&self, exp: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Get the remainder of `self / other`
    ///
    /// ```
    /// # use math::Number;
    /// assert_eq!(Number::from(5).modulo(2), Ok(Number::ONE));
    /// ```
    pub fn modulo(&self, other: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Calculate factorial of a given number
    /// The number is not limited to integer, it can be a fraction
    ///
    /// # Error
    /// return Error::FactorialNegative if the number is less than 0
    ///
    /// ```
    /// # use math::Number;
    /// assert_eq!(Number::Zero.factorial(), Number::One);
    /// assert_eq!(Number::from(5).factorial(), Ok(Number::from(120)));
    /// assert_eq!(Number::from(3.2).factorial().unwrap().to_string(6), "7.756689");
    /// assert!(Number::from(-1).factorial().is_err());
    /// ```
    pub fn factorial(&self) -> Result<Self> {
        todo!()
    }

    /// Returns the logarithm of the number with respect to an arbitrary `base`.
    ///
    /// # Error
    /// Error::LogUndefinedBase if the `base` is less or equal than 0
    /// Error::LogUndefinedNumber if the number is less or equal than 0
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     assert!(Number::random().log(0).is_err());
    ///     assert!(Number::random().log(-1.2).is_err());
    ///     assert!(Number::ZERO.log(Number::random()).is_err());
    ///     assert!(Number::from(-0.3).log(Number::random()).is_err());
    ///
    ///     let a = Number::random();
    ///     let base = Number::random();
    ///     let b = Number::from(2);
    ///
    ///     // Number same as base
    ///     assert_eq!(a.log(a)?, Number::ONE);
    ///     // Product rule log(xy) == log(x) + log(y)
    ///     assert_eq!(a.mul(b)?.log(base), a.log(base)?.add(b.log(base)?));
    ///     // Quotient rule log(x/y) == log(x) - log(y)
    ///     assert_eq!(a.div(b)?.log(base), a.log(base)?.sub(b.log(base)?));
    ///     // Log of power log(x^y) == y * log(x)
    ///     assert_eq!(a.pow(b)?.log(base), b.mul(a.log(base)?));
    ///     // Log of one
    ///     assert_eq!(Number::ONE.log(base)?, Number::ZERO);
    ///     // Log reciprocal log(1/x) = -ln(x);
    ///     assert_eq!(Number::ONE.div(a)?.log(base), a.log(base)?.mul(-1));
    ///     # Ok(())
    /// # }
    /// ```
    pub fn log(&self, base: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Same as `Number::log` with `base` of 2
    pub fn log2(&self) -> Result<Self> {
        todo!()
    }

    /// Same as `Number::log` with `base` of `Number::E`
    pub fn ln(&self) -> Result<Self> {
        todo!()
    }

    /// Same as `Number::log` with `base` of 10
    pub fn log10(&self) -> Result<Self> {
        todo!()
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner
            .partial_cmp(&other.inner)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Number {}
