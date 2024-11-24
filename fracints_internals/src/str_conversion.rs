use std::num::NonZeroUsize;

use awint::{Awi, InlAwi};
use thiserror::Error;

// TODO check if thiserror can do docs all in one

/// The error enum used to specify what parsing error happened when parsing a fracint
///
/// TODO
/// ```
/// use fracints::fi64;
/// assert_eq!(fi64::from_str_radix(&"-1.0",10).unwrap(),fi64::NEG_ONE);
///
/// assert_eq!(fi16::from_str_radix(&"0.123456789",10).unwrap(),fi16(4045));
/// assert_eq!(fi16(4045).to_string(), "0.12344".to_string());
/// ```
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum FracintSerdeError {
    #[error("Radix is outside of the range `2u8..=36u8`")]
    RadixOutOfRange,
    #[error("Input was an empty string")]
    Empty,
    #[error("The input was not Ascii")]
    NotAscii,
    #[error("The input did not have an integer part")]
    EmptyInteger,
    #[error("The integer part can only be '0' or '1'")]
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

/// Conversion to the internal integer for the fracint
pub fn i8_from_str(s: &str) -> Result<i8, FracintSerdeError> {
    use FracintSerdeError::*;

    let sign;
    let integer;
    let mut fraction = None;
    let mut exp_start = None;
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

    if i >= s.len() {
        return Err(EmptyInteger);
    }

    // the integral part should be '0' or '1'
    if s[i] == b'1' {
        // special case
        integer = "1".as_bytes();
        i += 1;
    } else if s[i] == b'0' {
        integer = "0".as_bytes();
        i += 1;
    } else {
        return Err(InvalidCharInInteger);
    }

    if i >= s.len() {
        return Err(EmptyFraction);
    }

    // there should always be a '.'
    if s[i] == b'.' {
        i += 1;
    } else {
        return Err(EmptyFraction);
    }
    let fraction_start = i;

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

    // fraction part, can be followed by 'e' or 'p' for exponent
    loop {
        if i >= s.len() {
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
    let Some(fraction) = fraction else {
        return Err(EmptyFraction);
    };

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
        exp = Some(&s[exp_start..]);
    }

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

    match Awi::from_bytes_general(
        Some(sign),
        integer,
        fraction,
        exp,
        radix,
        NonZeroUsize::new(i8::BITS as usize).unwrap(),
        (i8::BITS - 1) as isize,
    ) {
        Ok(awi) => Ok(awi.to_i8()),
        _ => Err(Overflow),
    }
}

/*macro_rules! impl_signed_conversions {
    ($($from_str_radix:ident, $iX:ident);*;) => {$(
    )*
    }
}

impl_signed_conversions!(
    fi8_from_str_radix, i8;
);*/
