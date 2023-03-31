/// Represent a number
pub struct Number {
    inner: f64,
    precision: u8,
}

impl AsRef<f64> for Number {
    fn as_ref(&self) -> &f64 {
        &self.inner
    }
}

impl Number {
    /// 0.0
    pub const ZERO: Self = Self {
        inner: 0.0,
        precision: 6,
    };

    /// 3.14159...
    pub const PI: Self = Self {
        inner: std::f64::consts::PI,
        precision: 6,
    };

    /// 2.71828...
    pub const E: Self = Self {
        inner: std::f64::consts::E,
        precision: 6,
    };

    /// Set the maximum precision of the Number
    /// This only affect the `to_string` (and its related) function
    /// The calculation is unaffected
    ///
    /// ```
    /// let mut e = Number::E;
    /// assert_eq!(e.to_string(), "2.718281");
    /// e.set_precision(2);
    /// assert_eq!(e.to_string(), "2.71");
    /// ```
    pub fn set_precision(&mut self, precision: u8) {
        self.precision = precision;
    }

    /// Get the decimal string
    ///
    /// ```
    /// let pi = Number::PI.to_string();
    /// let zero = Number::ZERO.to_string();
    /// let neg = Number::from(-0.1).to_string();
    ///
    /// assert_eq!(zero, "0");
    /// assert_eq!(neg, "-0.1");
    /// assert!(pi, "3.141592"); // default precision is 6 decimal points
    /// ```
    pub fn to_string(&self) -> String {
        todo!()
    }

    /// Get the binary string
    ///
    /// ```
    /// let pi = Number::PI.to_string_binary();
    /// let zero = Number::ZERO.to_string_binary();
    /// let neg = Number::from(-0.1).to_string_binary();
    ///
    /// assert_eq!(zero, "0");
    /// assert_eq!(neg, "-0.1");
    /// assert!(pi.starts_with("3.14159"));
    /// ```
    pub fn to_string_binary(&self) -> String {
        todo!()
    }

    /// Get the octal string
    ///
    /// ```
    /// let pi = Number::PI.to_string_octal();
    /// let zero = Number::ZERO.to_string_octal();
    /// let neg = Number::from(-0.1).to_string_octal();
    ///
    /// assert_eq!(zero, "0");
    /// assert_eq!(neg, "-0.00011");
    /// assert!(pi, "11.001001");
    /// ```
    pub fn to_string_octal(&self) -> String {
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
    pub fn to_string_hex(&self) -> String {
        todo!()
    }
}
