#![feature(proc_macro_hygiene)]

use normints::*;
use normints_macros::*;
use apint::*;
use image::{Rgb, RgbImage, ImageBuffer};
use std::u16;
use std::iter;

/*
fn hilbert_2d(t: u16) -> (u8, u8) {
    // basis vectors relative to the absolute base vector
    // the axis
    let mut a = vec![0, 1];
    // the sign of the axis
    let mut s = vec![false, false];
    // the output
    let mut o = vec![0u8, 0];
    // level of the hilbert curve
    let mut lvl: u32 = 8 - 1;
    loop {
        // position
        let p: u8 = (t.wrapping_shr(lvl * 2) & 0b11) as u8;

        // covert binary to gray code, which generates the unit hilbert curve
        // In this case, `d` corresponds to this
        //       y
        //  ___  ^      3_2
        //    |  |        |
        // >__|  --> x  0_1
        let mut d = p ^ p.wrapping_shr(1);

        // rotate
        let dx = ((d >> a[0]) & 1) ^ (s[0] as u8);
        let dy = ((d >> a[1]) & 1) ^ (s[1] as u8);

        // add to output
        o[0] += dx << lvl;
        o[1] += dy << lvl;

        if lvl == 0 {break}
        lvl -= 1;

        // find and swap two axis
        let mut fs = |axis0: usize, axis1: usize| {
            for i0 in 0..2 {
                if a[i0] == axis0 {
                    for i1 in 0..2 {
                        if a[i1] == axis1 {
                            let tmp = a[i0];
                            a[i0] = a[i1];
                            a[i1] = tmp;
                            let tmp = s[i0];
                            s[i0] = s[i1];
                            s[i1] = tmp;
                            return;
                        }
                    }
                }
            }
        };

        // negate sign of two axis
        macro_rules! n {
            ($a:ident, $s:ident, $axis0:expr, $axis1:expr) => {
                for i in 0..2 {
                    if $a[i] == $axis0 {
                        $s[i] = !$s[i];
                    }
                }
                for i in 0..2 {
                    if $a[i] == $axis1 {
                        $s[i] = !$s[i];
                    }
                }
            };
        }

        // Recurse. It is important to orientate the curve by putting an arrow in the direction
        // of travel of the curve.
        // | v______
        // |3|    2|
        // |_|   __|
        //       ^
        //       |
        //       |
        // ___   |__
        // |0|    1|
        // | |__>__|
        // ^
        // The following transformations work for any starting rotation of the curve.
        match p {
            0 => {
                // swap the x and y axis
                fs(0, 1);
            }
            1 => (),
            2 => (),
            3 => {
                // swap the x and y axis
                fs(0, 1);
                n!(a, s, 0, 1);
            }
            _ => unreachable!()
        }
    }
    (o[0], o[1])
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(512, 512);
    /*for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }*/

    // draw the bezier curve with points between the actual positions scaled up by 2, to show the
    // curve clearly
    let (mut px, mut py) = (0, 1);
    for t in 0..(1 << 16) {
        let (x, y) = hilbert_2d(t as u16);
        let (x, y) = ((x as u32) * 2, (y as u32) * 2);
        // check that only one coordinate is being changed at a time
        assert!((((x == px) as usize) + ((y == py) as usize)) == 1);

        let color = (t / 256) as u8;
        img.put_pixel(
            (x + px) / 2,
            img.height() - 1 - ((y + py) / 2),
            Rgb([color, color, color]));
        img.put_pixel(
            x,
            img.height() - 1 - y,
            Rgb([color, color, color]));

        px = x;
        py = y;
    }

    img.save("mandlebrot_hilbert.png").unwrap();
}
*/

/// `n` is the dimension of the hilbert curve, `lvl` is the level of Hilbert curve, and `t` is the index along that curve
/*fn hilbert_n_lvl_t(n: usize, lvl: usize, t: ApInt) -> Vec<ApInt> {
    if n == 0 {
        return Vec::new();
    }
    let output_bw = BitWidth::new(1 << lvl).unwrap();
    let n_bw = BitWidth::new(n).unwrap();
    let mut a = (0..n).collect::<Vec<usize>>();
    let mut s = ApInt::zero(n_bw);
    let mut o = vec![ApInt::from(0usize).into_zero_resize(output_bw); n];// iter::repeat(ApInt::from(0).into_zero_resize(bw)).take(n).collect::<Vec<ApInt>>();
    let mut lvl = lvl;
    loop {
        let mut p = t.clone().into_wrapping_lshr(lvl * n).unwrap().into_zero_resize(n_bw);
        let mut d = p.checked_xor(p.clone().into_wrapping_lshr(1).unwrap()).unwrap();
        d.
        // reflect
        d.bitxor_assign(s);

        // add to the output
        for i in 0..n {
            if d.get_bit_at(a[i]).unwrap() {
                o[i].wrapping_add_assign(ApInt::one(output_bw).into_wrapping_shl(i).unwrap()).unwrap();
            }
        }

        if lvl == 0 {break}
        lvl -= 1;

        //
    }
    o
}*/


