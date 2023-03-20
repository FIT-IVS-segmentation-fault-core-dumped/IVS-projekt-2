/// Represent a number
pub struct Number(f64);

impl AsRef<f64> for Number {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Number {
    /// 0.0
    pub const ZERO: Number = Self(0.0);

    ///
    pub fn to_string_radix(&self, _radix: u8) -> String {
        todo!()
    }
}
