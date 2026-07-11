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
        $sqrt_fast:expr,
        $n:expr,
        $to_int:ident
    ) => {
        // TODO make inner type private, not doing this currently because we need const
        // traits
        /// Signed Fractional Integer of the unit range.
        ///
        /// Note: in the future we want to make the inner type private, but not
        /// currently because we don't have stable const traits.
        #[allow(non_camel_case_types)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $ty(pub $iX);

        impl Fracint for $ty {
            type Int = $iX;

            const BITS: usize = $uX::BITS as usize;
            const MAX: Self = Self($iX::MAX);
            const MIN: Self = Self($iX::MIN);
            const NEG_ONE: Self = Self(-$iX::MAX);
            const ONE: Self = Self($iX::MAX);
            const SIGNED: bool = true;
            const ULP: Self = Self(1);
            const ZERO: Self = Self(0);

            fn from_int(x: Self::Int) -> Self {
                Self(x)
            }

            fn as_int(self) -> Self::Int {
                self.0
            }

            fn overflowing_abs(self) -> (Self, bool) {
                (self.wrapping_abs(), self == Self::MIN)
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

            fn sqrt_fast(self) -> Self {
                $sqrt_fast(self)
            }

            /// Generates a random fracint from the given entropy.
            /// Note: if `fiN::MIN` is generated, `fiN::ZERO` is returned instead
            ///
            /// # Examples
            ///
            /// ```
            /// use fracints::*;
            ///
            /// let mut rng = rand::rng();
            /// println!("{}", fi128::rand(&mut rng));
            /// ```
            #[cfg(feature = "rand_support")]
            fn rand<R: rand_core::RngCore + ?Sized>(rng: &mut R) -> Self {
                // TODO this seems to be slow in some cases, use `next_u32` and `next_u64` when
                // possible
                let mut dst = Self::ZERO.0.to_le_bytes();
                rng.fill_bytes(&mut dst);
                let x = Self(Self::Int::from_le_bytes(dst));
                if x == Self::MIN { Self::ZERO } else { x }
            }

            fn from_f32(f: f32) -> Option<Self> {
                if f.abs() > 1.0 {
                    return None;
                }
                if f == 1.0 {
                    return Some(Self::ONE);
                } else if f == -1.0 {
                    return Some(Self::NEG_ONE);
                }
                let mut f = F32::from_f32(f);
                let mut x: FP<inlawi_ty!($n)> = FP::new(true, InlAwi::zero(), $n - 1).unwrap();
                FP::truncate_(&mut x, &mut f);
                Some(Self::from_int(x.$to_int()))
            }

            fn from_f64(f: f64) -> Option<Self> {
                if f.abs() > 1.0 {
                    return None;
                }
                if f == 1.0 {
                    return Some(Self::ONE);
                } else if f == -1.0 {
                    return Some(Self::NEG_ONE);
                }
                let mut f = F64::from_f64(f);
                let mut x: FP<inlawi_ty!($n)> = FP::new(true, InlAwi::zero(), $n - 1).unwrap();
                FP::truncate_(&mut x, &mut f);
                Some(Self::from_int(x.$to_int()))
            }

            fn to_f32(self) -> f32 {
                let mut f: FP<inlawi_ty!($n)> =
                    FP::new(true, InlAwi::from(self.as_int()), $n - 1).unwrap();
                // the msnb is never greater than 2^0 so is never anywhere near unrepresentable
                FP::try_to_f32(&mut f).unwrap()
            }

            fn to_f64(self) -> f64 {
                let mut f: FP<inlawi_ty!($n)> =
                    FP::new(true, InlAwi::from(self.as_int()), $n - 1).unwrap();
                // the msnb is never greater than 2^0 so is never anywhere near unrepresentable
                FP::try_to_f64(&mut f).unwrap()
            }
        }

        impl fmt::Debug for $ty {
            /// Converts to a base 10 string representation
            ///
            /// `fiN::ONE` and `fiN::NEG_ONE` are special cased to "1.0" and "-1.0"
            /// respectively.
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // TODO use constant sized buffers
                write!(f, "{}({})", stringify!($ty), $to_string(self.0))
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

            // TODO have a function for determining max number of chars with respect to
            // radix

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
            /// number of chars.
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

        impl Shl<usize> for $ty {
            type Output = Self;

            fn shl(self, rhs: usize) -> Self {
                $ty(self.0 << rhs)
            }
        }

        impl ShlAssign<usize> for $ty {
            fn shl_assign(&mut self, rhs: usize) {
                self.0 <<= rhs
            }
        }

        impl Shr<usize> for $ty {
            type Output = Self;

            fn shr(self, rhs: usize) -> Self {
                $ty(self.0 >> rhs)
            }
        }

        impl ShrAssign<usize> for $ty {
            fn shr_assign(&mut self, rhs: usize) {
                self.0 >>= rhs
            }
        }

        impl Not for $ty {
            type Output = Self;

            fn not(self) -> Self {
                $ty(!self.0)
            }
        }

        impl BitOr for $ty {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                $ty(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $ty {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = *self | rhs;
            }
        }

        impl BitAnd for $ty {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                $ty(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $ty {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = *self & rhs;
            }
        }

        impl BitXor for $ty {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self {
                $ty(self.0 ^ rhs.0)
            }
        }

        impl BitXorAssign for $ty {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = *self ^ rhs;
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
    };
}
