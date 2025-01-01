use core::{
    fmt,
    iter::{Product, Sum},
    ops::*,
    str::FromStr,
};

// TODO decide on how `to_string_general` should work

/// A common trait for a special case of fixed point numbers in the form of all
/// fractional bits.
pub trait Fracint:
    Sized
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + core::hash::Hash
    + fmt::Display
    + fmt::Debug
    + FromStr
    + Neg<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Not<Output = Self>
    + BitOr<Output = Self>
    + BitOrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Sum
    + Product
{
    type Int: Sized
        + Clone
        + Copy
        + TryInto<u128, Error: fmt::Debug>
        + TryFrom<u128, Error: fmt::Debug>
        + fmt::Debug;

    /// The number of bits in this type
    const BITS: usize;
    /// The maximum value representable by `Self`
    const MAX: Self;
    /// The minimum value representable by `Self`
    const MIN: Self;
    /// For `fiN`, `ONE` and `MAX` are the same. Prefer to use `MAX` when
    /// wanting to emphasize the true numeric bounds or ordered maximum,
    /// and instead use `ONE` for numeric values.
    const ONE: Self;
    /// A numerical value of negative one.
    ///
    /// Note that `NEG_ONE` is not equal to `MIN`, but to `MIN + ULP`.
    /// The purpose of `NEG_ONE` in contrast to `MIN` is preventing
    /// certain overflows, such as `fiN::MIN.wrapping_mul(fiN::MIN)` or
    /// `fiN::wrapping_abs(fiN::MIN)`.
    const NEG_ONE: Self;
    /// One positive Unit in the Last Place
    const ULP: Self;
    /// Zero.
    const ZERO: Self;
    /// If this type is signed
    const SIGNED: bool;

    /// Casts from the `Self::Int` type
    fn from_int(x: Self::Int) -> Self;
    /// Casts to the `Self::Int` type
    fn as_int(self) -> Self::Int;

    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    fn is_negative(self) -> bool {
        self < Self::ZERO
    }
    fn is_positive(self) -> bool {
        self >= Self::ZERO
    }

    /// Returns a value representing the sign of `self`.
    ///
    /// - `fiN::NEG_ONE` if the value is negative
    /// - `fiN::ZERO` if the value is zero
    /// - `fiN::ONE` if the value is positive
    fn signum(self) -> Self {
        if self < Self::ZERO {
            Self::NEG_ONE
        } else if self == Self::ZERO {
            Self::ZERO
        } else {
            Self::ONE
        }
    }

    /// Wrapping absolute value of `self`.
    ///
    /// # Overflow behavior
    ///
    /// `Self::MIN.abs()` -> `Self::MIN`
    fn wrapping_abs(self) -> Self {
        if self.is_negative() {
            self.wrapping_neg()
        } else {
            self
        }
    }

    /// Returns a tuple of `self.wrapping_abs()` along with a boolean indicating
    /// whether an overflow happened.
    fn overflowing_abs(self) -> (Self, bool);

    /// Same as `overflowing_abs` except it returns `None` on overflow
    fn checked_abs(self) -> Option<Self> {
        match self.overflowing_abs() {
            (x, false) => Some(x),
            (_, true) => None,
        }
    }

    /// Saturating absolute value of `self`. It behaves the same way as
    /// `wrapping_abs` except `Self::MIN.saturating_abs()` -> `Self::ONE`
    fn saturating_abs(self) -> Self {
        self.checked_abs().unwrap_or(Self::ONE)
    }

    /// Wrapping negation of `self`.
    ///
    /// # Overflow behavior
    ///
    /// `Self::MIN.wrapping_neg()` -> `Self::MIN`
    fn wrapping_neg(self) -> Self;

    /// Returns a tuple of `self.wrapping_neg()` along with a boolean indicating
    /// whether an overflow happened.
    fn overflowing_neg(self) -> (Self, bool);

    /// Same as `overflowing_neg` except it returns `None` on overflow
    fn checked_neg(self) -> Option<Self> {
        match self.overflowing_neg() {
            (x, false) => Some(x),
            (_, true) => None,
        }
    }

    /// Saturating negation of `self`. It behaves the same way as `wrapping_neg`
    /// except `Self::MIN.saturating_neg()` -> `Self::ONE`
    fn saturating_neg(self) -> Self;

    /// Saturating inversion of `self`. It has the special cases
    ///
    /// - `fiN::MIN` => `-1`
    /// - `fiN::ZERO` => `iX::MAX`
    ///
    /// Note that it does not panic on zero.
    fn saturating_inv(self) -> Self::Int;

    /// Wrapping (modular) addition.
    ///
    /// # Overflow behavior
    ///
    /// If the numeric value of `self` added to `rhs` is more than `fiN::MAX`
    /// or less than `fiN::MIN` (if `fiN::MIN` should be avoided, use
    /// `fiN::NEG_ONE` as the minimum value instead), modular overflow will
    /// happen.
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Returns a tuple of `self.wrapping_add(rhs)` along with a boolean
    /// indicating whether an overflow happened.
    /// Note that if `self.wrapping_add(rhs) == fiN::MIN`, it is counted as
    /// overflow.
    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    /// Same as `overflowing_add` except it returns `None` on overflow
    fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.overflowing_add(rhs) {
            (v, false) => Some(v),
            (_, true) => None,
        }
    }

    /// Saturating addition. Saturates at the numeric bounds `fiN::NEG_ONE` and
    /// `fiN::ONE` instead of overflowing.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Wrapping (modular) subtraction.
    ///
    /// # Overflow behavior
    ///
    /// If the numeric value of `self` subtracted by `rhs` is more than
    /// `fiN::MAX` or less than `fiN::MIN` (if `fiN::MIN` should be avoided,
    /// use `fiN::NEG_ONE` as the minimum value instead), modular overflow
    /// will happen.
    fn wrapping_sub(self, rhs: Self) -> Self;

    /// Returns a tuple of `self.wrapping_sub(rhs)` along with a boolean
    /// indicating whether an overflow happened.
    /// Note that if `self.wrapping_sub(rhs) == fiN::MIN`, it is counted as
    /// overflow.
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    /// Same as `overflowing_sub` except it returns `None` on overflow
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.overflowing_sub(rhs) {
            (v, false) => Some(v),
            (_, true) => None,
        }
    }

    /// Saturating subtraction. Saturates at the numeric bounds `fiN::NEG_ONE`
    /// and `fiN::ONE` instead of overflowing.
    fn saturating_sub(self, rhs: Self) -> Self;

    /// Wrapping fracint multiplication.
    ///
    /// # Overflow Behavior
    ///
    /// There is only one case where overflow can occur:
    /// `fiN::MIN.wrapping_mul(fiN::MIN)` -> `fiN::MIN`.
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// Returns a tuple of `self.wrapping_mul(rhs)` along with a boolean
    /// indicating whether an overflow happened.
    ///
    /// Note that only the overflow possible is the corner case
    /// `fiN::MIN.wrapping_mul(fiN::MIN)` -> `fiN::MIN`.
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    /// Same as `overflowing_mul` except it returns `None` on overflow
    fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (v, false) => Some(v),
            (_, true) => None,
        }
    }

    /// Saturating fracint multiplication. Saturates at the numeric bounds
    /// `fiN::NEG_ONE` and `fiN::ONE` instead of overflowing.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Saturating fracint multiplication with an integer. Saturates at the
    /// numeric bounds `fiN::NEG_ONE` and `fiN::ONE` instead of overflowing.
    fn saturating_mul_int(self, rhs: Self::Int) -> Self;

    /// Saturating fracint division. Saturates at the numeric bounds
    /// `fiN::NEG_ONE` and `fiN::ONE` instead of overflowing.
    ///
    /// Panics are prevented and saturation handled in this way:
    /// - if `rhs == fiN::ZERO`, `self.signum()` is returned
    /// - else if `self.saturating_abs() >= rhs.saturating_abs()`, it will
    ///   return `fiN::NEG_ONE` if their signs are not equal, `fiN::ONE`
    ///   otherwise (except for `rhs == fiN::ZERO` as shown above)
    ///
    /// Note that it does not panic on zero
    fn saturating_div(self, rhs: Self) -> Self;

    /// Saturating fracint division with an integer. Saturates at the numeric
    /// bounds `fiN::NEG_ONE` and `fiN::ONE` instead of overflowing.
    ///
    /// Panics are prevented and saturation handled in this way:
    /// - if `rhs == fiN::ZERO`, `self.signum()` is returned
    /// - if `self == fiN::MIN` and `rhs == -1`, `Self::ONE` is returned
    fn saturating_div_int(self, rhs: Self::Int) -> Self;

    #[cfg(feature = "rand_support")]
    fn rand<R: rand_core::RngCore + ?Sized>(rng: &mut R) -> Result<Self, rand_core::Error>;

    /// Converts from an `f32` to `Self`. Returns `None` if the absolute value
    /// is greater than 1.0.
    fn from_f32(f: f32) -> Option<Self>;

    /// Converts from an `f64` to `Self`. Returns `None` if the absolute value
    /// is greater than 1.0.
    fn from_f64(f: f64) -> Option<Self>;

    /// Converts to an `f32`
    fn to_f32(self) -> f32;

    /// Converts to an `f64`
    fn to_f64(self) -> f64;

    /// Slow way of calculating the truncated square root using bisection. This
    /// will always underestimate the square root, with about as much bit error
    /// as the number of leading zero bits.
    fn sqrt_simple_bisection(self) -> Self {
        let mut set_bit = if Self::SIGNED {
            Self::ULP << (Self::BITS - 2)
        } else {
            Self::ULP << (Self::BITS - 1)
        };
        let mut res = Self::ZERO;
        let mut prev_sqr = Self::ZERO;
        loop {
            if set_bit.is_zero() {
                break res
            }
            let test = res | set_bit;
            let sqr = test.saturating_mul(test);
            if sqr <= self {
                res = test;
                if sqr == self {
                    break res
                }
            }
            if sqr == prev_sqr {
                // we have reached a point where the set bit no longer contributes
                break res
            }
            prev_sqr = sqr;
            set_bit >>= 1;
        }
    }

    /// Fast way of calculating the square root to within a few ULPs of the true
    /// value.
    fn sqrt_fast(self) -> Self;
}

