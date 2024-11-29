#[macro_export]
macro_rules! impl_signed {
    (
        $ty:ident,
        $s:expr,
        $iX:ident,
        $uX:ident,
        $to_string:ident,
        $from_str:ident,
        $normalized_mul:expr,
        $normalized_div:expr,
        $c:expr
    ) => {
        #[allow(non_camel_case_types)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $ty(pub $iX);

        impl $ty {
            /// The number of bits in this type
            pub const BITS: usize = $uX::BITS as usize;
            /// The maximum value representable by a `fiN``
            pub const MAX: $ty = $ty($iX::MAX);
            /// The minimum value representable by a `fiN`
            pub const MIN: $ty = $ty($iX::MIN);
            /// A numerical value of negative one.
            ///
            /// Note that `NEG_ONE` is not equal to `MIN`, but to `MIN + ULP`.
            /// The purpose of `NEG_ONE` in contrast to `MIN` is preventing
            /// certain overflows, such as `fiN::MIN.wrapping_mul(fiN::MIN)` or
            /// `fiN::wrapping_abs(fiN::MIN)`.
            pub const NEG_ONE: $ty = $ty(-$iX::MAX);
            /// For `fiN`, `ONE` and `MAX` are the same. Prefer to use `MAX` when
            /// wanting to emphasize the true numeric bounds or ordered maximum,
            /// and instead use `ONE` for numeric values.
            pub const ONE: $ty = $ty($iX::MAX);
            /// One positive Unit in the Last Place
            pub const ULP: $ty = $ty(1);
            /// Zero.
            pub const ZERO: $ty = $ty(0);

            // TODO decide on how `to_string_general` should work

            pub fn is_negative(self) -> bool {
                self < $ty::ZERO
            }

            pub fn is_positive(self) -> bool {
                self >= $ty::ZERO
            }

            /// Returns a value representing the sign of `self`.
            ///
            /// - `fiN::NEG_ONE` if the value is negative
            /// - `fiN::ZERO` if the value is zero
            /// - `fiN::ONE` if the value is positive
            pub fn signum(self) -> Self {
                if self < $ty::ZERO {
                    $ty::NEG_ONE
                } else if self == $ty::ZERO {
                    $ty::ZERO
                } else {
                    $ty::ONE
                }
            }

            /// Wrapping absolute value of `self`.
            ///
            /// # Overflow behavior
            ///
            /// `fiN::MIN.abs()` -> `fiN::MIN`
            pub fn wrapping_abs(self) -> Self {
                if self.is_negative() {
                    self.wrapping_neg()
                } else {
                    self
                }
            }

            /// Returns a tuple of `self.wrapping_abs()` along with a boolean indicating
            /// whether an overflow happened.
            pub fn overflowing_abs(self) -> (Self, bool) {
                (self.wrapping_abs(), self == $ty::MIN)
            }

            /// Same as `overflowing_abs` except it returns an `Option<fiN>`, where
            /// `Some(fiN)` means no overflow and `None` means overflow.
            pub fn checked_abs(self) -> Option<Self> {
                match self.overflowing_abs() {
                    (x, false) => Some(x),
                    (_, true) => None,
                }
            }

            /// Saturating absolute value of `self`. It behaves the same way as
            /// `wrapping_abs` except `fiN::MIN.saturating_abs()` -> `fiN::ONE`
            pub fn saturating_abs(self) -> Self {
                self.checked_abs().unwrap_or($ty::ONE)
            }

            /// Wrapping negation of `self`.
            ///
            /// # Overflow behavior
            ///
            /// `fiN::MIN.wrapping_neg()` -> `fiN::MIN`
            pub fn wrapping_neg(self) -> Self {
                $ty(self.0.wrapping_neg())
            }

            /// Returns a tuple of `self.wrapping_neg()` along with a boolean indicating
            /// whether an overflow happened.
            pub fn overflowing_neg(self) -> (Self, bool) {
                (self.wrapping_neg(), self == $ty::MIN)
            }

            /// Same as `overflowing_neg` except it returns an `Option<fiN>`, where
            /// `Some(fiN)` means no overflow and `None` means overflow.
            pub fn checked_neg(self) -> Option<Self> {
                match self.overflowing_neg() {
                    (x, false) => Some(x),
                    (_, true) => None,
                }
            }

            /// Saturating negation of `self`. It behaves the same way as `wrapping_neg`
            /// except `fiN::MIN.saturating_neg()` -> `fiN::ONE`
            pub fn saturating_neg(self) -> Self {
                self.checked_neg().unwrap_or($ty::ONE)
            }

            /// Wrapping reciprocal of `self`.
            ///
            /// # Note
            ///
            /// The sign bit is preserved except for the corner case:
            /// `fiN::MIN.wrapping_inv()` -> `iX::ZERO`
            pub fn wrapping_inv(self) -> $iX {
                $iX::MAX / self.0
            }

            /// Returns a tuple of `self.wrapping_inv()` along with a boolean indicating
            /// whether an overflow happened.
            pub fn overflowing_inv(self) -> ($iX, bool) {
                ($iX::MAX / self.0, self == $ty::ZERO)
            }

            /// Same as `overflowing_inv` except it returns an `Option<iX>`, where
            /// `Some(iX)` means no overflow and `None` means overflow.
            pub fn checked_inv(self) -> Option<$iX> {
                match self.overflowing_inv() {
                    (x, false) => Some(x),
                    (_, true) => None,
                }
            }

            /// Saturating negation of `self`. It behaves the same way as `wrapping_inv`
            /// except `fiN::MIN.saturating_inv()` -> `-1`
            pub fn saturating_inv(self) -> $iX {
                self.checked_inv().unwrap_or(-1)
            }

            // TODO
            /*
            /// Computes the square root of `self.saturating_abs()`
            pub fn saturating_abs_sqrt(&self) -> Self {
                let half_x = *self >> 1;
                let prev_conv = self;
                let conv = (*self >> 1) + ($ty::ONE >> 1);
                while (conv - prev_conv).abs() < ___ {
                    conv = (conv >> 1) + self.wrapping_div(conv);
                }
            }

            //
            /// Wrapping square root.
            ///
            /// # Panics
            ///
            /// If `self.is_negative` then the function panics.
            pub fn wrapping_sqrt(&self) -> Self {}

            /// Returns a tuple of `self.wrapping_sqrt(other)` along with a boolean indicating whether an overflow happened.
            pub fn overflowing_sqrt(&self) -> Self {}

            /// Same as `overflowing_sqrt` except it returns an `Option<fiN>`, where `Some(fiN)` means no overflow and `None` means overflow.
            pub fn checked_sqrt(self) -> Option<Self> {
                match self.overflowing_sqrt() {
                    (x, false) => Some(x),
                    (_, true) => None,
                }
            }
            */

            /// Wrapping (modular) addition.
            ///
            /// # Overflow behavior
            ///
            /// If the numeric value of `self` added to `other` is more than `fiN::MAX`
            /// or less than `fiN::MIN` (if `fiN::MIN` should be avoided, use
            /// `fiN::NEG_ONE` as the minimum value instead), modular overflow will
            /// happen.
            pub fn wrapping_add(self, other: Self) -> Self {
                $ty(self.0.wrapping_add(other.0))
            }

            /// Returns a tuple of `self.wrapping_add(other)` along with a boolean
            /// indicating whether an overflow happened.
            /// Note that if `self.wrapping_add(other) == fiN::MIN`, it is counted as
            /// overflow.
            pub fn overflowing_add(self, other: Self) -> (Self, bool) {
                let (internal, overflow) = self.0.overflowing_add(other.0);
                if internal == $iX::MIN {
                    ($ty(internal), true)
                } else {
                    ($ty(internal), overflow)
                }
            }

            /// Same as `overflowing_add` except it returns an `Option<fiN>`, where
            /// `Some(fiN)` means no overflow and `None` means overflow.
            pub fn checked_add(self, other: Self) -> Option<Self> {
                match self.overflowing_add(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating addition. Saturates at the numeric bounds `fiN::NEG_ONE` and
            /// `fiN::ONE` instead of overflowing.
            pub fn saturating_add(self, other: Self) -> Self {
                // note that $ty::MAX added to 0 does not overflow but $ty::MIN added to 0 does
                // overflow, and $ty::MIN.wrapping_add($ty::ZERO) and
                // $ty::ZERO.wrapping_add($ty::MIN) both overflow which means that
                // `other <= $ty::ZERO` catches all the cases
                self.checked_add(other).unwrap_or_else(|| {
                    if other <= $ty::ZERO {
                        $ty::NEG_ONE
                    } else {
                        $ty::ONE
                    }
                })
            }

            /// Wrapping (modular) subtraction.
            ///
            /// # Overflow behavior
            ///
            /// If the numeric value of `self` subtracted by `other` is more than
            /// `fiN::MAX` or less than `fiN::MIN` (if `fiN::MIN` should be avoided,
            /// use `fiN::NEG_ONE` as the minimum value instead), modular overflow
            /// will happen.
            pub fn wrapping_sub(self, other: Self) -> Self {
                $ty(self.0.wrapping_sub(other.0))
            }

            /// Returns a tuple of `self.wrapping_sub(other)` along with a boolean
            /// indicating whether an overflow happened.
            /// Note that if `self.wrapping_sub(other) == fiN::MIN`, it is counted as
            /// overflow.
            pub fn overflowing_sub(self, other: Self) -> (Self, bool) {
                let (internal, overflow) = self.0.overflowing_sub(other.0);
                if internal == $iX::MIN {
                    ($ty(internal), true)
                } else {
                    ($ty(internal), overflow)
                }
            }

            /// Same as `overflowing_sub` except it returns an `Option<fiN>`, where
            /// `Some(fiN)` means no overflow and `None` means overflow.
            pub fn checked_sub(self, other: Self) -> Option<Self> {
                match self.overflowing_sub(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating subtraction. Saturates at the numeric bounds `fiN::NEG_ONE`
            /// and `fiN::ONE` instead of overflowing.
            pub fn saturating_sub(self, other: Self) -> Self {
                self.checked_sub(other).unwrap_or_else(|| {
                    if other <= $ty::ZERO {
                        $ty::ONE
                    } else {
                        $ty::NEG_ONE
                    }
                })
            }

            /// Wrapping fracint multiplication.
            ///
            /// # Overflow Behavior
            ///
            /// There is only one case where overflow can occur:
            /// `fiN::MIN.wrapping_mul(fiN::MIN)` -> `fiN::MIN`.
            pub fn wrapping_mul(self, other: Self) -> Self {
                $ty($normalized_mul(self.0, other.0))
            }

            /// Returns a tuple of `self.wrapping_mul(other)` along with a boolean
            /// indicating whether an overflow happened.
            ///
            /// Note that only the overflow possible is the corner case
            /// `fiN::MIN.wrapping_mul(fiN::MIN)` -> `fiN::MIN`.
            pub fn overflowing_mul(self, other: Self) -> (Self, bool) {
                (
                    self.wrapping_mul(other),
                    self == $ty::MIN && other == $ty::MIN,
                )
            }

            /// Same as `overflowing_mul` except it returns an `Option<fiN>`, where
            /// `Some` means no overflow and `None` means overflow.
            pub fn checked_mul(self, other: Self) -> Option<Self> {
                match self.overflowing_mul(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating fracint multiplication. Saturates at the numeric bounds
            /// `fiN::NEG_ONE` and `fiN::ONE` instead of overflowing.
            pub fn saturating_mul(self, other: Self) -> Self {
                if self == $ty::MIN && other == $ty::MIN {
                    $ty::ONE
                } else {
                    self.wrapping_mul(other)
                }
            }

            /// Wrapping fracint division.
            /// It is strongly recommended to use `saturating_div` instead unless all of
            /// the invariants can be upheld
            ///
            /// # Overflow Behavior
            ///
            /// Overflow will happen if `self.saturating_abs() >=
            /// other.saturating_abs()`. TODO test this extensively Note that
            /// fiN::MIN can be produced if `self` and `other` are not equal in sign
            /// but equal in absolute value.
            ///
            /// # Panics
            ///
            /// This function will panic if `other` is `fiN::ZERO`.
            pub fn wrapping_div(self, other: Self) -> Self {
                if other == $ty::ZERO {
                    panic!("division by zero");
                }
                $ty($normalized_div(self.0, other.0))
            }

            /// Returns a tuple of `self.wrapping_div(other)` (with exceptions for
            /// division by zero) along with a boolean indicating whether an
            /// overflow happened. If `other == fiN::ZERO`, `(self.signum(),true)`
            /// is returned.
            pub fn overflowing_div(self, other: Self) -> (Self, bool) {
                if other == $ty::ZERO {
                    (self.signum(), true)
                } else {
                    (
                        self.wrapping_div(other),
                        self.saturating_abs() >= other.saturating_abs(),
                    )
                }
            }

            /// Same as `overflowing_div` except it returns an `Option<fiN>`, where
            /// `Some(fiN)` means no overflow and `None` means overflow or divide by
            /// zero.
            pub fn checked_div(self, other: Self) -> Option<Self> {
                match self.overflowing_div(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating fracint division. Saturates at the numeric bounds
            /// `fiN::NEG_ONE` and `fiN::ONE` instead of overflowing.
            ///
            /// Panics are prevented and saturation handled in this way:
            /// - if `other == fiN::ZERO`, `self.signum()` is returned
            /// - else if `self.saturating_abs() >= other.saturating_abs()`, it will
            ///   return `fiN::NEG_ONE` if their signs are not equal, `fiN::ONE`
            ///   otherwise (except for `other == fiN::ZERO` as shown above)
            /// - else it will return `self.wrapping_div(other)`
            pub fn saturating_div(self, other: Self) -> Self {
                if other == $ty::ZERO {
                    self.signum()
                } else if self.saturating_abs() >= other.saturating_abs() {
                    //self == other == 0 eliminated above
                    if (self < $ty::ZERO) != (other < $ty::ZERO) {
                        $ty::NEG_ONE
                    } else {
                        $ty::ONE
                    }
                } else {
                    self.wrapping_div(other)
                }
            }

            /*
            /// Wrapping fracint remainder.
            ///
            /// The current implementation is not final. (see issue TODO)
            ///
            /// # Overflow Behavior
            ///
            /// `fiN::MIN.wrapping_rem(fiN::ULP.wrapping_neg()) -> fiN::ZERO`
            ///
            /// # Panics
            ///
            /// This function will panic if `other` is 0.
            pub fn wrapping_rem(self, other: Self) -> Self {
                $ty(self.0 % other.0)
            }*/

            /*
            /// Returns a tuple of `self.wrapping_rem(other)` (with exceptions for
            /// `other == fiN::ZERO`) along with a boolean indicating whether an overflow happened.
            /// If `other == fiN::ZERO`, `(fiN::ZERO,true)` is returned.
            /// The current implementation is not final. (see issue TODO)
            pub fn overflowing_rem(self, other: Self) -> (Self, bool) {
                if other == $ty::ZERO {
                    ($ty::ZERO, true)
                } else {
                    (
                        self.wrapping_rem(other),
                        (self == $ty::MIN) && (other == ($ty::ULP.wrapping_neg())),
                    )
                }
            }

            /// Same as `overflowing_rem` except it returns an `Option<fiN>`, where `Some(fiN)`
            /// means no overflow and `None` means overflow or divide by zero.
            /// The current implementation is not final. (see issue TODO)
            pub fn checked_rem(self, other: Self) -> Option<Self> {
                match self.overflowing_rem(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating remainder. Saturates at the numeric bounds `fiN::NEG_ONE` and `fiN::ONE`
            /// instead of overflowing.
            ///
            /// Panics are prevented and saturation handled in this way:
            ///     *if `other == fiN::ZERO`, `fiN::ZERO` is returned
            /// The current implementation is not final. (see issue TODO)
            pub fn saturating_rem(self, other: Self) -> Self {
                if other == $ty::ZERO {
                    $ty::ZERO
                } else {
                    self.wrapping_rem(other)
                }
            }*/

            /// Intended to only be called for `-0.5 <= self <= 0.5`. This function has
            /// overflows outside of this range.
            fn cos_taudiv4_taylor_base(self) -> $ty {
                // This is based on the equation
                // cos((tau/4) * x) =
                // ( (4/tau)^2 - (x^2)/2 ) / (4/tau)^2
                // + x^4 / ( (4/tau)^4 * 4! )
                // - x^6 / ( (4/tau)^6 * 6! )
                // + x^8 / ( (4/tau)^8 * 8! )
                // - ...

                let cutoff = (self.0 as $uX) >> ($ty::BITS / 2);
                if cutoff == 0 || cutoff == ($uX::MAX >> ($ty::BITS / 2)) {
                    return $ty::ONE;
                }

                let theta_sqr = self * self;
                let mut sum = $c.num_4divtau_sqr - (theta_sqr / 2);
                // overflow cannot happen due to the cutoff around zero
                sum /= $c.num_4divtau_sqr;
                // this completes the first term, which has to be computed differently from the
                // others

                let mut factorial_num: $iX = 2;
                let mut factorial_mul: $iX = 2;
                let mut numerator = theta_sqr;
                let mut num_4divtau_mul = $c.num_4divtau_sqr;
                for i in 0..$c.cos_taylor_iters {
                    // update multipliers
                    num_4divtau_mul *= $c.num_4divtau_sqr;
                    numerator *= theta_sqr;
                    factorial_num += 1;
                    factorial_mul *= factorial_num;
                    factorial_num += 1;
                    factorial_mul *= factorial_num;
                    // The strategy is to first divide the numerator by the factorial, then
                    // divide by the fiN.
                    let temp0 = numerator / factorial_mul;
                    if temp0 == $ty::ZERO {
                        return sum;
                    }
                    let temp1 = temp0 / num_4divtau_mul;
                    if (i & 0b1) == 0 {
                        sum += temp1;
                    } else {
                        sum -= temp1;
                    }
                }
                sum
            }

            /// Intended to only be called for `-0.5 <= theta <= 0.5`. This function has
            /// overflows outside of this range.
            fn sin_taudiv4_taylor_base(self) -> $ty {
                // This is based on the equation
                // sin((tau/4) * x) =
                // ( x*((4/tau)^2) - (x^3)/6 ) / (4/tau)^3
                // + x^5 / ( (4/tau)^5 * 5! )
                // - x^7 / ( (4/tau)^7 * 7! )
                // + x^9 / ( (4/tau)^9 * 9! )
                // - ...

                // Early return. The numerator has two `x` in it, so no other checking.
                if self == $ty::ZERO {
                    return $ty::ZERO;
                }
                let theta_sqr = self * self;
                let mut numerator = theta_sqr * self;
                let mut sum = (self * $c.num_4divtau_sqr) - (numerator / 6);
                let mut num_4divtau_mul = $c.num_4divtau_sqr * $c.num_4divtau;
                // `sum` cannot be more than `num_4divtau_mul`, assuming the range above
                sum /= num_4divtau_mul;
                // this completes the first term, which has to be computed differently from the
                // others

                let mut factorial_num: $iX = 3;
                let mut factorial_mul: $iX = 6;
                for i in 0..$c.sin_taylor_iters {
                    // update multipliers
                    num_4divtau_mul *= $c.num_4divtau_sqr;
                    numerator *= theta_sqr;
                    factorial_num += 1;
                    factorial_mul *= factorial_num;
                    factorial_num += 1;
                    factorial_mul *= factorial_num;

                    let temp0 = numerator / factorial_mul;
                    if temp0 == $ty::ZERO {
                        return sum;
                    }
                    let temp1 = temp0 / num_4divtau_mul;
                    if (i & 0b1) == 0 {
                        sum += temp1;
                    } else {
                        sum -= temp1;
                    }
                }
                sum
            }

            /// Calculates `cos((tau/4) * theta)` or `cos((pi/2) * theta)`.
            /// By having a (tau/4) constant and cleverly rearranging the taylor series,
            /// this provides a basic way to calculate cosine for fracints.
            /// Max Error:
            /// TODO ULPS
            pub fn cos_taudiv4_taylor(self) -> $ty {
                if self >= ($ty::MIN / -2) {
                    -self.wrapping_add($ty::MIN).sin_taudiv4_taylor_base()
                } else if self >= ($ty::MIN / 2) {
                    self.cos_taudiv4_taylor_base()
                } else {
                    self.wrapping_sub($ty::MIN).sin_taudiv4_taylor_base()
                }
            }

            /// Calculates `sin((tau/4) * theta)` or `sin((pi/2) * theta)`.
            /// By having a (tau/4) constant and cleverly rearranging the taylor series,
            /// this provides a basic way to calculate sine for fracints.
            /// Max Error:
            /// TODO ULPS
            pub fn sin_taudiv4_taylor(self) -> $ty {
                if self >= ($ty::MIN / -2) {
                    self.wrapping_add($ty::MIN).cos_taudiv4_taylor_base()
                } else if self >= ($ty::MIN / 2) {
                    self.sin_taudiv4_taylor_base()
                } else {
                    -self.wrapping_sub($ty::MIN).cos_taudiv4_taylor_base()
                }
            }

            pub fn cos_sin_pi_taylor(self) -> ($ty, $ty) {
                // this compares the highest two bits of `self` offset by a eighth of a circle
                // to determine which combination to use.
                let o = self.wrapping_add($ty::MIN / -4).0 as $uX;
                match (
                    (o & (1 << ($ty::BITS - 2))) != 0,
                    (o & (1 << ($ty::BITS - 1))) != 0,
                ) {
                    (false, false) => {
                        let t = self * 2;
                        (t.cos_taudiv4_taylor_base(), t.sin_taudiv4_taylor_base())
                    }
                    (true, false) => {
                        let t = self.wrapping_sub($ty::MIN / -2) * 2;
                        (-t.sin_taudiv4_taylor_base(), t.cos_taudiv4_taylor_base())
                    }
                    (false, true) => {
                        let t = self.wrapping_add($ty::MIN) * 2;
                        (-t.cos_taudiv4_taylor_base(), -t.sin_taudiv4_taylor_base())
                    }
                    (true, true) => {
                        let t = self.wrapping_add($ty::MIN / -2) * 2;
                        (t.sin_taudiv4_taylor_base(), -t.cos_taudiv4_taylor_base())
                    }
                }
            }

            // TODO
            pub fn bezerp(bez: &[$ty], t: &$ty) -> $ty {
                let mut temp1 = bez.to_vec();
                let mut temp0: Vec<$ty>;
                loop {
                    temp0 = temp1;
                    temp1 = Vec::new();
                    for i in 0..(temp0.len() - 1) {
                        temp1.push((($ty::ONE - *t) * temp0[i]) + (*t * temp0[i + 1]));
                    }
                    if temp1.len() == 1 {
                        break temp1[0].clone();
                    }
                }
            }

            /// TODO
            /// bez.len() must == weight.len()
            pub fn rational_bezerp(bez: &[$ty], weight: &[$ty], t: &$ty) -> $ty {
                let mut temp1 = bez.to_vec();
                let mut temp0: Vec<$ty>;
                let mut temp_weight1 = weight.to_vec();
                let mut temp_weight0: Vec<$ty>;
                loop {
                    temp0 = temp1;
                    temp1 = Vec::new();
                    temp_weight0 = temp_weight1;
                    temp_weight1 = Vec::new();
                    for i in 0..(temp0.len() - 1) {
                        let weight =
                            (($ty::ONE - *t) * temp_weight0[i]) + (*t * temp_weight0[i + 1]);
                        temp_weight1.push(weight);
                        temp1.push(
                            ((($ty::ONE - *t) * temp0[i]) + (*t * temp0[i + 1]))
                                .saturating_div(weight),
                        );
                    }
                    if temp1.len() == 1 {
                        break temp1.clone()[0];
                    }
                }
            }
        }

        impl fmt::Display for $ty {
            /// Converts to a base 10 string representation
            ///
            /// `fiN::ONE` and `fiN::NEG_ONE` are special cased to "1.0" and "-1.0"
            /// respectively.
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // TODO use constant sized buffers
                write!(f, "{}", $to_string(self.0))
            }
        }

        impl FromStr for $ty {
            type Err = FracintSerdeError;

            /// Conversion from a string representation.
            ///
            /// The input can start with a '-' to make the output negative. Then it can
            /// optionally start with a "0b", "0o", or a "0x" prefix to use radix 2, 8,
            /// or 16 respectively, otherwise it is parsed as radix 10. There must
            /// be an integer part with digits in the correct radix. If including
            /// the fraction, a '.' followed by one more digits in the correct radix
            /// should be added. Finally, an exponent can be added by 'e' or 'p'
            /// (except radix 16 which must use 'p') and then a number in the same
            /// radix is used. The exponent is applied as `* radix^exponent` before
            /// round-to-even. '_'s can be used throughout the integer, fraction,
            /// and exponent parts as long as one term is not all underspaces.
            ///
            /// `s` can be arbitrarily long but significance changes stops after a
            /// number of chars. TODO this is always constant with respect to
            /// radix?.
            ///
            /// The number must be in the range `(-1.0,1.0)` or else an overflow error
            /// is returned. 1.0 is special cased to map to `fiN::ONE' even though
            /// it is not exactly representable, and -1.0 is special cased to map to
            /// `fiN::NEG_ONE` to avoid introducing `fiN::MIN`.
            ///
            /// See the [FracintParseError] documentation for parsing errors and
            /// examples.
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $from_str(s).map(|x| Self(x))
            }
        }

        #[cfg(not(debug_assertions))]
        impl Neg for $ty {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::wrapping_neg(self)
            }
        }

        #[cfg(debug_assertions)]
        impl Neg for $ty {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::checked_neg(self).unwrap()
            }
        }

        #[cfg(not(debug_assertions))]
        impl Add for $ty {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self::wrapping_add(self, other)
            }
        }

        #[cfg(debug_assertions)]
        impl Add for $ty {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self::checked_add(self, other).unwrap()
            }
        }

        impl AddAssign for $ty {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        #[cfg(not(debug_assertions))]
        impl Sub for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::wrapping_sub(self, rhs)
            }
        }

        #[cfg(debug_assertions)]
        impl Sub for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::checked_sub(self, rhs).unwrap()
            }
        }

        impl SubAssign for $ty {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        #[cfg(not(debug_assertions))]
        impl Mul for $ty {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                Self::wrapping_mul(self, other)
            }
        }

        #[cfg(debug_assertions)]
        impl Mul for $ty {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                Self::checked_mul(self, other).unwrap()
            }
        }

        #[cfg(not(debug_assertions))]
        impl Mul<$iX> for $ty {
            type Output = Self;

            fn mul(self, other: $iX) -> Self::Output {
                $ty(self.0.wrapping_mul(other))
            }
        }

        #[cfg(debug_assertions)]
        impl Mul<$iX> for $ty {
            type Output = Self;

            fn mul(self, other: $iX) -> Self::Output {
                $ty(self.0.checked_mul(other).unwrap())
            }
        }

        impl MulAssign for $ty {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<$iX> for $ty {
            fn mul_assign(&mut self, rhs: $iX) {
                *self = *self * rhs;
            }
        }

        #[cfg(not(debug_assertions))]
        impl Div for $ty {
            type Output = $ty;

            fn div(self, rhs: Self) -> Self {
                Self::wrapping_div(self, rhs)
            }
        }

        #[cfg(debug_assertions)]
        impl Div for $ty {
            type Output = $ty;

            fn div(self, rhs: Self) -> Self {
                Self::checked_div(self, rhs).unwrap()
            }
        }

        #[cfg(not(debug_assertions))]
        impl Div<$iX> for $ty {
            type Output = Self;

            fn div(self, other: $iX) -> Self::Output {
                $ty(self.0.wrapping_div(other))
            }
        }

        #[cfg(debug_assertions)]
        impl Div<$iX> for $ty {
            type Output = Self;

            fn div(self, other: $iX) -> Self::Output {
                $ty(self.0.checked_div(other).unwrap())
            }
        }

        impl DivAssign for $ty {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<$iX> for $ty {
            fn div_assign(&mut self, rhs: $iX) {
                *self = *self / rhs;
            }
        }

        /*#[cfg(not(debug_assertions))]
        impl Rem for $ty {
            type Output = Self;
            fn rem(self, other: Self) -> Self {
                Self::wrapping_rem(self, other)
            }
        }

        #[cfg(debug_assertions)]
        impl Rem for $ty {
            type Output = Self;
            fn rem(self, other: Self) -> Self {
                Self::checked_rem(self, other).unwrap()
            }
        }

        impl RemAssign for $ty {
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }*/

        impl Shr<$uX> for $ty {
            type Output = Self;

            fn shr(self, rhs: $uX) -> Self {
                $ty(self.0 >> rhs)
            }
        }

        impl Shl<$uX> for $ty {
            type Output = Self;

            fn shl(self, rhs: $uX) -> Self {
                $ty(self.0 << rhs)
            }
        }

        impl Shr<usize> for $ty {
            type Output = Self;

            fn shr(self, rhs: usize) -> Self {
                $ty(self.0 >> rhs)
            }
        }

        impl Shl<usize> for $ty {
            type Output = Self;

            fn shl(self, rhs: usize) -> Self {
                $ty(self.0 << rhs)
            }
        }

        impl Sum for $ty {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::ZERO, Add::add)
            }
        }

        impl Product for $ty {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::ONE, Mul::mul)
            }
        }

        impl<'a> Sum<&'a $ty> for $ty {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::ZERO, |acc, elem| acc + *elem)
            }
        }

        impl<'a> Product<&'a $ty> for $ty {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::ONE, |acc, elem| acc * *elem)
            }
        }

        /// N.B.: if `fiN::MIN` is generated, `fiN::ZERO` is returned instead
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate fracints;
        /// extern crate rand;
        /// use rand::Rng;
        /// #[macro_use]
        /// use fracints::*;
        /// let mut rng = rand::thread_rng();
        /// println!("{}", rng.gen::<fi128>());
        /// ```
        #[cfg(feature = "rand")]
        impl rand::distributions::Distribution<$ty> for rand::distributions::Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                let x = $ty(rng.gen());
                if x == $ty::MIN {
                    $ty::ZERO
                } else {
                    x
                }
            }
        }
    };
}
