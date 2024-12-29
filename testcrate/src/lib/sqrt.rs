// TODO for a more serious implementation we would be using unsigned fracints
// and some offset translation

use std::{ cmp::max};

use fracints::prelude::*;
use star_rng::StarRng;

use crate::{mutate_fracint, FracintTemperature, Optimizeable, RampOptimize};

/*
any kind of polynomial or rational has horrible convergence for square roots, only degree 2
polynomials or simple ad-hoc methods are reasonable for getting a few bits of precision, and then
some iterative method should be used to converge on the value.

TODO

There should probably be a LUT of

Goldschmidt for sqrt(x) and isqrt(x) seems like the best approach for a pure integer iterative
method. It looks like:

y_0 =_approx 1/sqrt(S)
x_0 = S * y_0
h_0 = 0.5 * y_0

r_n = 0.5 - x_n * h_n
x_(n+1) = x_n + r_n * x_n
h_(n+1) = h_n + r_n * h_n

we can make this work in plain fracints for the domain [0.25, 1.0) by setting up the prelude with:

y_0 =_approx 1/sqrt(S) = 1 + f // f in [0.0, 1.0)
x_0 = S * (1 + f) = g // g in [0.0, 1.0)
                      // (if we ensure that the initial approx. is always an underestimation)
h_0 = 0.5 * (1 + f) = h // h in [0.0, 1.0)

// and 0.25 may need to be special cased

// then the rest of the steps are ideal fracints

the initial `f` value can be from a LUT or from carefully chosen polynomials (which could also
use a LUT for different intervals if we want to get a full 8 bits of initial approximation)

The domain [0.25, 1.0) can be extended for all [4^(a-1), 4^a) intervals with the transform

sqrt(x) = 2^a * sqrt((4^(-a)) * x)

which can be done with fast bitshifts

# For an example of calculating sqrt(0.11):

We first transform for the interval [4^(-2), 4^(-1)] with

sqrt(0.11) = 2^(-1) * sqrt((4^(--1)) * 0.11) = (0.5) * sqrt(0.44)

We now calculate sqrt(0.44) with Goldschmidt

y_0 =

*/

/// Optimizes for y = 1/sqrt(x) - 1 in a range `start..=end`, and tries to make
/// the output always underestimate the true value (TODO there is probably some
/// fast method of proving that a fracint polynomial never exceeds a value,
/// despite any calculation jaggedness)
#[derive(Debug, Clone)]
pub struct ISqrt<F: Fracint + FracintDouble> {
    pub offset: F,
    pub a0: F,
    pub a1: F,
    pub a2: F,
    pub start: F,
    pub end: F,
    pub n: F::Int,
}

// TODO we should be able to move the more accurate `sqrt_slow` to `Fracint` so
// we don't need `FracintDouble`
impl<F: Fracint + FracintDouble> ISqrt<F> {
    pub fn rand(start: F, end: F, n: F::Int, rng: &mut StarRng) -> Self {
        Self {
            offset: F::rand(rng).unwrap(),
            a0: F::rand(rng).unwrap(),
            a1: F::rand(rng).unwrap(),
            a2: F::rand(rng).unwrap(),
            start,
            end,
            n,
        }
    }

    pub fn eval(&self, mut x: F) -> F {
        // custom polynomial where we insert two left shifts in order to obtain the
        // required slope
        x = x.wrapping_add(self.offset);
        let mut res = self.a2.wrapping_mul(x);
        res = res << 1;
        res = res.wrapping_add(self.a1);
        res = res.wrapping_mul(x);
        res = res << 1;
        res = res.wrapping_add(self.a0);
        res = res << 1;
        res
    }

    pub fn isqrt_sub1(&self, x: F) -> F {
        // use f = (1-sqrt(x))/sqrt(x) which happens to work for our [0.25, 1.0) target
        // range
        let sqrt = x.widen().sqrt_simple_bisection();
        ((F::Double::ONE - sqrt) / sqrt).truncate()
    }

