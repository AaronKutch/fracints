use common::trig_vectors::*;
use fracints::prelude::*;
use star_rng::StarRng;

// The `_ACC` bounds are in ULPs of the type. The `_FAST` bounds are dominated
// by the minimax polynomial truncation errors, which are absolute: the degree
// 2 polynomials used by fi8 and fi16 have max error around 1.0e-5, and the
// degree 3 polynomials used by fi32 and larger around 2.8e-8, so for the large
// widths the fast bounds are computed from those.

macro_rules! exhaustive {
    ($test:ident, $ty:ident, $iX:ident, $bound_acc:expr, $bound_fast:expr) => {
        #[test]
        fn $test() {
            let scale = ((1u128 << ($ty::BITS - 1)) as f64);
            let max = (1u128 << ($ty::BITS - 1)) as i128 - 1;
            let mut max_acc = 0u128;
            let mut max_fast = 0u128;
            for x in $iX::MIN..=$iX::MAX {
                let theta = core::f64::consts::PI * ((x as f64) / scale);
                let (c_ref, s_ref) = (theta.cos(), theta.sin());
                let c_ref = ((c_ref * scale).round_ties_even() as i128).clamp(-max, max);
                let s_ref = ((s_ref * scale).round_ties_even() as i128).clamp(-max, max);
                let (c, s) = $ty(x).cos_sin_pi();
                max_acc = std::cmp::max(max_acc, (c.0 as i128).abs_diff(c_ref));
                max_acc = std::cmp::max(max_acc, (s.0 as i128).abs_diff(s_ref));
                let (c, s) = $ty(x).cos_sin_pi_fast();
                max_fast = std::cmp::max(max_fast, (c.0 as i128).abs_diff(c_ref));
                max_fast = std::cmp::max(max_fast, (s.0 as i128).abs_diff(s_ref));
            }
            println!(
                "{} max ULP error: accurate {max_acc}, fast {max_fast}",
                stringify!($ty)
            );
            assert!(max_acc <= $bound_acc);
            assert!(max_fast <= $bound_fast);
        }
    };
}

exhaustive!(cos_sin_pi_fi8_exhaustive, fi8, i8, 1, 2);
exhaustive!(cos_sin_pi_fi16_exhaustive, fi16, i16, 1, 3);

macro_rules! vectors {
    ($test:ident, $ty:ident, $vec:ident, $bound_acc:expr, $bound_fast:expr) => {
        #[test]
        fn $test() {
            let mut max_acc = 0u128;
            let mut max_fast = 0u128;
            for &(x, c_ref, s_ref) in $vec.iter() {
                let (c, s) = $ty(x).cos_sin_pi();
                max_acc = std::cmp::max(max_acc, (c.0 as i128).abs_diff(c_ref as i128));
                max_acc = std::cmp::max(max_acc, (s.0 as i128).abs_diff(s_ref as i128));
                let (c, s) = $ty(x).cos_sin_pi_fast();
                max_fast = std::cmp::max(max_fast, (c.0 as i128).abs_diff(c_ref as i128));
                max_fast = std::cmp::max(max_fast, (s.0 as i128).abs_diff(s_ref as i128));
            }
            println!(
                "{} max ULP error: accurate {max_acc}, fast {max_fast}",
                stringify!($ty)
            );
            assert!(max_acc <= $bound_acc);
            assert!(max_fast <= $bound_fast);
        }
    };
}

vectors!(cos_sin_pi_fi32_vectors, fi32, FI32_COS_SIN_PI, 1, 70);
vectors!(
    cos_sin_pi_fi64_vectors,
    fi64,
    FI64_COS_SIN_PI,
    1,
    (3.0e-8 * (1u128 << 63) as f64) as u128
);
vectors!(
    cos_sin_pi_fi128_vectors,
    fi128,
    FI128_COS_SIN_PI,
    8,
    (3.0e-8 * (1u128 << 127) as f64) as u128
);

macro_rules! rad_vectors {
    ($test:ident, $ty:ident, $vec:ident, $bound_acc:expr) => {
        #[test]
        fn $test() {
            let mut max_acc = 0u128;
            for &(x, c_ref, s_ref) in $vec.iter() {
                let (c, s) = $ty(x).cos_sin_rad();
                max_acc = std::cmp::max(max_acc, (c.0 as i128).abs_diff(c_ref as i128));
                max_acc = std::cmp::max(max_acc, (s.0 as i128).abs_diff(s_ref as i128));
            }
            println!("{} rad max ULP error: {max_acc}", stringify!($ty));
            assert!(max_acc <= $bound_acc);
        }
    };
}

rad_vectors!(cos_sin_rad_fi64_vectors, fi64, FI64_COS_SIN_RAD, 3);
rad_vectors!(cos_sin_rad_fi128_vectors, fi128, FI128_COS_SIN_RAD, 8);

