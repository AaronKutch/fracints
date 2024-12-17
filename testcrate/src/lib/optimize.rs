use std::fmt::Debug;

use star_rng::StarRng;

pub trait Optimizeable: Debug + Clone {
    fn create_rand(rng: &mut StarRng) -> Self;

    fn cost(&self) -> u128;
}

pub struct RampOptimize<O: Optimizeable> {
    rng: StarRng,
    pub beam: Vec<(u128, O)>,
}

impl<O: Optimizeable> Debug for RampOptimize<O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (cost, o) in &self.beam {
            write!(f, "{}\n{:?}", cost, o)?;
        }
        Ok(())
    }
}

impl<O: Optimizeable> RampOptimize<O> {
    pub fn new(rng_seed: u64, population: usize) -> Option<Self> {
        if population == 0 {
            None
        } else {
            let mut res = Self {
                rng: StarRng::new(rng_seed),
                beam: vec![],
            };
            for _ in 0..population {
                res.beam.push((u128::MAX, O::create_rand(&mut res.rng)));
            }
            Some(res)
        }
    }

    pub fn step(&mut self) {
        // we interpolate from a 0.0 chance to be replaced for the best case to a ~1.0
        // chance for the worst case
        let population = self.beam.len();
        for i in 0..population {
            let chance = u32::try_from(
                u64::try_from(i)
                    .unwrap()
                    .checked_shl(32)
                    .unwrap()
                    .checked_div(u64::try_from(population).unwrap())
                    .unwrap(),
            )
            .unwrap();
            let replace = self.rng.next_u32() < chance;
            if replace {
                // choose a random case and mutate it before replacing the one chosen to be
                // replaced
                let replacement = self.beam
                    [usize::try_from(self.rng.next_u64()).unwrap() % population]
                    .1
                    .clone();
                let cost = replacement.cost();
                self.beam[i] = (cost, replacement);
            }
        }
        self.beam.sort_by(|(cost0, _), (cost1, _)| cost0.cmp(cost1))
    }
}
