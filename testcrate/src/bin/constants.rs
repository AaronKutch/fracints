//! This is a constant generator for `constants.rs`.
//! This is ugly, but
//!  -it needs to be compatible with the macros
//!  -it needs to work on stable Rust
//!  -sooner or later I will probably run into something that cannot be evaluated as a const fn
//!  -one of the constants may take a long time to calculate
//!  -anything else will add more compilation time to this crate
//!  -some constants are dependent on other constants

use normints::*;
use std::str::FromStr;
use std::{i128, i16, i32, i64, i8};

// N.B.: Some constants rely on other constants. Copy the output into `constants.rs` and repeat
// until the constants no longer change.

// These macros calculate how many iterations the `cos_taudiv4_taylor` and
// `sin_taudiv4_taylor` algorithms should use to obtain the best precision possible for
// `theta = 0.5`
macro_rules! cos_taylor_iters {
    ($( $iX:ident ),*) => {{
        let mut tmp = Vec::new();
        $(
        // this is a reduced version of the algorithm with `theta = 0.5`
        let mut factorial_num: $iX = 2;
        let mut factorial_mul: $iX = 2;
        let mut numerator = $iX::MIN / -4; // normint 0.5 * 0.5
        let mut i = 0;
        loop {
            // Overflows will completely mess up the algorithm, and if the numerator is smaller than
            // `factorioal_mul`, the algorithm does nothing after that point. The second case
            // usually happens before overflow, but I include this for completeness.
            match factorial_mul.checked_mul(factorial_num) {
                Some(a) => {
                    factorial_mul = a;
                    factorial_num += 1;
                }
                None => break,
            }
            match factorial_mul.checked_mul(factorial_num) {
                Some(a) => {
                    factorial_mul = a;
                    factorial_num += 1;
                }
                None => break,
            }
            // multiply by 0.25
            numerator /= 4;
            if numerator < factorial_mul {break}
            i += 1;
        }
        tmp.push(i);
        )*
        tmp
    }};
}

macro_rules! sin_taylor_iters {
    ($( $iX:ident ),*) => {{
        let mut tmp = Vec::new();
        $(
        let mut factorial_num: $iX = 3;
        let mut factorial_mul: $iX = 6;
        let mut numerator = $iX::MIN / -4; // normint 0.5 * 0.5 * 0.5
        let mut i = 0;
        loop {
            match factorial_mul.checked_mul(factorial_num) {
                Some(a) => {
                    factorial_mul = a;
                    factorial_num += 1;
                }
                None => break,
            }
            match factorial_mul.checked_mul(factorial_num) {
                Some(a) => {
                    factorial_mul = a;
                    factorial_num += 1;
                }
                None => break,
            }
            numerator /= 4;
            if numerator < factorial_mul {break}
            i += 1;
        }
        tmp.push(i);
        )*
        tmp
    }};
}

macro_rules! from_ni_macro {
    ($s:expr) => {{
        vec![
            fi8::from_str($s).unwrap().0.to_string(),
            fi16::from_str($s).unwrap().0.to_string(),
            fi32::from_str($s).unwrap().0.to_string(),
            fi64::from_str($s).unwrap().0.to_string(),
            fi128::from_str($s).unwrap().0.to_string(),
        ]
    }};
}

fn main() {
    let shift = vec![8u64, 16, 32, 64, 128];
    let mut shiftstring: Vec<String> = Vec::new();
    for x in shift.clone() {
        shiftstring.push(x.to_string())
    }
    let num_4divtau = from_ni_macro!("0.6366197723675813430755350534900574481378");
    let num_4divtau_sqr = from_ni_macro!("0.4052847345693510857755178528389105556174");
    let sqrt2div2 = from_ni_macro!("0.7071067811865475244008443621048490392848");
    let sqrt2minus1 = from_ni_macro!("0.414213562373095048801688724209698078569");
    let cos2pidiv16 = from_ni_macro!("0.92387953251128675612818318939678828682241");
    let cos_taylor_iters = cos_taylor_iters!(i8, i16, i32, i64, i128);
    let sin_taylor_iters = sin_taylor_iters!(i8, i16, i32, i64, i128);

    for (i, _) in shift.iter().enumerate() {
        println!("pub struct Const{0} {{\n    pub num_4divtau: ni{0},\n    pub num_4divtau_sqr: ni{0},\n    pub sqrt2div2: ni{0},\n    pub sqrt2minus1: ni{0},\n    pub costaudiv16: ni{0},\n    pub cos_taylor_iters: usize,\n    pub sin_taylor_iters: usize,\n}}\n", shiftstring[i]);
        println!(
            "pub const CONST{0}: Const{0} = Const{0} {{\n    num_4divtau: ni{0}({1}),\n    num_4divtau_sqr: ni{0}({2}),\n    sqrt2div2: ni{0}({3}),\n    sqrt2minus1: ni{0}({4}),\n    costaudiv16: ni{0}({5}),\n    cos_taylor_iters: {6},\n    sin_taylor_iters: {7},\n}};\n",
            shiftstring[i],
            num_4divtau[i],
            num_4divtau_sqr[i],
            sqrt2div2[i],
            sqrt2minus1[i],
            cos2pidiv16[i],
            cos_taylor_iters[i],
            sin_taylor_iters[i]
        );
    }
}