pub trait FracintDouble: Fracint {
    /// The double-sized version of `Self`
    type Double: Fracint + FracintHalf<Half = Self>;

    /// Widens `self` by extending with zero bits
    fn widen(self) -> Self::Double;

    /// Saturating widening multiplication of `self` and `other`. All inputs
    /// result in numerically _exact_ outputs, except for
    /// `fiN::MIN.full_mul(fiN::MIN)` which has to be saturated to `fiM::ONE`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fracints::*;
    ///
    /// // note: `from_str` and `to_string` sometimes have small
    /// // round-to-even errors when converting and displaying, but
    /// // the `full_mul` between those functions is truly exact
    /// // (except for the corner case).
    /// assert_eq!(
    ///     fi32!(0.123456789)
    ///         .saturating_widening_mul(fi32!(0.123456789))
    ///         .to_string(),
    ///     "0.0152415787947921023"
    /// );
    ///
    /// // the only overflow corner case
    /// assert_eq!(fi32::MIN.saturating_widening_mul(fi32::MIN), fi64::ONE);
    /// ```
    fn saturating_widening_mul(self, rhs: Self) -> Self::Double;

    /// This calculates a nearly exact truncated square root using bisection
    /// with a larger type
    fn sqrt_slow(self) -> Self {
        self.widen().sqrt_simple_bisection().truncate()
    }
}

pub trait FracintHalf: Fracint + From<Self::Half> {
    /// The half-sized version of `Self`
    type Half: Fracint + FracintDouble<Double = Self>;

    /// Returns half sized low and high parts. The high part is effectively a
    /// truncated version of `self`, and the low part is the remainder.
    fn split(self) -> (Self::Half, Self::Half);

    /// Truncates `self` down to a half sized type with the same value except
    /// with less than 1 ULP of error.
    fn truncate(self) -> Self::Half;
}
