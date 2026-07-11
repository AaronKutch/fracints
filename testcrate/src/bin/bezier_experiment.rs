//! An experiment quantifying the "table of rational quadratic bezier arcs"
//! approach to `cos_sin`.
//!
//! A rational quadratic bezier with control points `P0 = (cos(a), -sin(a))`,
//! `P1 = (1/cos(a), 0)` (weight `cos(a)`), `P2 = (cos(a), sin(a))` traces the
//! unit circle arc from `-a` to `a` _exactly_, which is why the output vector
//! is normalized "for free" up to rounding. In the centered parameter
//! `s = 2t - 1` in [-1, 1] the curve is
//!
//! x(s) = (cos(a)*(1 + s^2) + (1 - s^2)) / d(s)
//! y(s) = 2*sin(a)*s / d(s)
//! d(s) = (1 + s^2) + (1 - s^2)*cos(a)
//!
//! and a little algebra collapses the traced angle to the closed form
//!
//! theta(s) = 2*atan(s*tan(a/2))
//!
//! so the parameter-to-angle map is only linear to leading order: using `s`
//! directly as "angle within the segment" has error `~ (a^3/12)*max|s - s^3|
//! = a^3/(12*sqrt(27))` that scales _cubically_ in the segment half-angle
//! `a`. This experiment measures that, showing that every doubling of an arc
//! table gains only 3 bits of angle accuracy, and also that substituting the
//! corrected parameter `s = tan(theta*a/2)/tan(a/2)` (the Weierstrass /
//! half-angle-tangent parameterization) makes the angle exact too.

pub fn main() {
    println!("max |theta(s) - s*a| over the segment, linear parameter:");
    println!(
        "{:>10} {:>14} {:>10} {:>16}",
        "half-angle", "max err (rad)", "bits", "err/a^3"
    );
    for k in 3..17 {
        let a = core::f64::consts::TAU / ((1u64 << k) as f64);
        let t = (a / 2.0).tan();
        let mut max_err: f64 = 0.0;
        let n = 100000;
        for i in 0..=n {
            let s = -1.0 + 2.0 * (i as f64) / (n as f64);
            let theta = 2.0 * (s * t).atan();
            max_err = max_err.max((theta - s * a).abs());
        }
        println!(
            "{:>10} {:>14.3e} {:>10.2} {:>16.6}",
            format!("tau/2^{k}"),
            max_err,
            max_err.log2(),
            max_err / (a * a * a),
        );
    }

    // the cubic coefficient approaches 1/(12*sqrt(27)) = 0.016038
    println!("\n1/(12*sqrt(27)) = {:.6}", 1.0 / (12.0 * 27.0f64.sqrt()));

    println!("\nwith the tan-corrected parameter s = tan(theta_frac*a/2)/tan(a/2):");
    for k in [3, 6, 9] {
        let a = core::f64::consts::TAU / ((1u64 << k) as f64);
        let t = (a / 2.0).tan();
        let mut max_err: f64 = 0.0;
        let mut max_norm_err: f64 = 0.0;
        let n = 100000;
        for i in 0..=n {
            let target = (-1.0 + 2.0 * (i as f64) / (n as f64)) * a;
            let s = (target / 2.0).tan() / t;
            let d = (1.0 + s * s) + (1.0 - s * s) * a.cos();
            let x = (a.cos() * (1.0 + s * s) + (1.0 - s * s)) / d;
            let y = 2.0 * a.sin() * s / d;
            max_err = max_err.max((y.atan2(x) - target).abs());
            max_norm_err = max_norm_err.max((x * x + y * y - 1.0).abs());
        }
        println!(
            "  tau/2^{k}: max angle err {:.3e} rad, max norm err {:.3e}",
            max_err, max_norm_err
        );
    }
}
