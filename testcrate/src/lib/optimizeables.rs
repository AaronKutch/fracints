use fracints::Fracint;
use star_rng::StarRng;

// typically I would go for bit flipping, but this is important for switching
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
    let r = F::rand(rng) & mask;
    if rng.next_bool() {
        *f -= r;
    } else {
        *f += r;
    }
}

#[derive(Debug, Clone)]
pub struct Poly<F: Fracint> {
    pub a: Vec<F>,
    // necessary because of inaccuracies with different regions of the t^2 term
    pub offset: F,
}

impl<F: Fracint> Poly<F> {
    pub fn zero(n: usize) -> Self {
        Self {
            a: vec![F::ZERO; n],
            offset: F::ZERO,
        }
    }

    pub fn rand(n: usize, rng: &mut StarRng) -> Self {
        let mut a = vec![];
        for _ in 0..n {
            a.push(F::rand(rng));
        }
        Self {
            a,
            offset: F::rand(rng),
        }
    }

    pub fn eval(&self, mut t: F) -> F {
        // offset
        t = t.wrapping_add(self.offset);
        // use Horner evaluation
        // a0 + ((a1 + (a2 * t)) * t)
        let len = self.a.len();
        if len == 0 {
            return F::ZERO
        }
        let mut res = self.a[len - 1];
        for i in (0..(len - 1)).rev() {
            // use wrapping ops because that's what we would be using in optimized curves
            res = res.wrapping_mul(t).wrapping_add(self.a[i]);
        }
        res
    }

    pub fn mutate(&mut self, rng: &mut StarRng, temp: &FracintTemperature) {
        let i = rng.index(self.a.len() + 1).unwrap();
        if i == 0 {
            mutate_fracint(&mut self.offset, rng, temp)
        } else {
            mutate_fracint(&mut self.a[i - 1], rng, temp)
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
