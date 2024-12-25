use fracints::Fracint;
use star_rng::StarRng;

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

/*impl<F: Fracint> Optimizeable for Poly2<F> {
    type Temperature = FracintTemperature;

    fn create_rand(rng: &mut StarRng) -> Self {
    }

    fn cost(&self) -> u128 {
        todo!()
    }

    fn mutate(&mut self, rng: &mut StarRng, temp: &Self::Temperature) {
    }
}*/
