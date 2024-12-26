use common::{FracintTemperature, Optimizeable, RampOptimize, Rational, Sqrt};
use fracints::prelude::*;

fn main() {
    let seed = 0;
    let start = fi64!(0.5);
    let end = fi64!(1.0);
    let init = Sqrt {
        rational: Rational::zero(3, 0),
        start,
        end,
        n: 4,
    };
    let mut ramp = RampOptimize::<Sqrt<fi64>>::new(init, seed, 1024);
    for frozen_sig_add in 0..64 {
        for _ in 0..100 {
            ramp.step(&FracintTemperature { frozen_sig_add });
        }
    }
    let best = ramp.best();
    dbg!(&best);
    dbg!(best.cost());

    // dummy way to estimate the worst error, there is a better way that measures
    // the `y` (just have the bisection calculation method) and more rigorous
    // analysis that uses the structure of fixed point polynomials
    let mut worst_x = start;
    let tmp = best.eval(worst_x);
    let mut worst_diff = (worst_x - best.expected_inv(tmp)).saturating_abs();
    let points = 1000;
    let inc = (end - start).saturating_div_int(points);
    let mut x = start;
    for _ in 0..points {
        let next_y = best.eval(x);
        let diff = (x - best.expected_inv(next_y)).saturating_abs();
        if diff > worst_diff {
            worst_x = x;
            worst_diff = diff;
        }
        x += inc;
    }
    dbg!(worst_x, worst_diff);
    dbg!(best.eval(fi64!(0.5)));
    dbg!(best.eval(fi64!(0.75)));
    dbg!(best.eval(fi64!(1.0)));
}
