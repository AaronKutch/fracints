use fracints::Fracint;
use star_rng::StarRng;

pub struct Poly2<F: Fracint> {
    pub a0: F,
    pub a1: F,
    pub a2: F,
}

impl<F: Fracint> Poly2<F> {
    pub fn create(rng: &mut StarRng) -> Self {
        Self {
            a0: F::rand(rng).unwrap(),
            a1: F::rand(rng).unwrap(),
            a2: F::rand(rng).unwrap(),
        }
    }
}
