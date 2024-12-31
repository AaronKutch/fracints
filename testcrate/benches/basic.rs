#![feature(test)]

extern crate test;
use fracints::prelude::*;
use star_rng::StarRng;
use test::Bencher;

#[bench]
fn f64_inv_sqrt(bencher: &mut Bencher) {
    let mut rng = StarRng::new(0);
    bencher.iter(|| {
        let x = (rng.next_u64() as f64) * 2.0f64.powi(-63);
        x.powf(-0.5)
    })
}

#[bench]
fn f64_sqrt(bencher: &mut Bencher) {
    let mut rng = StarRng::new(0);
    bencher.iter(|| {
        let x = (rng.next_u64() as f64) * 2.0f64.powi(-64);
        x.sqrt()
    })
}

#[bench]
fn fi64_sqrt(bencher: &mut Bencher) {
    let mut rng = StarRng::new(0);
    bencher.iter(|| {
        let x = fi64::from_int((rng.next_u64() as i64).wrapping_abs());
        x.sqrt_fast()
    })
}

#[bench]
fn fi64_widen_sqr(bencher: &mut Bencher) {
    let rng = &mut StarRng::new(0);
    bencher.iter(|| {
        let x = fi64::from_int(rng.next_u64() as i64);
        x.saturating_widening_mul(x)
    })
}
