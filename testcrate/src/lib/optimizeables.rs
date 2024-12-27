use std::cmp::max;

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
pub struct Poly<F: Fracint> {
    pub a: Vec<F>,
}

impl<F: Fracint> Poly<F> {
    pub fn zero(n: usize) -> Self {
        Self {
            a: vec![F::ZERO; n],
        }
    }

    pub fn rand(n: usize, rng: &mut StarRng) -> Self {
        let mut a = vec![];
        for _ in 0..n {
            a.push(F::rand(rng).unwrap());
        }
        Self { a }
    }

    pub fn eval(&self, t: F) -> F {
        let mut add = F::ZERO;
        let mut mul = t;
        for i in 0..self.a.len() {
            if i == 0 {
                // avoid F::ONE multiplication error
                add = add.wrapping_add(self.a[i]);
            } else {
                add = add.wrapping_add(self.a[i].saturating_mul(mul));
                mul = mul.saturating_mul(t);
            }
        }
        add
    }

    pub fn mutate(&mut self, rng: &mut StarRng, temp: &FracintTemperature) {
        if let Some(x) = rng.index_slice_mut(&mut self.a) {
            mutate_fracint(x, rng, temp)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rational<F: Fracint> {
    pub num: Poly<F>,
    pub den: Poly<F>,
}

impl<F: Fracint> Rational<F> {
    pub fn zero(n_num: usize, n_den: usize) -> Self {
        Self {
            num: Poly::zero(n_num),
            den: Poly::zero(n_den),
        }
    }

    pub fn eval(&self, t: F) -> F {
        if self.den.a.is_empty() {
            self.num.eval(t)
        } else {
            self.num.eval(t).saturating_div(self.den.eval(t))
        }
    }

    pub fn mutate(&mut self, rng: &mut StarRng, temp: &FracintTemperature) {
        match rng.index(2).unwrap() {
            0 => self.num.mutate(rng, temp),
            1 => self.den.mutate(rng, temp),
            _ => unreachable!(),
        }
    }
}

// TODO for a more serious implementation we would be using unsigned fracints
// and some offset translation

/// Optimizes for y = sqrt(x) in a range `start..=end`
#[derive(Debug, Clone)]
pub struct Sqrt<F: Fracint> {
    pub rational: Rational<F>,
    pub start: F,
    pub end: F,
    pub n: F::Int,
}

impl<F: Fracint> Sqrt<F> {
    pub fn eval(&self, x: F) -> F {
        self.rational.eval(x)
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
        // TODO we should be able to move the more accurate `sqrt_slow` to `Fracint`
        let expected_y = x.sqrt_simple_bisection();
        expected_y.saturating_sub(y).saturating_abs().as_int().try_into().unwrap_or(u128::MAX)
        //let expected_x = self.expected_inv(y);
        //expected_x.saturating_sub(x).saturating_abs().as_int().try_into().unwrap_or(u128::MAX)
    }
}

impl<F: Fracint> Optimizeable for Sqrt<F> {
    type Temperature = FracintTemperature;

    fn cost(&self) -> u128 {
        let mut res = 0u128;
        let step = (self.end - self.start).saturating_div_int(self.n);
        let n: u128 = self.n.try_into().unwrap();
        let mut x = self.start;
        for _ in 0..n {
            res = max(res, self.error(x));
            x += step;
        }
        // for the last one, make sure we get the max value (the division for `step`
        // truncates) so that our optimizer disfavors overflow edge cases
        res = max(res, self.error(self.end));
        res
    }

    fn mutate(&mut self, rng: &mut StarRng, temp: &Self::Temperature) {
        self.rational.mutate(rng, temp);
    }
}
