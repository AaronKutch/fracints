#![allow(clippy::reversed_empty_ranges)]

use core::{
    fmt,
    iter::{Product, Sum},
    ops::*,
    primitive::*,
    result::Result,
    str::FromStr,
};

use awint::awint_internals::{dd_division_u256, widening_mul_add_u128};
use fracints_internals::{impl_signed, *};
#[cfg(feature = "rand")]
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{constants::*, Fracint};

macro_rules! impl_signed1 {
    ($(
        $ty:ident,
        $s:expr,
        $iX:ident,
        $uX:ident,
        $iD:ident,
        $to_string:ident,
        $from_str:ident,
        $c:expr
    );*;) => {$(
        impl_signed!(
            $ty,
            $s,
            $iX,
            $uX,
            $to_string,
            $from_str,
            |a: $iX, b: $iX| (($iD::from(a) * $iD::from(b)) >> ($uX::BITS - 1)) as $iX,
            |a: $iX, b: $iX| (($iD::from(a) << ($uX::BITS - 1)) / $iD::from(b)) as $iX,
            $c
        );
    )*};
}

impl_signed1!(
    fi8, "fi8", i8, u8, i16, i8_to_string, i8_from_str, CONST8;
    fi16, "fi16", i16, u16, i32, i16_to_string, i16_from_str, CONST16;
    fi32, "fi32", i32, u32, i64, i32_to_string, i32_from_str, CONST32;
    fi64, "fi64", i64, u64, i128, i64_to_string, i64_from_str, CONST64;
);
// the 128 bit case needs special handling for the widening multiplies
impl_signed!(
    fi128,
    "fi128",
    i128,
    u128,
    i128_to_string,
    i128_from_str,
    |mut lhs: i128, mut rhs: i128| {
        let lhs_msb = lhs < 0;
        let rhs_msb = rhs < 0;
        if lhs_msb {
            lhs = lhs.wrapping_neg();
        }
        if rhs_msb {
            rhs = rhs.wrapping_neg();
        }
        let lhs = lhs as u128;
        let rhs = rhs as u128;

        let (lo, hi) = widening_mul_add_u128(lhs, rhs, 0);
        // because of the shift, we retain one bit from `lo` TODO see if we could
        // optimize
        let mut res = (hi << 1) | (lo >> 127);

        if lhs_msb != rhs_msb {
            res = res.wrapping_neg();
        }
        res as i128
    },
    |mut lhs: i128, mut rhs: i128| {
        let lhs_msb = lhs < 0;
        let rhs_msb = rhs < 0;
        if lhs_msb {
            lhs = lhs.wrapping_neg();
        }
        if rhs_msb {
            rhs = rhs.wrapping_neg();
        }
        let lhs = lhs as u128;
        let rhs = rhs as u128;

        let lo = lhs << 127;
        let hi = lhs >> 1;
        let (quo, _) = dd_division_u256((lo, hi), (rhs, 0));
        let mut quo = quo.0;

        /*
        if lhs_msb {
            rem = rem.wrapping_neg();
        }
        */

        if lhs_msb != rhs_msb {
            quo = quo.wrapping_neg();
        }
        quo as i128
    },
    CONST128
);
