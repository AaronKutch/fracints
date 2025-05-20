use core::fmt;
use std::fmt::Debug;

use fracints::{Fracint, fi64};
use star_rng::StarRng;

pub trait Optimizeable: Debug + Clone {
    type Temperature;
    type Cost: Copy + Ord + fmt::Display;

    fn cost(&self) -> Self::Cost;

    fn mutate(&mut self, rng: &mut StarRng, temp: &Self::Temperature);
}

/// For quickly determining through brute force stochastic methods if there are
/// good approximations. We use this in this crate even when closed form methods
/// are available, because we want to optimize exactly with the real bit errors.
pub struct RampOptimize<O: Optimizeable> {
    rng: StarRng,
    pub beam: Vec<(O::Cost, O)>,
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
    pub fn new(init: O, rng_seed: u64, population: usize) -> Self {
        if population == 0 {
            panic!()
        }
        let mut res = Self {
            rng: StarRng::new(rng_seed),
            beam: vec![],
        };
        for _ in 0..population {
            res.beam.push((init.cost(), init.clone()));
        }
        res
    }

    pub fn step(&mut self, temp: &O::Temperature) {
        // we interpolate from a 0.0 chance to be replaced for the best case to a ~1.0
        // chance for the worst case
        let population: i64 = self.beam.len().try_into().unwrap();
        let inc = fi64!(1.0).saturating_div_int(population);
        let mut chance = fi64::ZERO;
        for i in 0..population {
            //let chance = inc.saturating_mul_int(i);
            chance += inc;
            let replace = fi64::rand(&mut self.rng).wrapping_abs() < chance;
            if replace {
                // choose a random case and mutate it before replacing the one chosen to be
                // replaced
                let mut replacement = self.rng.index_slice(&self.beam).unwrap().1.clone();
                replacement.mutate(&mut self.rng, temp);
                let cost = replacement.cost();
                self.beam[i as usize] = (cost, replacement);
            }
        }
        self.beam.sort_by(|(cost0, _), (cost1, _)| cost0.cmp(cost1))
    }

    pub fn best(&self) -> O {
        self.beam[0].1.clone()
    }
}
