//! Usually, we want to use the numerically stable De Casteljau's algorithm when computing bezier
//! curves, but if computations are exact, then we can directly evaluate them. This example
//! generates `output.png` which shows a grey cubic bezier curve generated without exact computation
//! and a colored curve with exact computation followed by rounding to fit it on an image.

#![feature(proc_macro_hygiene)]

use fracints::*;
use image::{ImageBuffer, Rgb, RgbImage};

fn cubic_bezerp(b: &[fi8], t: fi8) -> fi8 {
    let c0 = (fi8::ONE - t) * (fi8::ONE - t) * (fi8::ONE - t);
    let c1 = (fi8::ONE - t) * (fi8::ONE - t) * t;
    let c2 = (fi8::ONE - t) * t * t;
    let c3 = t * t * t;
    (c0 * b[0]) + (c1 * b[1] * 3) + (c2 * b[2] * 3) + (c3 * b[3])
}

fn cubic_bezerp_rounded_exact(b: &[fi8], t: fi8) -> fi8 {
    let s = fi8::ONE - t;
    let c0 = s.wrapping_full_mul(s);
    let c1 = t.wrapping_full_mul(t);
    let exact = c0.wrapping_full_mul(b[0].wrapping_full_mul(s))
        + (c0.wrapping_full_mul(b[1].wrapping_full_mul(t)) * 3)
        + (c1.wrapping_full_mul(b[2].wrapping_full_mul(s)) * 3)
        + c1.wrapping_full_mul(b[3].wrapping_full_mul(t));
    fi8::from_rounded(fi16::from_truncated(exact))
}

fn main() {
    let mut bx: Vec<fi8> = Vec::new();
    bx.push(fi8!(-0.5));
    bx.push(fi8!(-1.));
    bx.push(fi8!(1.));
    bx.push(fi8!(0.));
    let mut by: Vec<fi8> = Vec::new();
    by.push(fi8!(-0.5));
    by.push(fi8!(-1.));
    by.push(fi8!(0.));
    by.push(fi8!(0.));

    let mut img: RgbImage = ImageBuffer::new(256, 256);

    // draw the bezier curve
    for t in 0..=127i8 {
        // the y coordinate is negated and 1. is added to correct the image
        let coordx = (cubic_bezerp(&bx, fi8(t)) + fi8!(1.)).0 as u8;
        let coordy = ((-cubic_bezerp(&by, fi8(t))) + fi8!(1.)).0 as u8;
        img.put_pixel(coordx as u32, coordy as u32, Rgb([128, 128, 128]));
    }

    // draw the rounded exact bezier curve
    for t in 0..=127i8 {
        let coordx = (cubic_bezerp_rounded_exact(&bx, fi8(t)) + fi8!(1.)).0 as u8;
        let coordy = ((-cubic_bezerp_rounded_exact(&by, fi8(t))) + fi8!(1.)).0 as u8;
        let nit = fi8(t);
        let r = (fi8::ONE - nit) * (fi8::ONE - nit);
        let g = ((fi8::ONE) - nit) * nit * 2;
        let b = nit * nit;
        img.put_pixel(
            coordx as u32,
            coordy as u32,
            Rgb([r.0 as u8, g.0 as u8, b.0 as u8]),
        );
    }

    img.save("perfect_bezier_curves.png").unwrap();
}
