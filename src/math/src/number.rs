use crate::error::Error;
use crate::Result;
use num::rational::Ratio;
use num::BigInt;
use num::Signed as _;
use num::ToPrimitive;
use once_cell::sync::OnceCell;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Debug, Clone)]
/// Represent a number
pub struct Number {
    inner: Arc<Ratio<BigInt>>,
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

impl<T: Into<BigInt>> From<T> for Number {
    fn from(v: T) -> Self {
        let big = v.into();
        Self::new_unchecked(big, 1)
    }
}

impl From<&Self> for Number {
    fn from(s: &Self) -> Self {
        s.clone()
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::zero()
    }
}

impl Number {
    /// 0.0
    pub fn zero() -> Self {
        static ZERO: OnceCell<Number> = OnceCell::new();
        ZERO.get_or_init(|| Self::new_unchecked(0, 1)).clone()
    }

    /// 1.0
    pub fn one() -> Self {
        static ONE: OnceCell<Number> = OnceCell::new();
        ONE.get_or_init(|| Self::new_unchecked(1, 1)).clone()
    }

    /// 3.14159... ~= 104 348/33 215
    pub fn pi() -> Self {
        static PI: OnceCell<Number> = OnceCell::new();
        PI.get_or_init(|| Self::new_unchecked(104348, 33215))
            .clone()
    }
    ///
    /// 2.71828... ~= 2721 / 1001
    pub fn e() -> Self {
        static E: OnceCell<Number> = OnceCell::new();
        E.get_or_init(|| Self::new_unchecked(2721, 1001)).clone()
    }

    /// Minimum precision
    pub fn epsilon() -> Self {
        static EPSILON: OnceCell<Number> = OnceCell::new();
        EPSILON
            .get_or_init(|| Self::new_unchecked(1, 10000000))
            .clone()
    }

    /// Create a new number in the form `num / denom`
    /// This way we can safely create number can cannot be expressed in binary form like 0.1
    ///
    /// # Error
    /// Error::DivisionZero if `denom` is 0
    ///
    /// ```
    /// # use math::{Number, number::Radix};
    /// assert!(Number::new(43, 0).is_err());
    /// assert_eq!(Number::new(30, 10), Number::new(3, 1));
    /// assert_eq!(Number::new(2, 10), Number::new(1, 5));
    /// assert_eq!(Number::new(1, 10).unwrap().to_string(Radix::Dec, 5), "0.1");
    /// ```
    pub fn new(num: impl Into<BigInt>, denom: impl Into<BigInt>) -> Result<Self> {
        let denom = denom.into();
        if denom == num::zero() {
            return Err(Error::DivisionZero);
        }

        let num = num.into();

        if num == denom {
            return Ok(Self::one());
        }

        if num == num::zero() {
            return Ok(Self::zero());
        }

        Ok(Self {
            inner: Arc::new(Ratio::new(num.into(), denom.into())),
        })
    }

    /// Same as `Number::new` but bypass the zero check for denom
    pub fn new_unchecked(num: impl Into<BigInt>, denom: impl Into<BigInt>) -> Self {
        Self {
            inner: Arc::new(Ratio::new_raw(num.into(), denom.into())),
        }
    }

    /// Get the formatted string of a number
    ///
    /// ```
    /// # use math::number::{Number, Radix};
    /// let pi = Number::pi();
    /// let zero = Number::zero();
    /// let neg = Number::new_unchecked(-1, 10);
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
        let num = self.inner.abs();
        let whole = num.to_integer();
        let mut fract = num.fract();

        let mut res = match radix {
            Radix::Bin => format!("{:b}", whole),
            Radix::Oct => format!("{:o}", whole),
            Radix::Dec => format!("{}", whole),
            Radix::Hex => format!("{:X}", whole),
        };

        if self.inner.is_negative() {
            res.insert(0, '-');
        }

        if fract != num::zero() {
            let radix_len = match radix {
                Radix::Bin => 2u32,
                Radix::Oct => 8u32,
                Radix::Dec => 10u32,
                Radix::Hex => 16u32,
            };

            res.push('.');
            let mut cnt = 0;
            let big_radix = BigInt::from(radix_len);

            while fract != num::zero() && cnt < precision {
                let n = fract * &big_radix;
                let whole = n.to_integer().to_u32().unwrap();
                fract = n.fract();

                let mut ch = char::from_digit(whole, radix_len).unwrap();
                ch.make_ascii_uppercase();

                res.push(ch);
                cnt += 1;
            }

            res = res.trim_end_matches('0').trim_end_matches('.').to_owned();
        }

