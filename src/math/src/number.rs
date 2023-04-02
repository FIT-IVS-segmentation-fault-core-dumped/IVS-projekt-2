use crate::error::Error;
use crate::Result;
use fraction::GenericFraction;
use std::cmp::Ordering;

#[derive(Default, Debug, Clone, Copy)]
/// Represent a number
pub struct Number {
    inner: GenericFraction<u64>,
}

#[derive(Default, Debug, Clone, Copy)]
#[non_exhaustive]
/// Radix to use to represent a Number
pub enum Radix {
    /// Binary
    Bin,
    /// Octal
    Oct,
    #[default]
    /// Decimal - default
    Dec,
    /// Hexadecimal
    Hex,
}

impl<T: Into<GenericFraction<u64>>> From<T> for Number {
    fn from(v: T) -> Self {
        Self { inner: v.into() }
    }
}

impl Number {
    /// 0.0
    pub const ZERO: Self = Self::new_unchecked(0, 1);

    /// 1.0
    pub const ONE: Self = Self::new_unchecked(1, 1);

    /// 3.14159... ~= 104 348/33 215
    pub const PI: Self = Self::new_unchecked(104348, 33215);

    /// 2.71828... ~= 2721 / 1001
    pub const E: Self = Self::new_unchecked(2721, 1001);

    /// Create a new number in the form `num / denom`
    /// This way we can safely create number can cannot be expressed in binary form like 0.1
    ///
    /// # Error
    /// Error::DivisionZero if `denom` is 0
    ///
    /// ```
    /// # use math::Number;
    /// assert!(Number::new(43, 0).is_err());
    /// assert!(Number::new(30, 10), Number::new(3, 10));
    /// assert!(Number::new(2, 10), Number::new(1, 5));
    /// assert_eq!(Number::new(1, 10).unwrap().to_string(Radix::Dec, 5), "0.1");
    /// ```
    pub const fn new(num: i64, denom: i64) -> Result<Self> {
        if denom == 0 {
            return Err(Error::DivisionZero);
        }

        Ok(Self::new_unchecked(num, denom))
    }

    /// Same as `Number::new` but bypass the zero check for denom
    /// This function should be use only in const context!!!
    pub const fn new_unchecked(num: i64, denom: i64) -> Self {
        let n = num.abs() as u64;
        let d = denom.abs() as u64;

        let sign = if (num * denom).is_positive() {
            fraction::Sign::Plus
        } else {
            fraction::Sign::Minus
        };

        Self {
            inner: GenericFraction::new_raw_signed(sign, n, d),
        }
    }

    /// Get the formatted string of a number
    ///
    /// ```
    /// # use math::number::{Number, Radix};
    /// let pi = Numper::PI;
    /// let zero = Number::ZERO;
    /// let neg = Number::new(-1, 10);
    ///
    /// let precision = 6;
    ///
    /// assert_eq!(zero.to_string(Radix::Bin, precision), "0");
    /// assert_eq!(zero.to_string(Radix::Oct, precision), "0");
    /// assert_eq!(zero.to_string(Radix::Dec, precision), "0");
    /// assert_eq!(zero.to_string(Radix::Hex, precision), "0");
    ///
    /// assert_eq!(pi.to_string(Default::default(), precision), "3.141592");
    /// assert_eq!(pi.to_string(Radix::Bin, precision), "11.001001");
    /// assert_eq!(pi.to_string(Radix::Oct, precision), "3.110375");
    /// assert_eq!(pi.to_string(Radix::Hex, precision), "3.243F6A");
    ///
    /// assert_eq!(neg.to_string(Default::default(), precision), "-0.1");
    /// assert_eq!(neg.to_string(Radix::Bin, precision), "-0.00011");
    /// assert_eq!(neg.to_string(Radix::Oct, precision), "-0.063146");
    /// assert_eq!(neg.to_string(Radix::Hex, precision), "-0.199999");
    /// ```
    pub fn to_string(&self, radix: Radix, precision: u8) -> String {
        todo!()
    }
}

impl Number {
    /// Generate a random number in range of <0, 1>
    ///
    /// ```
    /// # use math::Number;
    /// assert!(Number::random() != Number::random());
    /// assert!(Number::random() >= Number::ZERO);
    /// assert!(Number::random() <= Number::ONE);
    /// ```
    pub fn random() -> Self {
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        let denom: u64 = rng.gen_range(1..(u64::MAX / 2));
        let num: u64 = rng.gen_range(0..=denom);

        Self::new_unchecked(num as _, denom as _)
    }

