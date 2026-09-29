#![no_std]

use fracints::{Fracint, fi128};

pub fn test(x: fi128) -> fi128 {
    x.sqrt_fast()
}

pub fn rand<R: rand_core::Rng + ?Sized>(rng: &mut R) -> fi128 {
    fi128::rand(rng)
}
