#![feature(proc_macro_hygiene)]

use fracints::*;
use fracints_macros::*;
use image::{ImageBuffer, Rgb, RgbImage};
#[allow(unused_imports)]
use std::{i16, i8};

fn hilbert_3d(t: u32) -> (u8, u8, u8) {
    // basis vectors relative to the absolute base vector
    // the axis
    let a = vec![0, 1, 2];
    // the sign of the axis
    let s = vec![false, false, false];
    // the output
    let mut o = vec![0u8, 0, 0];
    // level of the hilbert curve
    let mut lvl: u32 = 8 - 1;
    loop {
        // position
        let p: u8 = (t.wrapping_shr(lvl * 3) & 0b111) as u8;

        // covert binary to gray code
        let d = p ^ p.wrapping_shr(1);

        // rotate
        let dx = ((d >> a[0]) & 1) ^ (s[0] as u8);
        let dy = ((d >> a[1]) & 1) ^ (s[1] as u8);
        let dz = ((d >> a[2]) & 1) ^ (s[2] as u8);

        // add to output
        o[0] += dx << lvl;
        o[1] += dy << lvl;
        o[2] += dz << lvl;

        if lvl == 0 {
            break;
        }
        lvl -= 1;
    }
    (o[0], o[1], o[2])
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

    /*for t in i8::MIN..=i8::MAX {
        let nit = fi8(t);
        let (x, y) = nit.cos_sin_taudiv2_taylor();
        let r = (t as u8).wrapping_mul(2);
        img.put_pixel(
            x.wrapping_sub(fi8::MIN).0 as u8 as u32,
            y.saturating_neg().wrapping_sub(fi8::MIN).0 as u8 as u32,
            Rgb([r, r, r]));
    }*/

    for t in i16::MIN..=i16::MAX {
        let nit = fi16(t);
        let (x, y) = nit.cos_sin_taudiv2_taylor();
        let (x, y) = (fi8::from_truncated(x), fi8::from_truncated(y));
        let (r, g, b) = hilbert_3d((t as u32) << 8);
        img.put_pixel(
            x.wrapping_sub(fi8::MIN).0 as u8 as u32,
            y.saturating_neg().wrapping_sub(fi8::MIN).0 as u8 as u32,
            Rgb([r, g, b]),
        );
    }

    for t in i16::MIN..=i16::MAX {
        let nit = fi16(t);
        let (x, y) = (nit, nit.sin_taudiv4_taylor());
        let (x, y) = (fi8::from_rounded(x), fi8::from_rounded(y));

        let (r, g, b) = hilbert_3d((t as u32) << 8);

        img.put_pixel(
            x.wrapping_sub(fi8::MIN).0 as u8 as u32,
            y.saturating_neg().wrapping_sub(fi8::MIN).0 as u8 as u32,
            Rgb([r, g, b]),
        );
    }

    img.save("cos_sin.png").unwrap();
}
