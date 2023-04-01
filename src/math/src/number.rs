use crate::Result;
use std::cmp::Ordering;

/// Represent a number
pub struct Number {
    inner: f64,
}

impl AsRef<f64> for Number {
    fn as_ref(&self) -> &f64 {
        &self.inner
    }
}
impl Number {
    /// 0.0
    pub const ZERO: Self = Self { inner: 0.0 };

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
    /// let random = Number::random();
    /// assert!(random.div(Number::ZERO).is_err());
    /// assert_eq!(Number::ZERO.div(random), Number::ZERO);
    /// assert_eq!(rand.div(rand), Number::ONE);
    /// let a = Number::from(3);
    /// let b = Number::from(1.5);
    /// let neg_b = Number::from(-1.5);
    /// assert_eq!(a.div(b), Ok(Number::from(2)));
    /// assert_eq!(a.div(neg_b), Ok(Number::from(-2)))
    /// ```
    pub fn div(&self, other: impl Into<Self>) -> Result<Self> {
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
