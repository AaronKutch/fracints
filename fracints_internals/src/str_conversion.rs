use std::num::NonZeroUsize;

use awint::{Awi, FP, InlAwi};
use thiserror::Error;

// TODO these docs could probably be at a module level and be more comprehensive

/// The error enum used to specify what parsing error happened when parsing a
/// fracint.
///
/// The input consists of an integer part, optional fraction part, and optional
/// exponent part. It can have a prefixed '-' to be negative, then a radix other
/// than 10 can be specified that will apply to the following parts, then the
/// integer part is specified. The fraction part begins with '.', and the
/// exponent can be started at the end with 'e' or 'p', and can include a '-' to
/// be negative. A single rigorous round-to-even is applied after the exponent
/// is applied, and then bounds are checked. The value "1.0" is special cased to
/// `fiN::ONE` and "-1" is special cased to `fiN::NEG_ONE`.
///
/// ```
/// use core::str::FromStr;
///
/// use fracints::{Fracint, fi8, fi64, fi128};
///
/// assert_eq!(fi64!(1), fi64::ONE);
/// assert_eq!(fi64!(1.000_000), fi64::ONE);
/// assert_eq!(fi64!(-1), fi64::NEG_ONE);
///
/// // The fraction 1138687895422480281 / 2^63 most closely matches the value of 0.123456789
/// assert_eq!(fi64!(0.123456789), fi64(1138687895422480281));
/// assert_eq!(
///     fi64::ONE.saturating_div_int(3).to_string(),
///     "0.3333333333333333333"
/// );
///
/// // exponents
/// assert_eq!(fi64!(0.000123e2), fi64!(0.0123));
/// assert_eq!(fi64!(123.456e-3), fi64!(0.123456));
/// assert_eq!(fi64!(42e-7), fi64!(0.0000042));
///
/// // extreme precision
/// assert_eq!(
///     fi128!(0.636619772367581343075535053490057448137_8),
///     fi128(108315241484954818046902227470560947936)
/// );
/// assert_eq!(
///     fi128(108315241484954818046902227470560947936).to_string(),
///     "0.636619772367581343075535053490057448135",
/// );
///
/// // round-to-even example, 1.5 / 128.0 is exactly 0.01171875
/// assert_eq!(fi8!(0.01171874), fi8(1));
/// assert_eq!(fi8!(0.01171875), fi8(2));
///
/// // Other radixes, note that certain literals currently fail to parse in Rust,
/// // so the macro unfortunately can't be used.
/// assert_eq!(fi8::from_str("0b0.1010101").unwrap(), fi8!(0.664));
/// assert_eq!(
///     fi64::from_str("-0xbeef.123456_p-5").unwrap(),
///     fi64!(-0.046614714728434592)
/// );
/// ```
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum FracintSerdeError {
    #[error("Radix is outside of the range `2..=36`")]
    RadixOutOfRange,
    #[error("Input was an empty string")]
    Empty,
    #[error("The input was not Ascii")]
    NotAscii,
    #[error("The input did not have an integer part")]
    EmptyInteger,
    #[error("The integer part has a char not in the radix")]
    InvalidCharInInteger,
    #[error(
        "The input does not have a fraction, there should always be a '.' and digits following it"
    )]
    EmptyFraction,
    #[error("There was an invalid char in the fraction")]
    InvalidCharInFraction,
    #[error("There was a char for the exponent but the value was empty")]
    EmptyExponent,
    #[error("The exponent was empty, or there was a char in the exponent that was not valid")]
    InvalidCharInExponent,
    #[error("The numeric value was not in range")]
    Overflow,
    #[error("miscellanious error")]
    Other,
}

// TODO do this in a more optimized way without `awint`

