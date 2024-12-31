use fracints_internals::traits::*;

use crate::fi16;

pub fn eval_simple_isqrt_lut(lut: &[fi16], cutoff: fi16, bits: usize, x: fi16) -> fi16 {
    // the cutoff is needed because the last interval special case cannot
    // underestimate without subtracting a lot from the last LUT entry
    if (x < fi16::ZERO) || (x >= cutoff) {
        return fi16::ZERO
    }
    // we will find the interval `x` lies in and interpolate between the two LUT
    // slots

    // find the index in the LUT so we find the two sides of an interval
    let x_i = x.as_int() as u16;
    let inx0 = x_i >> (16 - 1 - bits);
    let rem = x_i.wrapping_sub(inx0 << (16 - 1 - bits));
    // the fractional point within that interval
    let rem_inx = fi16::from_int((rem << bits) as i16);
    // adjust for the 0.25 start
    let inx0 = inx0.wrapping_sub(0b1 << (bits - 2));
    let y0 = lut[inx0 as usize];
    let inx1 = inx0.wrapping_add(1);
    if (inx1 as usize) < lut.len() {
        let y1 = lut[inx1 as usize];
        // (y1 - y0)*t + y0
        y1.wrapping_sub(y0).wrapping_mul(rem_inx).wrapping_add(y0)
    } else {
        // y0 - (y0*t)
        y0.wrapping_sub(y0.wrapping_mul(rem_inx))
    }
}

pub fn goldschmidt<F: Fracint>(s: F, f: F, n: usize) -> F {
    let half = (F::ONE >> 1).wrapping_add(F::ULP);

    let mut r;
    let mut g = f.wrapping_mul(s).wrapping_add(s);
    let mut h = half.wrapping_add(f >> 1);

    for i in 0..n {
        r = half.wrapping_sub(g.wrapping_mul(h));
        g = r.wrapping_mul(g).wrapping_add(g);
        if i == (n - 1) {
            // don't need the last step
            break
        }
        h = r.wrapping_mul(h).wrapping_add(h);
    }
    g
}
