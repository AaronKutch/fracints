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

        impl Fracint for $ty {
            type Int = $iX;

            const BITS: usize = $uX::BITS as usize;
            const MAX: Self = Self($iX::MAX);
            const MIN: Self = Self($iX::MIN);
            const NEG_ONE: Self = Self(-$iX::MAX);
            const ONE: Self = Self($iX::MAX);
            const ULP: Self = Self(1);
            const ZERO: Self = Self(0);

            fn is_negative(self) -> bool {
                self < Self::ZERO
            }

            fn is_positive(self) -> bool {
                self >= Self::ZERO
            }

            fn signum(self) -> Self {
                if self < Self::ZERO {
                    Self::NEG_ONE
                } else if self == Self::ZERO {
                    Self::ZERO
                } else {
                    Self::ONE
                }
            }

            fn wrapping_abs(self) -> Self {
                if self.is_negative() {
                    self.wrapping_neg()
                } else {
                    self
                }
            }

            fn overflowing_abs(self) -> (Self, bool) {
                (self.wrapping_abs(), self == Self::MIN)
            }

            fn saturating_abs(self) -> Self {
                self.checked_abs().unwrap_or(Self::ONE)
            }

            fn wrapping_neg(self) -> Self {
                Self(self.0.wrapping_neg())
            }

            fn overflowing_neg(self) -> (Self, bool) {
                (self.wrapping_neg(), self == Self::MIN)
            }

            fn saturating_neg(self) -> Self {
                self.checked_neg().unwrap_or(Self::ONE)
            }

            fn saturating_inv(self) -> $iX {
                $iX::MAX.checked_div(self.0).unwrap_or(-1)
            }

            fn wrapping_add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }

            fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (internal, overflow) = self.0.overflowing_add(rhs.0);
                if internal == $iX::MIN {
                    (Self(internal), true)
                } else {
                    (Self(internal), overflow)
                }
            }

            fn saturating_add(self, rhs: Self) -> Self {
                // note that $ty::MAX added to 0 does not overflow but $ty::MIN added to 0 does
                // overflow, and $ty::MIN.wrapping_add($ty::ZERO) and
                // $ty::ZERO.wrapping_add($ty::MIN) both overflow which means that
                // `rhs <= $ty::ZERO` catches all the cases
                self.checked_add(rhs).unwrap_or_else(|| {
                    if rhs <= Self::ZERO {
                        Self::NEG_ONE
                    } else {
                        Self::ONE
                    }
                })
            }

            fn wrapping_sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }

            fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (internal, overflow) = self.0.overflowing_sub(rhs.0);
                if internal == $iX::MIN {
                    (Self(internal), true)
                } else {
                    (Self(internal), overflow)
                }
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                self.checked_sub(rhs).unwrap_or_else(|| {
                    if rhs <= Self::ZERO {
                        Self::ONE
                    } else {
                        Self::NEG_ONE
                    }
                })
            }

            fn wrapping_mul(self, rhs: Self) -> Self {
                Self($normalized_mul(self.0, rhs.0))
            }

            fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                (
                    self.wrapping_mul(rhs),
                    self == Self::MIN && rhs == Self::MIN,
                )
            }

            fn saturating_mul(self, rhs: Self) -> Self {
                if self == Self::MIN && rhs == Self::MIN {
                    Self::ONE
                } else {
                    self.wrapping_mul(rhs)
                }
            }

            fn saturating_mul_int(self, rhs: $iX) -> Self {
                let mut res = Self(self.0.saturating_mul(rhs));
                if res == Self::MIN {
                    res = Self::NEG_ONE;
                }
                res
            }

            fn saturating_div(self, rhs: Self) -> Self {
                if rhs == Self::ZERO {
                    self.signum()
                } else if self.saturating_abs() >= rhs.saturating_abs() {
                    //self == rhs == 0 eliminated above
                    if (self < Self::ZERO) != (rhs < Self::ZERO) {
                        Self::NEG_ONE
                    } else {
                        Self::ONE
                    }
                } else {
                    Self($normalized_div(self.0, rhs.0))
                }
            }

            fn saturating_div_int(self, rhs: Self::Int) -> Self {
                if rhs == 0 {
                    self.signum()
                } else if (self == Self::MIN) && (rhs == -1) {
                    Self::ONE
                } else {
                    Self(self.0.wrapping_div(rhs))
                }
            }
        }

        impl $ty {
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

        impl Neg for $ty {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self.saturating_neg()
            }
        }

        impl Add for $ty {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                self.saturating_add(rhs)
            }
        }

        impl AddAssign for $ty {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl Sub for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self.saturating_sub(rhs)
            }
        }

        impl SubAssign for $ty {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl Mul for $ty {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                self.saturating_mul(rhs)
            }
        }

        impl Mul<$iX> for $ty {
            type Output = Self;

            fn mul(self, rhs: $iX) -> Self::Output {
                self.saturating_mul_int(rhs)
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

        impl Div for $ty {
            type Output = $ty;

            fn div(self, rhs: Self) -> Self {
                self.saturating_div(rhs)
            }
        }

        impl Div<$iX> for $ty {
            type Output = Self;

            fn div(self, rhs: $iX) -> Self::Output {
                self.saturating_div_int(rhs)
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