/// exact values at the axes and the eighth turn boundaries
macro_rules! special_values {
    ($test:ident, $ty:ident, $sqrt_half:expr) => {
        #[test]
        fn $test() {
            let sqrt_half = $ty($sqrt_half);
            let quarter = $ty($ty::MIN.0 / -2);
            let eighth = $ty($ty::MIN.0 / -4);
            let f = $ty::cos_sin_pi;
            assert_eq!(f($ty::ZERO), ($ty::ONE, $ty::ZERO));
            assert_eq!(f(quarter), ($ty::ZERO, $ty::ONE));
            assert_eq!(f($ty::MIN), ($ty::NEG_ONE, $ty::ZERO));
            assert_eq!(f(quarter.wrapping_neg()), ($ty::ZERO, $ty::NEG_ONE));
            // the eighth turn boundaries hit the `t == MIN` special case
            // in half of the quadrant reductions
            assert_eq!(f(eighth), (sqrt_half, sqrt_half));
            assert_eq!(
                f(eighth.wrapping_neg()),
                (sqrt_half, sqrt_half.wrapping_neg())
            );
            assert_eq!(
                f($ty(quarter.0 + eighth.0)),
                (sqrt_half.wrapping_neg(), sqrt_half)
            );
            assert_eq!(
                f($ty(-quarter.0 - eighth.0)),
                (sqrt_half.wrapping_neg(), sqrt_half.wrapping_neg())
            );
            // the fast variant does not interpolate the axes exactly (its
            // constant term is a minimax coefficient), but the sines at the
            // axes and everything at the eighth turn boundaries are exact
            let f = $ty::cos_sin_pi_fast;
            assert_eq!(f($ty::ZERO).1, $ty::ZERO);
            assert_eq!(f(quarter).0, $ty::ZERO);
            assert_eq!(f(eighth), (sqrt_half, sqrt_half));
            // the tau parameterization is a wrapping doubling of the pi one
            assert_eq!(quarter.cos_sin_tau(), ($ty::NEG_ONE, $ty::ZERO));
            assert_eq!(eighth.cos_sin_tau(), ($ty::ZERO, $ty::ONE));
            assert_eq!($ty::MIN.cos_sin_tau(), ($ty::ONE, $ty::ZERO));
            assert_eq!($ty(eighth.0 / 2).cos_sin_tau(), (sqrt_half, sqrt_half));
            // radians at zero
            assert_eq!($ty::ZERO.cos_sin_rad(), ($ty::ONE, $ty::ZERO));
            assert_eq!($ty::ZERO.cos_sin_rad_fast().1, $ty::ZERO);
        }
    };
}

special_values!(cos_sin_special_fi8, fi8, 91);
special_values!(cos_sin_special_fi16, fi16, 23170);
special_values!(cos_sin_special_fi32, fi32, 1518500250);
special_values!(cos_sin_special_fi64, fi64, 6521908912666391106);
special_values!(
    cos_sin_special_fi128,
    fi128,
    120307984584002255772516886238812528464
);

/// `cos^2 + sin^2 == 1` to within a few ULPs. The exact products are computed
/// in plain integers because the sum can slightly exceed 1.0. The error is
/// reported in ULPs of the base type (the norm moves by up to ~2 ULP per ULP
/// of error in `c` or `s`).
macro_rules! norm {
    ($test:ident, $ty:ident, $bound_acc:expr, $bound_fast:expr) => {
        #[test]
        fn $test() {
            let one = 1i128 << (2 * ($ty::BITS - 1));
            let norm_err = |(c, s): ($ty, $ty)| -> u128 {
                let norm = (c.0 as i128) * (c.0 as i128) + (s.0 as i128) * (s.0 as i128);
                norm.abs_diff(one) >> ($ty::BITS - 1)
            };
            let rng = &mut StarRng::new(0);
            let mut max_acc = 0u128;
            let mut max_fast = 0u128;
            for _ in 0..1000 {
                let x = $ty::rand(rng);
                max_acc = std::cmp::max(max_acc, norm_err(x.cos_sin_pi()));
                max_fast = std::cmp::max(max_fast, norm_err(x.cos_sin_pi_fast()));
            }
            println!(
                "{} norm error: accurate {max_acc}, fast {max_fast}",
                stringify!($ty)
            );
            assert!(max_acc <= $bound_acc);
            assert!(max_fast <= $bound_fast);
        }
    };
}

norm!(cos_sin_norm_fi32, fi32, 3, 150);
norm!(
    cos_sin_norm_fi64,
    fi64,
    3,
    (2.0e-7 * (1u128 << 63) as f64) as u128
);

/// fi64 against fi128 at the same angles, catching problems below the f64
/// reference resolution
#[test]
fn cos_sin_cross_width() {
    let rng = &mut StarRng::new(1);
    let mut max: u128 = 0;
    for _ in 0..1000 {
        let x = fi64::rand(rng);
        let (c64, s64) = x.cos_sin_pi();
        let (c128, s128) = x.widen().cos_sin_pi();
        // round the fi128 result to fi64
        let round = |x: fi128| fi64(((x.0 + (1i128 << 63)) >> 64) as i64);
        max = max.max(c64.0.abs_diff(round(c128).0) as u128);
        max = max.max(s64.0.abs_diff(round(s128).0) as u128);
    }
    println!("fi64 vs fi128 max ULP difference: {max}");
    assert!(max <= 2);
}

/// radians against f64 for the widths without dedicated radian vectors
#[test]
fn cos_sin_rad_f64_check() {
    let rng = &mut StarRng::new(2);
    let mut max: f64 = 0.0;
    for _ in 0..10000 {
        let x = fi32::rand(rng);
        let theta = x.to_f64();
        let (c, s) = x.cos_sin_rad();
        max = max.max((c.to_f64() - theta.cos()).abs());
        max = max.max((s.to_f64() - theta.sin()).abs());
    }
    println!("fi32 rad max abs error: {max:e}");
    // a few ULPs of fi32
    assert!(max <= 4.0 / (1u64 << 31) as f64);
}
