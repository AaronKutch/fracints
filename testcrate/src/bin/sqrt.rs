use common::{FracintTemperature, Optimizeable, RampOptimize, Rational, Sqrt};
use fracints::prelude::*;

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

sqrt(x) = sqrt(4^a) * sqrt((4^(-a)) * x)

where `sqrt(4^a)` can be precomputed in a BITS/2 sized LUT

and this all must be done in a truncating square root if we want the less significant
bits to be correct

# For an example of calculating sqrt(0.11):

We first transform for the interval [4^(-2), 4^(-1)] with

sqrt(0.11) = sqrt(4^(-1)) * sqrt((4^(--1)) * 0.11) = (0.5) * sqrt(0.44)

We now calculate sqrt(0.44) with Goldschmidt

y_0 =

*/

fn main() {
    // it seems only a degree 2 polynomial is reasonable, others add too little to
    // be worth it
    let seed = 0;
    let start = fi16!(0.5);
    let end = fi16!(0.53125);
    let init = Sqrt {
        rational: Rational::zero(3, 0),
        start,
        end,
        n: 4, // this seems to be all that is needed
    };
    // also the topology must have many local minimums which I didn't expect or I am
    // doing something wrong
    let mut ramp = RampOptimize::new(init, seed, 128);
    for frozen_sig_add in 0..fi16::BITS {
        for _ in 0..100 {
            ramp.step(&FracintTemperature { frozen_sig_add });
        }
    }
    let best = ramp.best();
    dbg!(&best);
    dbg!(best.cost());

    // dummy way to estimate the worst error, there is a better way that measures
    // with more rigorous analysis that uses the structure of fixed point
    // polynomials
    let mut worst_x = start;
    let mut worst_diff = worst_x
        .sqrt_simple_bisection()
        .saturating_sub(best.eval(worst_x));
    let points = 1000;
    let inc = (end - start).saturating_div_int(points);
    let mut x = start;
    for _ in 0..points {
        let diff = x.sqrt_simple_bisection().saturating_sub(best.eval(x));
        if diff > worst_diff {
            worst_x = x;
            worst_diff = diff;
        }
        x += inc;
    }
    dbg!(worst_x, worst_diff);
    dbg!(best.eval(fi16!(0.5)));
    dbg!(best.eval(fi16!(0.75)));
    dbg!(best.eval(fi16!(1.0)));
}