    /// Add two numbers together
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    /// let a = Number::new(1, 10)?;
    /// let b = Number::new(2, 10)?;
    ///
    /// assert_eq!(a.add(b)?, Number::new(3, 10)?);
    /// assert_eq!(b.add(a), a.add(b));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn add(&self, other: impl Into<Self>) -> Result<Self> {
        Ok(Self {
            inner: self.inner + other.into().inner,
        })
    }

    /// Subtract two numbers
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let a = Number::new(1, 10)?;
    ///     let b = Number::new(2, 10)?;
    ///
    ///     assert_eq!(a.sub(b)?, Number::new(-1, 10)?);
    ///     assert_eq!(b.sub(a)?, Number::new(1, 10)?);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn sub(&self, other: impl Into<Self>) -> Result<Self> {
        Ok(Self {
            inner: self.inner - other.into().inner,
        })
    }

    /// Multiply two numbers
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let a = Number::new(1, 10)?;
    ///     let b = Number::new(2, 10)?;
    ///
    ///     assert_eq!(a.mul(b)?, Number::new(2, 100)?);
    ///     assert_eq!(b.mul(a), a.mul(b));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn mul(&self, other: impl Into<Self>) -> Result<Self> {
        Ok(Self {
            inner: self.inner * other.into().inner,
        })
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
    ///     let b = Number::new(3, 2);
    ///     let neg_b = Number::new(-3, 2);
    ///     assert_eq!(a.div(b)?, Number::from(2));
    ///     assert_eq!(a.div(neg_b)?, Number::from(-2));
    /// #   Ok(())
    /// # }
    /// ```
    pub fn div(&self, other: impl Into<Self>) -> Result<Self> {
        let other = other.into();

        if other == Self::ZERO {
            return Err(Error::DivisionZero);
        }

        Ok(Self {
            inner: self.inner / other.inner,
        })
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
    /// assert_eq!(Number::from(-5).modulo(2), Ok(Number::ONE));
    /// assert_eq!(Number::from(5).modulo(-2), Number::ONE.mul(-1));
    /// ```
    pub fn modulo(&self, other: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Get the absolute value of the given number
    ///
    /// ```
    /// # use math::Number;
    /// let a = Number::random();
    /// let neg_a = a.mul(-1);
    ///
    /// assert_eq!(a.abs(), Ok(a));
    /// assert_eq!(neg_a.abs(), Ok(a));
    /// ```
    pub fn abs(&self) -> Result<Self> {
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
    /// assert_eq!(Number::new(32, 10).factorial().unwrap().to_string(6), "7.75669");
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
    ///     assert!(Number::new(-3, 10).log(Number::random()).is_err());
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

    /// Returns the nth root of a number
    ///
    /// # Error
    /// Error::ZeroNthRoot if the `nth` is 0
    /// Error::NegativeRoot if the number is negative
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     assert!(Number::random().root(0).is_err());
    ///     assert!(Number::from(-1).root(2).is_err());
    ///     assert!(Number::from(-1).root(88).is_err());
    ///     assert!(Number::from(-1).root(3).is_ok());
    ///     assert!(Number::from(-1).root(87).is_ok());
    ///
    ///     let a = Number::random();
    ///     let b = Number::random();
    ///     let nth = Number::random();
    ///
    ///     // root(a^nth) == a
    ///     assert_er!(a.pow(nth)?.root(nth), a);
    ///     // root(ab) == root(a) * root(b)
    ///     assert_eq!(a.mul(b)?.root(nth), a.root(nth)?.mul(b.root(nth)?));
    ///     // root(a/b) == root(a) / root(b)
    ///     assert_eq!(a.div(b)?.root(nth), a.root(nth)?.div(b.root(nth)?));
    ///
    /// #     Ok(())
    /// # }
    /// ```
    pub fn root(&self, nth: impl Into<Self>) -> Result<Self> {
        todo!()
    }

    /// Returns the square root of a number.
    /// This function is the same as `root` with the `nth` of 2
    pub fn sqrt(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the sine of a number (in radians).
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let x = Number::random();
    ///     let half_pi = Number::PI.div(2)?;
    ///     let sin_x = x.sin();
    ///
    ///     // sin(x) == cos(PI/2 - x)
    ///     assert_eq!(sin_x, half_pi.sub(x)?.cos());
    ///
    /// #     Ok(())
    /// # }
    /// ```
    pub fn sin(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the cosine of a number (in radians).
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let x = Number::random();
    ///     let half_pi = Number::PI.div(2)?;
    ///     let cos_x = x.cos();
    ///
    ///     // cos(x) == sin(PI/2 - x)
    ///     assert_eq!(cos_x, half_pi.sub(x)?.sin());
    ///
    /// #     Ok(())
    /// # }
    /// ```
    pub fn cos(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the tangent of a number (in radians).
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let x = Number::random();
    ///     let tan_x = x.tg();
    ///
    ///     // tg(x) == sin(x) / cos(x)
    ///     assert_eq!(tan_x, x.sin()?.div(x.cos()?));
    ///     // tg(x) == 1 / cotg(x)
    ///     assert_eq!(tan_x, Number::ONE.div(x.cotg()?));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn tg(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the cotangent of a number (in radians).
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let x = Number::random();
    ///     let cot_x = x.cotg();
    ///
    ///     // cotg(x) == cos(x) / sin(x)
    ///     assert_eq!(cot_x, x.cos()?.div(x.sin()?));
    ///     // cotg(x) == 1 / tg(x)
    ///     assert_eq!(cot_x, Number::ONE.div(x.tg()?));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn cotg(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the arcsine of a number. Return value is in radians in the range <-pi/2, pi/2>
    ///
    /// # Error
    /// Error::OutOfRange if the number is not in the range <-1, 1>
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     assert!(Number::from(-2).arcsin().is_err());
    ///     assert!(Number::from(2).arcsin().is_err());
    ///     let x = Number::random();
    ///     let sin_x = x.sin()?;
    ///
    ///     assert_eq!(x, sin_x.arcsin()?);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn arcsin(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the arccosine of a number. Return value is in radians in the range <0, pi>
    ///
    /// # Error
    /// Error::OutOfRange if the number is not in the range <-1, 1>
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     assert!(Number::from(-2).arccos().is_err());
    ///     assert!(Number::from(2).arccos().is_err());
    ///     let x = Number::random();
    ///     let cos_x = x.cos()?;
    ///
    ///     assert_eq!(x, cos_x.arccos()?);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    pub fn arccos(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the arctangent of a number. Return value is in radians in the range <-pi/2, pi/2>
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let x = Number::random();
    ///     let tan_x = x.tg()?;
    ///
    ///     assert_eq!(x, tan_x.arctg()?);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn arctg(&self) -> Result<Self> {
        todo!()
    }

    /// Computes the arccotangent of a number. Return value is in radians in the range <0, pi>
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     let x = Number::random();
    ///     let cot_x = x.cotg()?;
    ///
    ///     assert_eq!(x, cot_x.arccotg()?);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    pub fn arccotg(&self) -> Result<Self> {
        todo!()
    }

    /// Calculate combination number of the given `n` and `k`
    ///
    /// Since combination number is defined as `C(n, k)` mathematically
    /// Neither `n` nor `k` is considered the center of the function
    /// that's why this function does not taking `self` as parameter like other functions
    ///
    /// # Error
    /// Error::FactorialNegative if either `n` or `k` is negative, because they will need to be factorialized
    ///
    /// ```
    /// # use math::Number;
    ///
    /// # fn main() -> math::Result<()> {
    ///     assert!(Number::combination(-1, Number::random()).is_err());
    ///     assert!(Number::combination(Number::random(), -1).is_err());
    ///
    ///     let n = Number::random();
    ///
    ///     // if k > n => C(n, k) == 0
    ///     assert_eq!(Number::combination(4, 3)?, Number::ZERO);
    ///     // C(n, 0) == 1
    ///     assert_eq!(Number::combination(n, 0)?, Number::ONE);
    ///     // C(n, 1) == n
    ///     assert_eq!(Number::combination(n, 1)?, n);
    ///     // C(n, n) == 1
    ///     assert_eq!(Number::combination(n, n)?, Number::ONE);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn combination(n: impl Into<Self>, k: impl Into<Self>) -> Result<Self> {
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
