#[macro_export]
macro_rules! impl_signed {
    (
        // the new normalized integer type we are defining
        $ty:ident,
        // the string represenation of $ty, e.g. "fi8"
        $s:expr,
        // the type used internally
        $iX:ident,
        // the unsigned version of $iX
        $uX:ident,
        // closures
        $normalized_mul:expr,
        $normalized_div:expr,
        $c:expr // constants
    ) => {
        #[allow(non_camel_case_types)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $ty(pub $iX);

        impl $ty {
            /// The number of bits in this type
            pub const BITS: usize = $uX::BITS;
            /// One positive Unit in the Last Place
            pub const ULP: $ty = $ty(1);
            /// The minimum value representable by a `fiN`
            pub const MIN: $ty = $ty($iX::MIN);
            /// A numerical value of negative one.
            ///
            /// Note that `NEG_ONE` is not equal to `MIN`, but to `MIN + ULP`.
            /// The purpose of `NEG_ONE` in contrast to `MIN` is preventing
            /// certain overflows, such as `fiN::MIN.wrapping_mul(fiN::MIN)` or
            /// `fiN::wrapping_abs(fiN::MIN)`.
            pub const NEG_ONE: $ty = $ty(-$iX::MAX);
            /// Zero.
            pub const ZERO: $ty = $ty(0);
            /// For `fiN`, `ONE` and `MAX` are the same. Prefer to use `MAX` when
            /// wanting to emphasize the true numeric bounds or ordered maximum,
            /// and instead use `ONE` for numeric values.
            pub const ONE: $ty = $ty($iX::MAX);
            /// The maximum value representable by a `fiN``
            pub const MAX: $ty = $ty($iX::MAX);

            /// Converts `src`, assumed to be a &str representation of an `fiN` in base `radix`
            /// (which must be in the range `2u32..=36u32`), to `fiN`.
            ///
            /// `src` can be arbitrarily long but significance usually stops after a number
            /// of chars. TODO this is always constant with respect to radix?.
            ///
            /// The number must be in the range `(-1.0,1.0)` (e.g. `&"0.43987236"`,
            /// `&-"0.999"`, `&"0.000001"`, `&"-.12345"`, `&".999999"`, `&".1"`).
            /// Other cases are:
            ///  - `&"-1."` => `fiN::NEG_ONE`
            ///  - `&"0."` => `fiN::ZERO`
            ///  - `&"1."` => `fiN::ONE`
            /// An arbitrary number of `0` and `_` is allowed after these strings.
            ///
            /// All valid inputs have decimal points to prevent confusing expressions such as
            /// `fi32(1)` (which is just `fi32::ULP`) with the procedural macro call `fi32!(1)`
            /// (invalid, should be `fi32!(1.)`).
            ///
            /// Note: the resulting `fiN` can be plus or minus 0.5 ULP away from the best
            /// approximation that can be made of src. If there exists two possible values that are
            /// both exactly 0.5 ULPs away from `src`, then it rounds to what is even in the
            /// non-fraction representation.
            ///
            /// See the [FracintParseError] documentation for parsing errors and examples.
            pub fn from_str_radix(src: &str, radix: u8) -> Result<Self, FracintParseError> {
                use FracintParseError::*;
                if radix < 2 || radix > 36 {
                    return Err(RadixOutOfRange);
                }
                let src_len = src.len();
                let mut chars = src.chars();
                let (is_negative, len) = match chars.next() {
                    Some('-') => match chars.next() {
                        Some('1') => match chars.next() {
                            Some('.') => loop {
                                match chars.next() {
                                    None => return Ok($ty::NEG_ONE),
                                    Some('0') | Some('_') => (),
                                    _ => return Err(InvalidCharInFraction),
                                }
                            },
                            None => return Err(NoDecimalPoint),
                            _ => return Err(InvalidCharAfterOne),
                        },
                        Some('0') => match chars.next() {
                            Some('.') => (true, src_len - 3),
                            None => return Err(NoDecimalPoint),
                            _ => return Err(InvalidCharAfterZero),
                        },
                        Some('.') => (true, src_len - 2),
                        None => return Err(SingleNeg),
                        _ => return Err(InvalidCharAfterNeg),
                    },
                    Some('1') => match chars.next() {
                        Some('.') => loop {
                            match chars.next() {
                                None => return Ok($ty::ONE),
                                Some('0') => (),
                                _ => return Err(InvalidCharInFraction),
                            }
                        },
                        None => return Err(NoDecimalPoint),
                        _ => return Err(InvalidCharAfterOne),
                    },
                    Some('0') => match chars.next() {
                        Some('.') => (false, src_len - 2),
                        None => return Err(NoDecimalPoint),
                        _ => return Err(InvalidCharAfterZero),
                    },
                    Some('.') => (false, src_len - 1),
                    None => return Err(EmptyInput),
                    _ => return Err(InvalidBeginningChar),
                };
                if len == 0 {
                    // handles an edge case where `mul` gets shifted to 0, meaning that rounding
                    // will increase a fiN(0) to fiN(1).
                    return Ok($ty::ZERO);
                }
                // get the raw integer representation of the fraction.
                // I could use iX::from_str_radix but I may want to do special optimizations in the
                // future and other things. $uD is used so that $from_str_radix_conversion does not
                // have to round any to prevent overflow.

                // This multiplies `tmp` by the radix, adds a digit, and repeats. In parallel, a
                // `mul` variable is being multiplied by the radix as many times as `tmp` is
                // multiplied by the radix.

                // This is set such that the radix multiplied by itself `chars` times cannot
                // overflow.
                let bitwidth = BitWidth::new(
                    (1usize << (32 - radix.leading_zeros()))
                        .wrapping_mul(len)
                        .wrapping_add($ushift),
                )
                .unwrap();
                let radix2 = ApInt::from_u32(radix).into_zero_resize(bitwidth);
                let mut tmp = ApInt::zero(bitwidth);
                let mut mul = ApInt::one(bitwidth);
                for c in chars {
                    match c.to_ascii_lowercase().to_digit(radix) {
                        Some(digit) => {
                            let digit2 = ApInt::from_u32(digit).into_zero_resize(bitwidth);
                            tmp.wrapping_mul_assign(&radix2).unwrap();
                            tmp.wrapping_add_assign(&digit2).unwrap();
                            mul.wrapping_mul_assign(&radix2).unwrap();
                        }
                        None => {
                            if c != '_' {
                                return Err(InvalidCharInFraction);
                            }
                        }
                    }
                }
                todo!();
                /*
                // To understand how this works, imagine `fi8::from_str_radix(&"0.123", 10)`.
                // The routine above will produce `tmp = 123` and `mul = 1000`. Shifting `tmp` by
                // `$ishift` will result in `tmp = 123 * 128 = 15744`. Dividing `tmp` by `mul`
                // produces a quotient of 15 and a remainder of 744. The remainder is greater than
                // or equal to 1000 >> 1, so we round up for a final value of fi8(16). This
                // corresponds to 0.125, which is as close as possible to the "0.123" input.

                // In order for the rounding to always be correct and within 0.5 ULPs of the input
                // value, we have to use arbitrary precision arithmetic.
                tmp.wrapping_shl_assign($ishift).unwrap();
                let rounding_point = mul.clone().into_wrapping_lshr(1).unwrap();
                ApInt::wrapping_udivrem_assign(&mut tmp, &mut mul).unwrap();
                if mul.checked_uge(&rounding_point).unwrap() {
                    tmp.wrapping_inc();
                }

                // note: unless the rounding is changed to never incrementing, it is possible to
                // round to `fiN::MAX + 1`, which needs to be guarded against.
                let max_value = ApInt::from($iX::MAX).into_zero_resize(bitwidth);
                if tmp.checked_ugt(&max_value).unwrap() {
                    tmp.wrapping_dec();
                }

                let output = $apint_to_fiN(tmp);
                if is_negative {
                    Ok(-output) // $ty::MIN is caught above, so no overflow
                } else {
                    Ok(output)
                }*/
            }

            /// Converts the `fiN` to a string representation in base `radix`,
            /// with a numerical error of <= 0.5 ULP.
            ///
            /// There are some special cases:
            /// - `fiN::ONE` => "1." (the decimal point here is a reminder that 1 cannot be
            ///   exactly represented in `fiN`, and to preserve roundtrips)
            /// - `fiN::NEG_ONE` | `fiN::MIN` => "-1." (to prevent `fiN::MIN` propagation)
            /// - `fiN::ZERO` => "0." (the decimal point here is to correspond with
            ///   `from_str_radix` roundtrips)
            ///
            /// To know the max number of digits displayed, consider what happens when fi32(0) is
            /// incremented internally to fi32(1). The value it represents will go from 0 to
            /// ~0.0000000004657, or a difference of ~4.657*10^-10. These smallest increments (ULPs)
            /// will always change the tenth place and onward.
            /// In other words, the number of digits in the fraction is the smallest number of
            /// digits for which every smallest increment of fiN always produces a unique string.
            /// This function will thus display a max of ten digits in the fraction when it is
            /// called with radix 10 on a fi32.
            /// If there are trailing zeros such as "0.7500000000", then it will be trimmed to
            /// "0.75".
            ///
            /// # Errors
            ///
            /// If `radix` is not in the range `2u32..=36u32`, this will return `None`.
            pub fn to_string_radix(&self, radix: u32) -> Option<String> {
                assert!(
                    radix >= 2 && radix <= 36,
                    "radix must lie in the range `[2, 36]` - found {}",
                    radix
                );
                match *self {
                    $ty::NEG_ONE | $ty::MIN => return "-1.".to_string(),
                    $ty::ZERO => return "0.".to_string(),
                    $ty::ONE => return "1.".to_string(),
                    _ => (),
                }

                todo!()
                /*
                // Except in radixes that are a power-of-two (2, 4, 8, 16, 32) or larger than
                // `$iX::MAX` (which is not possible for i8), The number of digits needed for a
                // unique representation is always equal to the number of digits needed for
                // `$iX::MAX + 1` representation.
                // This is because `1 / ($iX::MAX + 1)` (using perfect precision here) is always
                // going to have the most significant digit a number of places from the decimal
                // point equal to the number of digits needed for `$iX::MAX`, except when
                // `$iX::MAX + 1` can be factored completely into only `radix`. In this case, the
                // number of digits needed for the representation is one less than `$iX::MAX + 1`.

                // Suppose that `$iX::MAX + 1` was not a power of 2, but rather 10. This edge case
                // can be seen in `1 / 1000 = 0.001` (4 digits in "1000", but 3 digits in "001").

                // TODO: this can be put into a set of constants
                let str_len = {
                    // for being able to divide $iX::MAX + 1
                    let bitwidth = BitWidth::new($ty::BITS + 1).unwrap();
                    let radix2 = ApInt::from_u32(radix).into_zero_resize(bitwidth);
                    let mut val = ApInt::one(bitwidth);
                    val.wrapping_shl_assign($ty::BITS).unwrap();
                    let mut len = 0;
                    loop {
                        val.wrapping_udiv_assign(&radix2).unwrap();
                        len += 1;
                        if val.is_zero() {
                            break len;
                        }
                    }
                };
                let factor_num = match radix {
                    2 | 4 | 8 | 16 | 32 => str_len - 2,
                    _ => str_len - 1,
                };
                let bitwidth = BitWidth::new(
                    (1usize << (32 - radix.leading_zeros())).wrapping_mul(factor_num + 2),
                )
                .unwrap();

                // For example, fi8(16) will result in `(16 * 1000) / 128 = 125` which results in
                // "0.125". TODO: store a 1000 / 128 constant in fixed point

                let radix2 = ApInt::from_u32(radix).into_zero_resize(bitwidth);
                let mut mul = ApInt::one(bitwidth);
                for _ in 0..factor_num {
                    mul.wrapping_mul_assign(&radix2).unwrap();
                }
                // `mul` is now one `radix` factor away from `1000`

                // $ty::MIN is caught earlier so no overflow
                let mut val = ApInt::from((*self).wrapping_abs().0).into_zero_resize(bitwidth);
                val.wrapping_mul_assign(&mul).unwrap();
                val.wrapping_mul_assign(&radix2).unwrap();
                // `val` is now the `16 * 1000`

                let round = val.get_bit_at($ishift - 1).unwrap();
                // division by power of two
                val.wrapping_lshr_assign($ishift).unwrap();
                if round {
                    val.wrapping_inc();
                }
                // `val` is now the `125`

                // Using a brute force mechanism for simplicity.
                // what will happen is that the `mul` 100 is compared to `val` 125, and found to
                // be less than or equal to. `mul` is subtracted from `val` to make 25 and `ascii`
                // is incremented. The next inner loop breaks, `mul` is divided by the radix, and
                // when `ascii` is added to the string, `s` is "0.1".
                //
                // On the next outer loop, 10 is found to be less than 25, 10 is subtracted from 25,
                // the ascii is incremented, and this is repeated one more time.
                // the result is that `mul` is 1, `val` is 5, `s` is "0.12".
                //
                // The outer loop runs one more time, and the string ends up as the correct "0.125".
                let mut s = if self.is_negative() {
                    String::from("-0.")
                } else {
                    String::from("0.")
                };
                loop {
                    if val.is_zero() {
                        // trailing zeros not included automatically
                        break;
                    }
                    // 48 is ascii for zero
                    let mut ascii = 48u8;
                    if mul.checked_ule(&val).unwrap() {
                        loop {
                            if mul.checked_ule(&val).unwrap() {
                                val.wrapping_sub_assign(&mul).unwrap();
                                ascii += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    mul.wrapping_udiv_assign(&radix2).unwrap();
                    if ascii > 57 {
                        // move up ascii table to letters
                        ascii += 17;
                    }
                    s.push(char::from(ascii));
                }
                s*/
            }

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

            /// Returns a tuple of `self.wrapping_abs()` along with a boolean indicating whether an
            /// overflow happened.
            pub fn overflowing_abs(self) -> (Self, bool) {
                (self.wrapping_abs(), self == $ty::MIN)
            }

            /// Same as `overflowing_abs` except it returns an `Option<fiN>`, where `Some(fiN)`
            /// means no overflow and `None` means overflow.
            pub fn checked_abs(self) -> Option<Self> {
                match self.overflowing_abs() {
                    (x, false) => Some(x),
                    (_, true) => None,
                }
            }

            /// Saturating absolute value of `self`. It behaves the same way as `wrapping_abs`
            /// except `fiN::MIN.saturating_abs()` -> `fiN::ONE`
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

            /// Returns a tuple of `self.wrapping_neg()` along with a boolean indicating whether an
            /// overflow happened.
            pub fn overflowing_neg(self) -> (Self, bool) {
                (self.wrapping_neg(), self == $ty::MIN)
            }

            /// Same as `overflowing_neg` except it returns an `Option<fiN>`, where `Some(fiN)`
            /// means no overflow and `None` means overflow.
            pub fn checked_neg(self) -> Option<Self> {
                match self.overflowing_neg() {
                    (x, false) => Some(x),
                    (_, true) => None,
                }
            }

            /// Saturating negation of `self`. It behaves the same way as `wrapping_neg` except
            /// `fiN::MIN.saturating_neg()` -> `fiN::ONE`
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

            /// Returns a tuple of `self.wrapping_inv()` along with a boolean indicating whether an
            /// overflow happened.
            pub fn overflowing_inv(self) -> ($iX, bool) {
                ($iX::MAX / self.0, self == $ty::ZERO)
            }

            /// Same as `overflowing_inv` except it returns an `Option<iX>`, where `Some(iX)` means
            /// no overflow and `None` means overflow.
            pub fn checked_inv(self) -> Option<$iX> {
                match self.overflowing_inv() {
                    (x, false) => Some(x),
                    (_, true) => None,
                }
            }

            /// Saturating negation of `self`. It behaves the same way as `wrapping_inv` except
            /// `fiN::MIN.saturating_inv()` -> `-1`
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
            /// If the numeric value of `self` added to `other` is more than `fiN::MAX` or less than
            /// `fiN::MIN` (if `fiN::MIN` should be avoided, use `fiN::NEG_ONE` as the minimum value
            /// instead), modular overflow will happen.
            pub fn wrapping_add(self, other: Self) -> Self {
                $ty(self.0.wrapping_add(other.0))
            }

            /// Returns a tuple of `self.wrapping_add(other)` along with a boolean indicating
            /// whether an overflow happened.
            /// Note that if `self.wrapping_add(other) == fiN::MIN`, it is counted as overflow.
            pub fn overflowing_add(self, other: Self) -> (Self, bool) {
                let (internal, overflow) = self.0.overflowing_add(other.0);
                if internal == $iX::MIN {
                    ($ty(internal), true)
                } else {
                    ($ty(internal), overflow)
                }
            }

            /// Same as `overflowing_add` except it returns an `Option<fiN>`, where `Some(fiN)`
            /// means no overflow and `None` means overflow.
            pub fn checked_add(self, other: Self) -> Option<Self> {
                match self.overflowing_add(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating addition. Saturates at the numeric bounds `fiN::NEG_ONE` and `fiN::ONE`
            /// instead of overflowing.
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
            /// If the numeric value of `self` subtracted by `other` is more than `fiN::MAX` or
            /// less than `fiN::MIN` (if `fiN::MIN` should be avoided, use `fiN::NEG_ONE` as the
            /// minimum value instead), modular overflow will happen.
            pub fn wrapping_sub(self, other: Self) -> Self {
                $ty(self.0.wrapping_sub(other.0))
            }

            /// Returns a tuple of `self.wrapping_sub(other)` along with a boolean indicating
            /// whether an overflow happened.
            /// Note that if `self.wrapping_sub(other) == fiN::MIN`, it is counted as overflow.
            pub fn overflowing_sub(self, other: Self) -> (Self, bool) {
                let (internal, overflow) = self.0.overflowing_sub(other.0);
                if internal == $iX::MIN {
                    ($ty(internal), true)
                } else {
                    ($ty(internal), overflow)
                }
            }

            /// Same as `overflowing_sub` except it returns an `Option<fiN>`, where `Some(fiN)`
            /// means no overflow and `None` means overflow.
            pub fn checked_sub(self, other: Self) -> Option<Self> {
                match self.overflowing_sub(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating subtraction. Saturates at the numeric bounds `fiN::NEG_ONE` and
            /// `fiN::ONE` instead of overflowing.
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

            /// Returns a tuple of `self.wrapping_mul(other)` along with a boolean indicating
            /// whether an overflow happened.
            ///
            /// Note that only the overflow possible is the corner case
            /// `fiN::MIN.wrapping_mul(fiN::MIN)` -> `fiN::MIN`.
            pub fn overflowing_mul(self, other: Self) -> (Self, bool) {
                (
                    self.wrapping_mul(other),
                    self == $ty::MIN && other == $ty::MIN,
                )
            }

            /// Same as `overflowing_mul` except it returns an `Option<fiN>`, where `Some`
            /// means no overflow and `None` means overflow.
            pub fn checked_mul(self, other: Self) -> Option<Self> {
                match self.overflowing_mul(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating fracint multiplication. Saturates at the numeric bounds `fiN::NEG_ONE`
            /// and `fiN::ONE` instead of overflowing.
            pub fn saturating_mul(self, other: Self) -> Self {
                if self == $ty::MIN && other == $ty::MIN {
                    $ty::ONE
                } else {
                    self.wrapping_mul(other)
                }
            }

            /// Wrapping fracint division.
            /// It is strongly recommended to use `saturating_div` instead unless all of the
            /// invariants can be upheld
            ///
            /// # Overflow Behavior
            ///
            /// Overflow will happen if `self.saturating_abs() >= other.saturating_abs()`. TODO
            /// test this extensively Note that fiN::MIN can be produced if `self` and `other` are
            /// not equal in sign but equal in absolute value.
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

            /// Returns a tuple of `self.wrapping_div(other)` (with exceptions for division by zero)
            /// along with a boolean indicating whether an overflow happened.
            /// If `other == fiN::ZERO`, `(self.signum(),true)` is returned.
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

            /// Same as `overflowing_div` except it returns an `Option<fiN>`, where `Some(fiN)`
            /// means no overflow and `None` means overflow or divide by zero.
            pub fn checked_div(self, other: Self) -> Option<Self> {
                match self.overflowing_div(other) {
                    (v, false) => Some(v),
                    (_, true) => None,
                }
            }

            /// Saturating fracint division. Saturates at the numeric bounds `fiN::NEG_ONE` and
            /// `fiN::ONE` instead of overflowing.
            ///
            /// Panics are prevented and saturation handled in this way:
            /// - if `other == fiN::ZERO`, `self.signum()` is returned
            /// - else if `self.saturating_abs() >= other.saturating_abs()`, it will return
            ///   `fiN::NEG_ONE` if their signs are not equal, `fiN::ONE` otherwise (except for
            ///   `other == fiN::ZERO` as shown above)
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

            /// Intended to only be called for `-0.5 <= self <= 0.5`. This function has overflows
            /// outside of this range.
            fn cos_taudiv4_taylor_base(self) -> $ty {
                // This is based on the equation
                // cos((tau/4) * x) =
                // ( (4/tau)^2 - (x^2)/2 ) / (4/tau)^2
                // + x^4 / ( (4/tau)^4 * 4! )
                // - x^6 / ( (4/tau)^6 * 6! )
                // + x^8 / ( (4/tau)^8 * 8! )
                // - ...

                let cutoff = (self.0 as $uX) >> ($ushift / 2);
                if cutoff == 0 || cutoff == ($uX::MAX >> ($ushift / 2)) {
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

            /// Intended to only be called for `-0.5 <= theta <= 0.5`. This function has overflows
            /// outside of this range.
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
            /// By having a (tau/4) constant and cleverly rearranging the taylor series, this
            /// provides a basic way to calculate cosine for fracints.
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
            /// By having a (tau/4) constant and cleverly rearranging the taylor series, this
            /// provides a basic way to calculate sine for fracints.
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
                // this compares the highest two bits of `self` offset by a eighth of a circle to
                // determine which combination to use.
                let o = self.wrapping_add($ty::MIN / -4).0 as $uX;
                match ((o & (1 << ($ishift - 1))) != 0, (o & (1 << $ishift)) != 0) {
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
            /// Uses `self.to_string_radix(10)`.
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.to_string_radix(10))
            }
        }

        impl FromStr for $ty {
            type Err = fracintParseError;

            /// Uses `Self::from_str_radix(s, 10)`.
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_str_radix(s, 10)
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