    pub fn error(&self, x: F) -> F {
        let y = self.eval(x);
        if y < F::ZERO {
            return F::MAX;
        }
        let expected_y = self.isqrt_sub1(x);
        let diff = expected_y.saturating_sub(y);
        if diff < F::ZERO {
            return F::MAX;
        }
        diff
        //let expected_x = self.expected_inv(y);
        //expected_x.saturating_sub(x).saturating_abs().as_int().try_into().
        // unwrap_or(u128::MAX)
    }
}

impl<F: Fracint + FracintDouble> Optimizeable for ISqrt<F> {
    type Cost = F;
    type Temperature = FracintTemperature;

    fn cost(&self) -> F {
        let mut res = F::ZERO;
        let step = (self.end - self.start).saturating_div_int(self.n);
        let n: u128 = self.n.try_into().unwrap();
        let mut x = self.start;
        for _ in 0..n {
            res = max(res, self.error(x));
            x += step;
        }
        // for the last one, make sure we get the max value (the division for `step`
        // truncates) so that our optimizer disfavors overflow edge cases, TODO again
        // there should be a more rigorous way of optimizing against the real truncation
        // arithmetic results while ensuring endpoints are perfect
        res = max(res, self.error(self.end));
        res
    }

    fn mutate(&mut self, rng: &mut StarRng, temp: &Self::Temperature) {
        match rng.index(4).unwrap() {
            0 => mutate_fracint(&mut self.offset, rng, temp),
            1 => mutate_fracint(&mut self.a0, rng, temp),
            2 => mutate_fracint(&mut self.a1, rng, temp),
            3 => mutate_fracint(&mut self.a2, rng, temp),
            _ => unreachable!(),
        }
    }
}

/// Calculates `1/sqrt(x)` for [0.25, 1.0]. Assumes `N` is of the form `3 *
/// (2^M)`.
pub struct ISqrtInitialLUT<F: Fracint, const N: usize>(pub [(F, F, F, F); N]);

impl<F: Fracint + FracintDouble, const N: usize> ISqrtInitialLUT<F, N> {
    /// we don't have stable const traits and have to use this method that will
    /// then produce the values actually placed in the LUT constant
    pub fn generate() -> (Vec<(F, F, F, F)>, F) {
        let seed = 5;
        let rng = &mut StarRng::new(seed);
        if !F::SIGNED {
            // also we would want to claim the extra bit
            todo!()
        }
        let mut res = vec![];
        // 0.25
        let mut start = F::ULP << (F::BITS - 3);
        // no error if in correct form
        let lb_step = (N / 3).trailing_zeros() as usize;
        let step = F::ULP << (F::BITS - lb_step - 3);
        // this seems to be all that is needed
        let n = 4.try_into().unwrap();

        // TODO the topology of this optimization must be much rougher than I expected
        // or I am doing something wrong, because it is the most finicky thing ever,
        // need a more rigorous way of doing this, perhaps the plain curve fitting
        // methods or a simple LUT would have worked but this is good enough for now

        // TODO nope we are just using the simple LUT
        for _ in 0..N {
            // have to add some total retries on top of all this
            let mut actual_best = None;
            let mut actual_worst_error = F::MAX;
            'outer: for i in 0..8 {
                let mut worst_error = F::ZERO;
                let end = start + step;

                let mut init = ISqrt::rand(start, end, n, rng);
                let init_best_cost = init.cost();
                for _ in 0..1000 {
                    let next = ISqrt::rand(start, end, n, rng);
                    let cost = next.cost();
                    if cost < init_best_cost {
                        init = next;
                    }
                }

                let mut ramp = RampOptimize::new(init, i, 128);
                for frozen_sig_add in 0..F::BITS {
                    for _ in 0..300 {
                        ramp.step(&FracintTemperature { frozen_sig_add });
                    }
                }
                let mut best = ramp.best();

                // we don't have a quick perfect method of insuring not overestimating (which
                // would result in catastrophic overflow which we definitely do not want), for
                // now just brute force subtract as necessary which is good for 16 bits
                let mut x = start;
                let mut worst_over = F::ZERO;
                loop {
                    if x >= end {
                        break
                    }

                    let expected_y = best.isqrt_sub1(x);
                    let diff = best.eval(x).saturating_sub(expected_y);
                    // `>=` because the truncated parts can break ties in the wrong way
                    if diff >= F::ZERO {
                        worst_over = max(worst_over, diff);
                    }
                    x += F::ULP;
                }

                // correct the constant term
                if worst_over > F::ZERO {
                    best.a0 -= worst_over + F::ULP;
                }

                // double check
                let mut x = start;
                loop {
                    if x >= end {
                        break
                    }

                    let expected_y = best.isqrt_sub1(x);
                    let y = best.eval(x);
                    if y >= expected_y {
                        continue 'outer;
                    }

                    worst_error = max(worst_error, expected_y - y);
                    x += F::ULP;
                }

                if worst_error < actual_worst_error {
                    actual_worst_error = worst_error;
                    actual_best = Some(best);
                }
            }
            let best = actual_best.unwrap();
            res.push((best.offset, best.a0, best.a1, best.a2));
            dbg!(res.last().unwrap(), actual_worst_error);

            start += step;
        }
        (res, F::ZERO)
    }
}

