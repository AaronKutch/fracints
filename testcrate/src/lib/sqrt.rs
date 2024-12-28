// TODO for a more serious implementation we would be using unsigned fracints
// and some offset translation

use std::cmp::max;

use fracints::prelude::*;
use star_rng::StarRng;

use crate::{FracintTemperature, Optimizeable, Poly, RampOptimize};

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
    pub poly: Poly<F>,
    pub start: F,
    pub end: F,
    pub n: F::Int,
}

// TODO we should be able to move the more accurate `sqrt_slow` to `Fracint` so
// we don't need `FracintDouble`
impl<F: Fracint + FracintDouble> ISqrt<F> {
    pub fn eval(&self, x: F) -> F {
        self.poly.eval(x)
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
        self.poly.mutate(rng, temp);
    }
}

/// Calculates `1/sqrt(x)` for [0.25, 1.0]. Assumes `N` is of the form `3 *
/// (2^M)`.
pub struct ISqrtInitialLUT<F: Fracint, const N: usize>(pub [(F, F, F); N]);

impl<F: Fracint + FracintDouble, const N: usize> ISqrtInitialLUT<F, N> {
    /// we don't have stable const traits and have to use this method that will
    /// then produce the values actually placed in the LUT constant
    pub fn generate() -> (Vec<(F, F, F)>, F) {
        if !F::SIGNED {
            // also we would want to claim the extra bit
            todo!()
        }
        let mut res = vec![];
        let mut worst_error = F::ZERO;
        // 0.25
        let mut start = F::ULP << (F::BITS - 3);
        // no error if in correct form
        let lb_step = (N / 3).trailing_zeros() as usize;
        let step = F::ULP << (F::BITS - lb_step - 3);
        for _ in 0..N {
            let end = start + step;

            // it seems only a simple degree 2 polynomial is reasonable, others add too
            // little to be worth it
            let seed = 0;
            let init = ISqrt {
                poly: Poly::zero(3),
                start,
                end,
                n: 4.try_into().unwrap(), // this seems to be all that is needed
            };
            // also the topology must have many local minimums which I didn't expect or I
            // suspect I am doing something wrong, because it is not always a consistent
            // result across multiple seeds
            let mut ramp = RampOptimize::new(init, seed, 128);
            for frozen_sig_add in 0..F::BITS {
                for _ in 0..100 {
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
                best.poly.a[0] -= worst_over + F::ULP;
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
                    //panic!()
                }

                worst_error = max(worst_error, expected_y - y);
                x += F::ULP;
            }

            res.push((best.poly.a[0], best.poly.a[1], best.poly.a[2]));

            start += step;
        }
        (res, worst_error)
    }
}