fn hilbert_3d(t: u32) -> (u8, u8, u8) {
    // basis vectors relative to the absolute base vector
    // the axis
    let mut a = vec![0, 1, 2];
    // the sign of the axis
    let mut s = vec![false, false, false];
    // the output
    let mut o = vec![0u8, 0, 0];
    // level of the hilbert curve
    let mut lvl: u32 = 8 - 1;
    loop {
        // position
        let p: u8 = (t.wrapping_shr(lvl * 3) & 0b111) as u8;

        // covert binary to gray code
        let mut d = p ^ p.wrapping_shr(1);

        // rotate
        let dx = ((d >> a[0]) & 1) ^ (s[0] as u8);
        let dy = ((d >> a[1]) & 1) ^ (s[1] as u8);
        let dz = ((d >> a[2]) & 1) ^ (s[2] as u8);
if t == 200 {dbg!((t, &a, &s, &o, dx, dy, dz));}
        // add to output
        o[0] += dx << lvl;
        o[1] += dy << lvl;
        o[2] += dz << lvl;

        if lvl == 0 {break}
        lvl -= 1;

        // find and swap two axis
        let mut fs = |a: &mut Vec<usize>, s: &mut Vec<bool>, axis0: usize, axis1: usize| {
            for i0 in 0..3 {
                if a[i0] == axis0 {
                    for i1 in 0..3 {
                        if a[i1] == axis1 {
                            let tmp = a[i0];
                            a[i0] = a[i1];
                            a[i1] = tmp;
                            let tmp = s[i0];
                            s[i0] = s[i1];
                            s[i1] = tmp;
                            return
                        }
                    }
                }
            }
        };

        // negate sign of two axis
        let mut n = |a: &mut Vec<usize>, s: &mut Vec<bool>, axis0: usize, axis1: usize| {
            for i in 0..3 {
                if a[i] == axis0 {
                    s[i] = !s[i];
                }
            }
            for i in 0..3 {
                if a[i] == axis1 {
                    s[i] = !s[i];
                }
            }
        };

        // Recurse.
        match p {
            0 => {
                fs(&mut a, &mut s, 0, 2);
            }
            1 => {
                fs(&mut a, &mut s, 1, 2);
            }
            2 => (),
            3 => {
                fs(&mut a, &mut s, 0, 2);
                n(&mut a, &mut s, 0, 2);
            }
            4 => {
                fs(&mut a, &mut s, 0, 2);
            }
            5 => (),
            6 => {
                fs(&mut a, &mut s, 1, 2);
                n(&mut a, &mut s, 1, 2);
            }
            7 => {
                fs(&mut a, &mut s, 0, 2);
                n(&mut a, &mut s,0,2);
            }
            _ => unreachable!()
        }
    }
    (o[0], o[1], o[2])
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(0x100, 16);
    /*for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }*/
    let (mut px, mut py, mut pz) = (0, 0, 1);
    for t in 0..0x100 {
        let (x, y, z) = hilbert_3d((t) as u32);
        //dbg!(t,x,y,z);
        if x != px {
            assert!(x.wrapping_sub(px) == 1 || px.wrapping_sub(x) == 1);
        }
        if y != py {
            assert!(y.wrapping_sub(py) == 1 || py.wrapping_sub(y) == 1);
        }
        if z != pz {
            assert!(z.wrapping_sub(pz) == 1 || pz.wrapping_sub(z) == 1);
        }
        assert!((((x == px) as usize) + ((y == py) as usize) + ((z == pz) as usize)) == 2);

        /*for i in 0..16 {
            img.put_pixel(
                t,
                i,
                Rgb([x, y, z]));
        }*/

        px = x;
        py = y;
        pz = z;
    }

    img.save("mandlebrot_hilbert.png").unwrap();
}