        res
    }
}

impl Number {
    /// Generate a random number in range of <0, 1>
    ///
    /// ```
    /// # use math::Number;
    /// assert!(Number::random() != Number::random());
    /// assert!(Number::random() >= Number::zero());
    /// assert!(Number::random() <= Number::one());
    /// ```
    pub fn random() -> Self {
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        let denom = rng.gen_range(1..u32::MAX);
        let num = rng.gen_range(0..=denom);

        Self::new_unchecked(num, denom)
    }

    /// Add two numbers together
    ///
    /// ```
    /// # use math::Number;
    /// # fn main() -> math::Result<()> {
    /// let a = Number::new(1, 10)?;
    /// let b = Number::new(2, 10)?;
    ///
    /// assert_eq!(a.add(&b)?, Number::new(3, 10)?);
    /// assert_eq!(b.add(&a), a.add(&b));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn add(&self, other: impl Into<Self>) -> Result<Self> {
        let rhs = other.into();

        if rhs == Self::zero() {
            return Ok(rhs);
        }

        if self == &Self::zero() {
            return Ok(self.clone());
        }

        let res = &*self.inner + &*rhs.inner;

        Ok(Self {
            inner: Arc::new(res),
        })
    }

    /// Subtract two numbers
    ///
    /// ```
    /// # use math::Number;
    /// # fn main() -> math::Result<()> {
    /// let a = Number::new(1, 10)?;
    /// let b = Number::new(2, 10)?;
    ///
    /// assert_eq!(a.sub(&b)?, Number::new(-1, 10)?);
    /// assert_eq!(b.sub(&a)?, Number::new(1, 10)?);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn sub(&self, other: impl Into<Self>) -> Result<Self> {
        let rhs = other.into();

        if rhs == Self::zero() {
            return Ok(self.clone());
        }

        let res = &*self.inner - &*rhs.inner;

        Ok(Self {
            inner: Arc::new(res),
        })
    }

    /// Multiply two numbers
    ///
    /// ```
    /// # use math::Number;
    /// # fn main() -> math::Result<()> {
    /// let a = Number::new(1, 10)?;
    /// let b = Number::new(2, 10)?;
    ///
    /// assert_eq!(a.mul(&b)?, Number::new(2, 100)?);
    /// assert_eq!(b.mul(&a), a.mul(&b));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn mul(&self, other: impl Into<Self>) -> Result<Self> {
        if self == &Self::zero() {
            return Ok(Self::zero());
        }

        let rhs = other.into();

        if rhs == Self::zero() {
            return Ok(Self::zero());
        }

        if self == &Self::one() {
            return Ok(rhs);
        }

        if rhs == Self::one() {
            return Ok(self.clone());
        }

        let res = &*self.inner * &*rhs.inner;

        Ok(Self {
            inner: Arc::new(res),
        })
    }

    /// Divide two numbers
    ///
    /// # Error
    /// Return Error::DivisionZero if `other` is 0
    ///
    /// ```
    /// # use math::Number;
    /// # fn main() -> math::Result<()> {
    /// let random = Number::random();
    /// assert!(random.div(Number::zero()).is_err());
    /// assert_eq!(Number::zero().div(&random)?, Number::zero());
    /// assert_eq!(random.div(&random)?, Number::one());
    ///
    /// let a = Number::from(3);
    /// let b = Number::new(3, 2)?;
    /// let neg_b = Number::new(-3, 2)?;
    /// assert_eq!(a.div(&b)?, Number::from(2));
    /// assert_eq!(a.div(&neg_b)?, Number::from(-2));
    /// #   Ok(())
    /// # }
    /// ```
    pub fn div(&self, other: impl Into<Self>) -> Result<Self> {
        let rhs = other.into();

        if rhs == Self::zero() {
            return Err(Error::DivisionZero);
        }

        if rhs == Self::one() {
            return Ok(self.clone());
        }

        let res = &*self.inner / &*rhs.inner;

        Ok(Self {
            inner: Arc::new(res),
        })
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// ```
    /// # use math::Number;
    /// assert_eq!(Number::random().power(Number::zero()), Ok(Number::one()));
    /// assert_eq!(Number::from(5).power(2), Ok(Number::from(25)));
    /// ```
    pub fn power(&self, exp: impl Into<Self>) -> Result<Self> {
        let exp = exp.into();

        if exp == Self::zero() {
            return Ok(Self::one());
        }

        if exp == Self::one() {
            return Ok(self.clone());
        }

        if exp == Self::from(-1) {
            return Self::one().div(self.clone());
        }

        let to_pow = exp
            .inner
            .numer()
            .to_i32()
            .ok_or_else(|| Error::Message(String::from("Exponent is too large")))?;
        let to_root = exp.inner.denom();

        let mut res = self.clone();

        if to_root != &num::one() {
            res = res.root(to_root.clone())?;
        }

        res.inner = Arc::new((*res.inner).pow(to_pow));

        Ok(res)
    }

    /// Get the modulo of `self / other`
    ///
    /// ```
    /// # use math::Number;
    /// assert_eq!(Number::from(5).modulo(2), Ok(Number::one()));
    /// assert_eq!(Number::from(-5).modulo(2), Ok(Number::one()));
    /// assert_eq!(Number::from(5).modulo(-2), Ok(Number::from(-1)));
    /// assert_eq!(Number::from(-5).modulo(-2), Ok(Number::from(-1)));
    /// assert_eq!(Number::from(-7).modulo(3), Ok(Number::from(2)));
    /// ```
    pub fn modulo(&self, other: impl Into<Self>) -> Result<Self> {
        let divisor = other.into();
        self.remainder(&divisor)?.add(&divisor)?.remainder(&divisor)
    }

    /// Get the remainder of `self / other`
    ///
    /// ```
    /// # use math::Number;
    /// assert_eq!(Number::from(5).modulo(2), Ok(Number::one()));
    /// assert_eq!(Number::from(5).modulo(-2), Ok(Number::from(-1)));
    /// assert_eq!(Number::from(-5).modulo(2), Ok(Number::from(1)));
    /// assert_eq!(Number::from(-7).modulo(3), Ok(Number::from(2)));
    /// assert_eq!(Number::from(7).modulo(-3), Ok(Number::from(-2)));
    /// ```
    pub fn remainder(&self, other: impl Into<Self>) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(&*self.inner % &*other.into().inner),
        })
    }

    /// Get the absolute value of the given number
    ///
    /// ```
    /// # use math::Number;
    /// let a = Number::random();
    /// let neg_a = a.mul(-1).unwrap();
    ///
    /// assert_eq!(a.abs(), Ok(a.clone()));
    /// assert_eq!(neg_a.abs(), Ok(a));
    /// ```
    pub fn abs(&self) -> Result<Self> {
        Ok(Self {
            inner: self.inner.abs().into(),
        })
    }

    /// Calculate factorial of a given number
    /// The number is not limited to integer, it can be a fraction
    ///
    /// # Error
    /// return Error::FactorialNegative if the number is less than 0
    ///
    /// ```
    /// # use math::Number;
    /// # use math::number::Radix;
    /// # fn main() -> math::Result<()> {
    /// assert_eq!(Number::zero().factorial()?, Number::one());
    /// assert_eq!(Number::from(5).factorial()?, Number::from(120));
    /// assert_eq!(Number::new(32, 10)?.factorial()?.to_string(Radix::Dec, 6), "7.75669");
    /// assert!(Number::from(-1).factorial().is_err());
    /// #     Ok(())
    /// # }
    /// ```
    pub fn factorial(&self) -> Result<Self> {
        if self.inner.is_negative() {
            return Err(Error::FactorialNegative);
        }

        if self == &Self::zero() {
            return Ok(Self::one());
        }

        if self == &Self::one() {
            return Ok(self.clone());
        }

        if self.inner.is_integer() {
            let mut res = Self::from(2);
            let to = Self::from(self.inner.numer().clone());
            let mut cnt = Self::from(3u64);
            while cnt <= to {
                res = res.mul(&cnt)?;
                cnt = cnt.add(1)?;
            }

            return Ok(res);
        }

        self.add(1)?.gamma()
    }

    /// Calculate gamma function
    pub fn gamma(&self) -> Result<Self> {
        let p = [
            Self::new_unchecked(9999999999998099i128, 10000000000000000i128),
            Self::new_unchecked(6765203681218851i128, 1000000000000000i128),
            Self::new_unchecked(-1259139216722289i128, 1000000000000000i128),
            Self::new_unchecked(7713234287776531i128, 10000000000000000i128),
            Self::new_unchecked(-1766150291621406i128, 10000000000000000i128),
            Self::new_unchecked(1250734327868691i128, 100000000000000000i128),
            Self::new_unchecked(-13857109526572012i128, 100000000000000000i128),
            Self::new_unchecked(9984369578019571i128, 1000000000000000000i128),
            Self::new_unchecked(15056327351493116i128, 100000000000000000000i128),
        ];

        let mut iter = p.into_iter().enumerate();
        let mut y = iter.next().unwrap().1;

        while let Some((i, val)) = iter.next() {
            y = y.add(val.div(self.add(i as i128)?.sub(1)?)?)?;
        }

        let t: Self = self.add(Self::new_unchecked(65, 10))?;
        let sqrt_2pi = Self::pi().mul(2)?.sqrt()?;

        let h = self.sub(Self::new_unchecked(1, 2))?;

        sqrt_2pi
            .mul(y)?
            .div(self.sqrt()?)?
            .mul(t.power(self.sub(h)?)?)?
            .mul(Self::e().power((t.mul(-1))?)?)
    }

    /// Returns the logarithm of the number with respect to an arbitrary `base`.
    ///
    /// # Error
    /// Error::LogUndefinedBase if the `base` is less or equal than 0
    /// Error::LogUndefinedNumber if the number is less or equal than 0
    ///
    /// ```
    /// # use math::Number;
    /// # fn main() -> math::Result<()> {
    /// assert!(Number::random().log(0).is_err());
    /// assert!(Number::random().log(Number::new_unchecked(-3,2)).is_err());
    /// assert!(Number::zero().log(Number::random()).is_err());
    /// assert!(Number::new(-3, 10)?.log(Number::random()).is_err());
    ///
    /// let a = Number::random();
    /// let base = Number::random();
    /// let b = Number::from(2);
    ///
    /// // Number same as base
    /// assert_eq!(a.log(&a)?, Number::one());
    /// // Product rule log(xy) == log(x) + log(y)
    /// assert_eq!(a.mul(&b)?.log(&base), a.log(&base)?.add(b.log(&base)?));
    /// // Quotient rule log(x/y) == log(x) - log(y)
    /// assert_eq!(a.div(&b)?.log(&base), a.log(&base)?.sub(b.log(&base)?));
    /// // Log of power log(x^y) == y * log(x)
    /// assert_eq!(a.power(&b)?.log(&base), b.mul(a.log(&base)?));
    /// // Log of one
    /// assert_eq!(Number::one().log(&base)?, Number::zero());
    /// // Log reciprocal log(1/x) = -ln(x);
    /// assert_eq!(Number::one().div(&a)?.log(&base), a.log(&base)?.mul(-1));
    /// # Ok(())
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
    /// # fn main() -> math::Result<()> {
    /// assert!(Number::random().root(0).is_err());
    /// assert!(Number::from(-1).root(2).is_err());
    /// assert!(Number::from(-1).root(88).is_err());
    /// assert!(Number::from(-1).root(3).is_ok());
    /// assert!(Number::from(-1).root(87).is_ok());
    ///
    /// let a = Number::random();
    /// let b = Number::random();
    /// let nth = Number::random();
    ///
    /// // root(a^nth) == a
    /// assert_eq!(a.power(&nth)?.root(&nth)?, a.clone());
    /// // root(ab) == root(a) * root(b)
    /// assert_eq!(a.mul(&b)?.root(&nth), a.root(&nth)?.mul(b.root(&nth)?));
    /// // root(a/b) == root(a) / root(b)
    /// assert_eq!(a.div(&b)?.root(&nth), a.root(&nth)?.div(b.root(&nth)?));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn root(&self, nth: impl Into<Self>) -> Result<Self> {
        let nth = nth.into();
        if nth == Self::zero() {
            return Err(Error::ZeroNthRoot);
        }

        if self == &Self::zero() {
            return Ok(Self::zero());
        }

        let to_root = nth.inner.numer();
        let to_pow = nth.inner.denom();

        let mut res: Self = self.clone();

        if to_root != &num::one() {
            let mut x_pre = Self::random().mul(10)?;
            let mut del_x = Self::one();
            let mut x_k = Self::zero();

            while del_x <= Self::epsilon() {
                x_k = Self::from(to_root - 1)
                    .mul(&x_pre)?
                    .add(self.div(&x_pre.power(to_root - 1)?)?)?
                    .div(to_root.clone())?;

                del_x = x_k.sub(x_pre)?.abs()?;
                x_pre = x_k.clone();
            }

            res = x_k;
        }

        if to_pow != &num::one() {
            res = res.power(to_pow.clone())?;
        }

        return Ok(res);
    }

    /// Returns the square root of a number.
    /// This function is the same as `root` with the `nth` of 2
    pub fn sqrt(&self) -> Result<Self> {
        self.root(2)
    }

    /// Computes the sine of a number (in radians).
    ///
    /// ```
    /// # use math::Number;
    /// # fn main() -> math::Result<()> {
    /// let x = Number::random();
    /// let half_pi = Number::pi().div(2)?;
    /// let sin_x = x.sin();
    ///
    /// // sin(x) == cos(PI/2 - x)
    /// assert_eq!(sin_x, half_pi.sub(x)?.cos());
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
    /// # fn main() -> math::Result<()> {
    /// let x = Number::random();
    /// let half_pi = Number::pi().div(2)?;
    /// let cos_x = x.cos();
    ///
    /// // cos(x) == sin(PI/2 - x)
    /// assert_eq!(cos_x, half_pi.sub(x)?.sin());
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
    /// # fn main() -> math::Result<()> {
    /// let x = Number::random();
    /// let tan_x = x.tg();
    ///
    /// // tg(x) == sin(x) / cos(x)
    /// assert_eq!(tan_x, x.sin()?.div(x.cos()?));
    /// // tg(x) == 1 / cotg(x)
    /// assert_eq!(tan_x, Number::one().div(x.cotg()?));
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
    /// # fn main() -> math::Result<()> {
    /// let x = Number::random();
    /// let cot_x = x.cotg();
    ///
    /// // cotg(x) == cos(x) / sin(x)
    /// assert_eq!(cot_x, x.cos()?.div(x.sin()?));
    /// // cotg(x) == 1 / tg(x)
    /// assert_eq!(cot_x, Number::one().div(x.tg()?));
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
    /// # fn main() -> math::Result<()> {
    /// assert!(Number::from(-2).arcsin().is_err());
    /// assert!(Number::from(2).arcsin().is_err());
    /// let x = Number::random();
    /// let sin_x = x.sin()?;
    ///
    /// assert_eq!(x, sin_x.arcsin()?);
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
    /// # fn main() -> math::Result<()> {
    /// assert!(Number::from(-2).arccos().is_err());
    /// assert!(Number::from(2).arccos().is_err());
    /// let x = Number::random();
    /// let cos_x = x.cos()?;
    ///
    /// assert_eq!(x, cos_x.arccos()?);
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
    /// # fn main() -> math::Result<()> {
    /// let x = Number::random();
    /// let tan_x = x.tg()?;
    ///
    /// assert_eq!(x, tan_x.arctg()?);
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
    /// # fn main() -> math::Result<()> {
    /// let x = Number::random();
    /// let cot_x = x.cotg()?;
    ///
    /// assert_eq!(x, cot_x.arccotg()?);
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
    /// # fn main() -> math::Result<()> {
    /// assert!(Number::combination(-1, Number::random()).is_err());
    /// assert!(Number::combination(Number::random(), -1).is_err());
    ///
    /// let n = Number::random();
    ///
    /// // if k > n => C(n, k) == 0
    /// assert_eq!(Number::combination(3, 4)?, Number::zero());
    /// // C(n, 0) == 1
    /// assert_eq!(Number::combination(&n, 0)?, Number::one());
    /// // C(n, n) == 1
    /// assert_eq!(Number::combination(&n, &n)?, Number::one());
    /// // C(n, 1) == n
    /// assert_eq!(Number::combination(&n, 1)?, n);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn combination(n: impl Into<Self>, k: impl Into<Self>) -> Result<Self> {
        todo!()
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
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
