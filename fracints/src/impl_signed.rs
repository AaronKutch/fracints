#![allow(clippy::reversed_empty_ranges)]

use core::{
    fmt,
    iter::{Product, Sum},
    ops::*,
    primitive::*,
    result::Result,
    str::FromStr,
};

use awint::{
    Bits, InlAwi,
    awint_internals::{u256_div_rem, widening_mul_add_u128},
    fp::{F32, F64, FP},
    inlawi_ty,
};
use fracints_internals::{impl_signed, *};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Fracint, FracintDouble, FracintHalf, constants::*, internal::*};

macro_rules! sqrt_fast {
    ($name:ident, $ty:ident, $n:expr, $truncate:tt, $widen:tt) => {
        fn $name(mut s: $ty) -> $ty {
            if s <= $ty::ZERO {
                return $ty::ZERO
            }

            // get the shift amount
            let lz = s.as_int().leading_zeros();
            let mut shift = 0;
            if lz >= 3 {
                shift = ((lz - 1) / 2) as usize;
            }
            s <<= shift * 2;

            // initial estimation
            let f = eval_simple_isqrt_lut(
                &SIMPLE_ISQRT_LUT,
                SIMPLE_ISQRT_CUTOFF,
                SIMPLE_ISQRT_BITS,
                ($truncate)(s),
            );
            let f = ($widen)(f);

            // iterative method
            let tmp = goldschmidt(s, f, $n);

            // shift back half the original
            tmp >> shift
        }
    };
}

fn sqrt_fast_fi8(s: fi8) -> fi8 {
    s.sqrt_slow()
}

sqrt_fast!(sqrt_fast_fi16, fi16, 1, { |s: fi16| s }, { |f: fi16| f });

sqrt_fast!(sqrt_fast_fi32, fi32, 2, { |s: fi32| s.truncate() }, {
    |f: fi16| f.widen()
});

sqrt_fast!(
    sqrt_fast_fi64,
    fi64,
    3,
    { |s: fi64| s.truncate().truncate() },
    { |f: fi16| f.widen().widen() }
);

sqrt_fast!(
    sqrt_fast_fi128,
    fi128,
    4,
    { |s: fi128| s.truncate().truncate().truncate() },
    { |f: fi16| f.widen().widen().widen() }
);

macro_rules! impl_signed1 {
    ($(
        $ty:ident,
        $s:expr,
        $iX:ident,
        $uX:ident,
        $iD:ident,
        $to_string:ident,
        $from_str:ident,
        $sqrt_fast:ident,
        $n:expr,
        $to_int:ident,
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
            $sqrt_fast,
            $n,
            $to_int,
            $c
        );
    )*};
}

impl_signed1!(
    fi8, "fi8", i8, u8, i16, i8_to_string, i8_from_str, sqrt_fast_fi8, 8, to_i8, CONST8;
    fi16, "fi16", i16, u16, i32, i16_to_string, i16_from_str, sqrt_fast_fi16, 16, to_i16, CONST16;
    fi32, "fi32", i32, u32, i64, i32_to_string, i32_from_str, sqrt_fast_fi32, 32, to_i32, CONST32;
    fi64, "fi64", i64, u64, i128, i64_to_string, i64_from_str, sqrt_fast_fi64, 64, to_i64, CONST64;
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
        // because of the shift, we retain one bit from `lo`
        // TODO see if we could optimize
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
        let (quo, _) = u256_div_rem((lo, hi), (rhs, 0));
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
    sqrt_fast_fi128,
    128,
    to_i128,
    CONST128
);

impl_signed_double!(fi8, fi16, i8, u8, i16, u16);
impl_signed_double!(fi16, fi32, i16, u16, i32, u32);
impl_signed_double!(fi32, fi64, i32, u32, i64, u64);
impl_signed_double!(fi64, fi128, i64, u64, i128, u128);
