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

    ///
    pub fn to_string_radix(&self, _radix: u8) -> String {
        todo!()
    }
}
