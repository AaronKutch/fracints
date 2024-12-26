use fracints::Fracint;
use star_rng::StarRng;

use crate::Optimizeable;

// typically I would go for bit flipping, bit this is important for switching
// between small positives and negatives

pub struct FracintTemperature {
    /// Fracints are mutated by adding a random value with some number of
    /// significant bits (randomly uses negatives for signed fracints). This
    /// tells the number of leading bits of this additional value that are
    /// "frozen" to zero.
    pub frozen_sig_add: usize,
}

pub fn mutate_fracint<F: Fracint>(f: &mut F, rng: &mut StarRng, temp: &FracintTemperature) {
    // always be able to mutate at least 1 ULP, signed modifier is because `F::MAX`
    // for `F::SIGNED` does not have the sign bit
    let frozen_sig_add = temp
        .frozen_sig_add
        .clamp(0, if F::SIGNED { F::BITS - 2 } else { F::BITS - 1 });
    let mask = F::MAX >> frozen_sig_add;
    let r = F::rand(rng).unwrap() & mask;
    if rng.next_bool() {
        *f -= r;
    } else {
        *f += r;
    }
}

#[derive(Debug, Clone)]
pub struct Poly2<F: Fracint> {
    pub a0: F,
    pub a1: F,
    pub a2: F,
}

impl<F: Fracint> Poly2<F> {
    pub fn zero() -> Self {
        Self {
            a0: F::ZERO,
            a1: F::ZERO,
            a2: F::ZERO,
        }
    }

    pub fn rand(rng: &mut StarRng) -> Self {
        Self {
            a0: F::rand(rng).unwrap(),
            a1: F::rand(rng).unwrap(),
            a2: F::rand(rng).unwrap(),
        }
    }

    pub fn eval(&self, t: F) -> F {
        let mut res = self.a0;
        res = res.wrapping_add(self.a1.saturating_mul(t));
        res = res.wrapping_add(self.a2.saturating_mul(t.saturating_mul(t)));
        res
    }

    pub fn mutate(&mut self, rng: &mut StarRng, temp: &FracintTemperature) {
        match rng.index(3).unwrap() {
            0 => mutate_fracint(&mut self.a0, rng, temp),
            1 => mutate_fracint(&mut self.a1, rng, temp),
            2 => mutate_fracint(&mut self.a2, rng, temp),
            _ => unreachable!(),
        }
    }
}

// TODO for a more serious implementation we would be using unsigned fracints
// and some offset translation

///pub  Optimizes for y = sqrt(x) in a range `c..=d`
#[derive(Debug, Clone)]
pub struct Sqrt<F: Fracint> {
    pub poly2: Poly2<F>,
    pub c: F,
    pub d: F,
    pub n: F::Int,
}

impl<F: Fracint> Sqrt<F> {
    pub fn eval(&self, x: F) -> F {
        self.poly2.eval(x)
    }

    /// calculates the expected inverse x = y^2
    pub fn expected_inv(&self, y: F) -> F {
        y.saturating_mul(y)
    }

    /// Calculates an error from the expected value
    pub fn error(&self, x: F) -> u128 {
        let y = self.eval(x);
        if y < F::ZERO {
            return u128::MAX;
        }
        let x1 = self.expected_inv(y);
        let diff: F = if x < x1 {
            x1.saturating_sub(x)
        } else {
            x.saturating_sub(x1)
        };
        diff.as_int().try_into().unwrap_or(u128::MAX)
    }
}

impl<F: Fracint> Optimizeable for Sqrt<F> {
    type Temperature = FracintTemperature;

    fn cost(&self) -> u128 {
        let mut res = 0u128;
        let step = (self.d - self.c).saturating_div_int(self.n);
        let n: u128 = self.n.try_into().unwrap();
        let mut x = self.c;
        for _ in 0..n {
            res = res.saturating_add(self.error(x));
            x += step;
        }
        // for the last one, make sure we get the max value (the division for `step`
        // truncates) so that our optimizer disfavors overflow edge cases
        res = res.saturating_add(self.error(self.d));
        res
    }

    fn mutate(&mut self, rng: &mut StarRng, temp: &Self::Temperature) {
        self.poly2.mutate(rng, temp);
    }
}