/// Takes a string and the bitwidth of the result type
fn common_from_str(
    s: &str,
    bw: isize,
) -> Result<Result<(bool, Awi), awint::SerdeError>, FracintSerdeError> {
    use FracintSerdeError::*;

    let sign;
    let integer;
    let mut fraction = None;
    let mut exp = None;
    let mut exp_negative = false;
    let radix;

    let is_empty_or_all_underscores = |s: &[u8]| {
        let mut all_underscores = true;
        for c in s {
            if *c != b'_' {
                all_underscores = false;
                break;
            }
        }
        all_underscores
    };

    let is_integral = |c: u8, radix: u8| {
        let is_underscore = c == b'_';
        let is_binary = (b'0' <= c) && (c <= b'1');
        let is_octal = (b'0' <= c) && (c <= b'7');
        let is_decimal = (b'0' <= c) && (c <= b'9');
        let is_lowerhex = (b'a' <= c) && (c <= b'f');
        let is_upperhex = (b'A' <= c) && (c <= b'F');
        match radix {
            2 => is_underscore || is_binary,
            8 => is_underscore || is_octal,
            10 => is_underscore || is_decimal,
            16 => is_underscore || is_decimal || is_lowerhex || is_upperhex,
            _ => unreachable!(),
        }
    };

    let s = s.as_bytes();
    if is_empty_or_all_underscores(s) {
        return Err(Empty);
    }

    // handle sign
    let mut i = 0;
    if s[i] == b'-' {
        if s.len() <= 1 {
            return Err(EmptyInteger);
        }
        sign = true;
        i += 1;
    } else {
        sign = false;
    }

    // handle radix
    if (s[i] == b'0') && ((i + 1) < s.len()) {
        if s[i + 1] == b'b' {
            radix = 2;
            i += 2;
        } else if s[i + 1] == b'o' {
            radix = 8;
            i += 2;
        } else if s[i + 1] == b'x' {
            radix = 16;
            i += 2;
        } else {
            radix = 10;
        }
    } else {
        radix = 10;
    }

    // integer part, can be followed by '.' for fraction, 'e' or 'p' for exponent
    let integer_start = i;
    let mut fraction_start = None;
    let mut exp_start = None;
    loop {
        if i >= s.len() {
            integer = &s[integer_start..i];
            break;
        }
        if !is_integral(s[i], radix) {
            if s[i] == b'.' {
                fraction_start = Some(i + 1);
            } else if (s[i] == b'e') || (s[i] == b'p') {
                exp_start = Some(i + 1);
            } else {
                return Err(InvalidCharInInteger);
            }
            integer = &s[integer_start..i];
            i += 1;
            break;
        }
        i += 1;
    }

    // fraction part, can be followed by 'e' or 'p' for exponent
    if let Some(fraction_start) = fraction_start {
        loop {
            if i >= s.len() {
                fraction = Some(&s[fraction_start..i]);
                break;
            }
            if !is_integral(s[i], radix) {
                if (s[i] == b'e') || (s[i] == b'p') {
                    exp_start = Some(i + 1);
                } else {
                    return Err(InvalidCharInFraction);
                }
                fraction = Some(&s[fraction_start..i]);
                i += 1;
                break;
            }
            i += 1;
        }
    }

    // exponent part
    if let Some(mut exp_start) = exp_start {
        loop {
            if i >= s.len() {
                break;
            }
            if !is_integral(s[i], radix) {
                if s[i] == b'-' {
                    if exp_negative {
                        return Err(InvalidCharInExponent);
                    }
                    exp_negative = true;
                    exp_start += 1;
                    i += 1;
                    continue;
                } else {
                    return Err(InvalidCharInExponent);
                }
            }
            i += 1;
        }
        exp = Some(&s[exp_start..i]);
    }

    if is_empty_or_all_underscores(integer) {
        return Err(EmptyInteger);
    }

    if let Some(fraction) = fraction {
        if is_empty_or_all_underscores(fraction) {
            return Err(EmptyFraction);
        }
    }
    let fraction = fraction.unwrap_or(&[]);

    let pad0 = &mut InlAwi::from_usize(0);
    let pad1 = &mut InlAwi::from_usize(0);
    let mut usize_awi = InlAwi::from_usize(0);
    let exp = if let Some(exp) = exp {
        if is_empty_or_all_underscores(exp) {
            return Err(EmptyExponent);
        }
        if usize_awi
            .bytes_radix_(Some(exp_negative), exp, radix, pad0, pad1)
            .is_err()
        {
            return Err(Overflow);
        }
        usize_awi.to_isize()
    } else {
        0
    };

    // note we handle the sign ourselves, the sign bit is instead room for ONE and
    // NEG_ONE
    Ok(Awi::from_bytes_general(
        None,
        integer,
        fraction,
        exp,
        radix,
        NonZeroUsize::new(bw as usize).unwrap(),
        bw - 1,
    )
    .map(|res| (sign, res)))
}

macro_rules! impl_signed_conversions {
($($iX:ident $to_string:ident $from_str:ident $to_iX:ident $from_iX:ident);*;) => {$(
    /// Conversion of the internal integer of a fracint to a base 10 string
    pub fn $to_string(x: $iX) -> String {
        const TMP: $iX = -$iX::MAX;
        match x {
            TMP | $iX::MIN => return "-1.0".to_string(),
            0 => return "0.0".to_string(),
            $iX::MAX => return "1.0".to_string(),
            _ => (),
        }
        let sign = x < 0;
        let x = FP::new(true, InlAwi::$from_iX(x), ($iX::BITS - 1) as isize).unwrap();
        let (int, frac) = FP::to_str_general(&x, 10, false, 1, 1, 4096).unwrap();
        if sign {
            format!("-{int}.{frac}")
        } else {
            format!("{int}.{frac}")
        }
    }

    /// Conversion from a string representation to the internal integer of a
    /// fracint.
    pub fn $from_str(s: &str) -> Result<$iX, FracintSerdeError> {
        use FracintSerdeError::*;
        match common_from_str(s, $iX::BITS as isize)? {
            Ok((sign, awi)) => {
                // ONE and NEG_ONE special cases
                if awi.msb() {
                    if awi.is_imin() {
                        if sign {
                            Ok(-$iX::MAX)
                        } else {
                            Ok($iX::MAX)
                        }
                    } else {
                        Err(Overflow)
                    }
                } else if sign {
                    Ok(-awi.$to_iX())
                } else {
                    Ok(awi.$to_iX())
                }
            }
            _ => Err(Overflow),
        }
    }
)*}
}

impl_signed_conversions!(
    i8 i8_to_string i8_from_str to_i8 from_i8;
    i16 i16_to_string i16_from_str to_i16 from_i16;
    i32 i32_to_string i32_from_str to_i32 from_i32;
    i64 i64_to_string i64_from_str to_i64 from_i64;
    i128 i128_to_string i128_from_str to_i128 from_i128;
);
