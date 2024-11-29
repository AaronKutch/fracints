use core::{
    fmt,
    iter::{Product, Sum},
    ops::*,
    primitive::*,
    result::Result,
    str::FromStr,
};

use fracints_internals::{impl_signed, *};
#[cfg(feature = "rand")]
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::constants::*;

macro_rules! impl_signed1 {
    (
        $ty:ident,
        $s:expr,
        $iX:ident,
        $uX:ident,
        $iD:ident,
        $to_string:ident,
        $from_str:ident,
        $c:expr
    ) => {
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
    };
}

impl_signed1!(fi8, "fi8", i8, u8, i16, i8_to_string, i8_from_str, CONST8);
/*
impl_fiN_2!(fi8, fi16, i8, u8, i16, u16, 7, 8, CONST8, CONST16);

impl_fiN_1!(
    fi16,
    "fi16",
    i16,
    u16,
    i32,
    16,
    |x: ApInt| fi16(x.resize_to_i16()),
    fi16_ok,
    CONST16
);

impl_fiN_2!(fi16, fi32, i16, u16, i32, u32, 15, 16, CONST16, CONST32);

impl_fiN_1!(
    fi32,
    "fi32",
    i32,
    u32,
    i64,
    32,
    |x: ApInt| fi32(x.resize_to_i32()),
    fi32_ok,
    CONST32
);

impl_fiN_2!(fi32, fi64, i32, u32, i64, u64, 31, 32, CONST32, CONST64);

impl_fiN_1!(
    fi64,
    "fi64",
    i64,
    u64,
    i128,
    64,
    |x: ApInt| fi64(x.resize_to_i64()),
    fi64_ok,
    CONST64
);

impl_fiN_2!(fi64, fi128, i64, u64, i128, u128, 63, 64, CONST64, CONST128);*/

// TODO there is a mul and div routine from `awint_internals`
/*
impl_fiN_0!(
    fi128,
    "fi128",
    i128,
    u128,
    127,
    128,
    |a: i128, b: i128| {
        let mut lhs = ApInt::from(a);
        lhs.sign_resize(256);
        let mut rhs = ApInt::from(b);
        rhs.sign_resize(256);
        lhs.wrapping_mul_assign(&rhs).unwrap();
        lhs.wrapping_ashr_assign(127).unwrap();
        lhs.resize_to_i128()
    },
    |a: i128, b: i128| {
        let mut lhs = ApInt::from(a);
        lhs.sign_resize(256);
        let mut rhs = ApInt::from(b);
        rhs.sign_resize(256);
        lhs.wrapping_shl_assign(127).unwrap();
        lhs.wrapping_sdiv_assign(&rhs).unwrap();
        lhs.resize_to_i128()
    },
    |x: ApInt| fi128(x.resize_to_i128()),
    fi128_ok,
    CONST128
);
*/
