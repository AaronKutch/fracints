use std::str::FromStr;

use fracints::*;
use stacked_errors::{Result, ensure_eq};

macro_rules! basic_cases {
    ($ty:ident, $iX:ident) => {
        ensure_eq!($ty($iX::MIN / 2).to_string(), "-0.5".to_string());
        ensure_eq!($ty($iX::MAX / 2 + 1).to_string(), "0.5".to_string());
        ensure_eq!($ty::from_str("-0.5").unwrap(), $ty($iX::MIN / 2));
        ensure_eq!($ty::from_str("0.5").unwrap(), $ty($iX::MAX / 2 + 1));
        ensure_eq!($ty::from_str("-0.5").unwrap(), $ty($iX::MIN / 2));
        ensure_eq!($ty::MIN.to_string(), "-1.0".to_string());
        ensure_eq!($ty::NEG_ONE.to_string(), "-1.0".to_string());
        ensure_eq!($ty::ZERO.to_string(), "0.0".to_string());
        ensure_eq!($ty::ONE.to_string(), "1.0".to_string());
        ensure_eq!($ty::MAX.to_string(), "1.0".to_string());
        ensure_eq!($ty::from_str("-1.0").unwrap(), $ty::NEG_ONE);
        ensure_eq!($ty::from_str("0.0").unwrap(), $ty::ZERO);
        ensure_eq!($ty::from_str("1.0").unwrap(), $ty::ONE);

        ensure_eq!($ty($iX::MAX / 4 + 1) + $ty($iX::MAX / 4), $ty($iX::MAX / 2));
        ensure_eq!($ty($iX::MIN / 4) + $ty($iX::MIN / 4), $ty($iX::MIN / 2));
        ensure_eq!($ty($iX::MAX / 4 + 1) + $ty($iX::MIN / 4), $ty(0));
        ensure_eq!($ty($iX::MIN / 4) + $ty($iX::MAX / 4 + 1), $ty(0));
        ensure_eq!($ty($iX::MAX / 4) - $ty($iX::MAX / 4), $ty(0));
        ensure_eq!($ty($iX::MIN / 4) - $ty($iX::MIN / 4), $ty(0));
        ensure_eq!(
            $ty($iX::MAX / 4 + 1) - $ty($iX::MIN / 4),
            $ty($iX::MAX / 2 + 1)
        );
        ensure_eq!($ty($iX::MIN / 4) - $ty($iX::MAX / 4 + 1), $ty($iX::MIN / 2));

        ensure_eq!($ty($iX::MAX).wrapping_add($ty($iX::MAX)), $ty(-2));
        ensure_eq!($ty($iX::MAX).wrapping_add($ty($iX::MIN)), $ty(-1));
        ensure_eq!($ty($iX::MIN).wrapping_add($ty($iX::MAX)), $ty(-1));
        ensure_eq!($ty($iX::MIN).wrapping_add($ty($iX::MIN)), $ty(0));
        ensure_eq!($ty($iX::MAX).wrapping_sub($ty($iX::MAX)), $ty(0));
        ensure_eq!($ty($iX::MAX).wrapping_sub($ty($iX::MIN)), $ty(-1));
        ensure_eq!($ty($iX::MIN).wrapping_sub($ty($iX::MAX)), $ty(1));
        ensure_eq!($ty($iX::MIN).wrapping_sub($ty($iX::MIN)), $ty(0));

        ensure_eq!($ty($iX::MIN / 2) * $ty($iX::MIN / 2), $ty($iX::MAX / 4 + 1));
        ensure_eq!($ty($iX::MIN / 2) * $ty($iX::MAX / 2 + 1), $ty($iX::MIN / 4));
        ensure_eq!($ty($iX::MAX / 2 + 1) * $ty($iX::MIN / 2), $ty($iX::MIN / 4));
        ensure_eq!(
            $ty($iX::MAX / 2 + 1) * $ty($iX::MAX / 2 + 1),
            $ty($iX::MAX / 4 + 1)
        );

        ensure_eq!(
            $ty($iX::MIN / 2).saturating_div($ty($iX::MIN / 2)),
            $ty::ONE
        );
        ensure_eq!(
            $ty($iX::MIN / 2).saturating_div($ty($iX::MAX / 2 + 1)),
            $ty::NEG_ONE
        );
        ensure_eq!(
            $ty($iX::MAX / 2 + 1).saturating_div($ty($iX::MAX / 2 + 1)),
            $ty::ONE
        );
        ensure_eq!(
            $ty($iX::MAX / 4 + 1).saturating_div($ty($iX::MAX / 2 + 1)),
            $ty($iX::MAX / 2 + 1)
        );

        ensure_eq!($ty($iX::MIN).wrapping_mul($ty($iX::MIN)), $ty::MIN);
        ensure_eq!($ty($iX::MIN).wrapping_mul($ty($iX::MAX)), $ty::NEG_ONE);
        ensure_eq!($ty($iX::MAX).wrapping_mul($ty($iX::MIN)), $ty::NEG_ONE);
        ensure_eq!($ty($iX::MAX).wrapping_mul($ty($iX::MAX)), $ty($iX::MAX - 1));
        ensure_eq!($ty::MIN.saturating_div($ty::NEG_ONE), $ty::ONE);
        ensure_eq!($ty::MIN.saturating_div($ty::MIN), $ty::ONE);
        ensure_eq!($ty::MIN.saturating_div($ty::NEG_ONE), $ty::ONE);
        ensure_eq!($ty::NEG_ONE.saturating_div($ty::MIN), $ty::ONE);
        ensure_eq!($ty::NEG_ONE.saturating_div($ty::NEG_ONE), $ty::ONE);
        ensure_eq!($ty::MIN.saturating_div($ty::MAX), $ty::NEG_ONE);
        ensure_eq!($ty::NEG_ONE.saturating_div($ty::MAX), $ty::NEG_ONE);
        ensure_eq!($ty::MAX.saturating_div($ty::MIN), $ty::NEG_ONE);
        ensure_eq!($ty::MAX.saturating_div($ty::NEG_ONE), $ty::NEG_ONE);
        ensure_eq!($ty::MAX.saturating_div($ty::MAX), $ty::ONE);
    };
}

#[cfg(test)]
#[test]
fn basic_cases() -> Result<()> {
    basic_cases!(fi8, i8);
    basic_cases!(fi16, i16);
    basic_cases!(fi32, i32);
    basic_cases!(fi64, i64);
    basic_cases!(fi128, i128);
    Ok(())
}