pub fn isqrt_sub1<F: Fracint + FracintDouble>(x: F) -> F {
    let sqrt = x.widen().sqrt_simple_bisection();
    ((F::Double::ONE - sqrt) / sqrt).truncate()
}

pub fn eval_simple_isqrt_lut(lut: &[fi16], bits: usize, x: fi16) -> fi16 {
    if x < fi16::ZERO {
        return fi16::ZERO
    }
    // we will find the interval `x` lies in and interpolate between the two LUT slots

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
        // y0*(1 - t)
        fi16::ONE.wrapping_sub(rem_inx).wrapping_mul(y0)
    }
}

pub fn simple_isqrt_lut(n: usize) -> (Vec<fi16>, usize) {
    assert!((n % 3) == 0);
    assert!((n / 3).is_power_of_two());
    assert!(n <= 4096);
    assert!(n >= 6);
    let bits = ((n / 3).trailing_zeros() as usize) + 2;
    let mut lut = vec![];
    let mut start = fi16!(0.25);
    let step = fi16!(0.75).saturating_div_int(n as i16);
    for _ in 0..n {
        lut.push(isqrt_sub1(start));
        start += step;
    }

    // make sure we always underestimate
    let mut x = fi16!(0.25);
    let mut worst_over = fi16!(0.0);
    loop {
        if x == fi16!(1.0) {
            break
        }
        let max_y = isqrt_sub1(x) - fi16::ULP.saturating_mul_int(2);
        let actual_y = eval_simple_isqrt_lut(&lut, bits, x);
        if actual_y > max_y {
            let over = actual_y - max_y;
            if over > worst_over {
                worst_over = over;
            }
        }

        x += fi16::ULP;
    }
    // we are just moving all of them down
    for y in &mut lut {
        *y -= worst_over;
    }

    // recalculate and check
    let mut x = fi16!(0.25);
    let mut worst_under = fi16!(0.0);
    loop {
        if x == fi16!(1.0) {
            break
        }
        let max_y = isqrt_sub1(x) - fi16::ULP;
        let actual_y = eval_simple_isqrt_lut(&lut, bits, x);
        if max_y < actual_y {
            dbg!(x, max_y, actual_y);
            //panic!()
        } else {
            let under = max_y - actual_y;
            if under > worst_under {
                worst_under = under;
            }
        }

        x += fi16::ULP;
    }
    dbg!(worst_under);

    (lut, bits)
}
