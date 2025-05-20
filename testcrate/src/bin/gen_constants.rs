//! This is a constant generator for `constants.rs`.
//! This is ugly, but there are many things practically requiring allocation or
//! otherwise can't be done in a `const fn`, and some constant bootstraps take a
//! significant amount of time to calculate and would add to compilation time

use std::fmt::Write;

use common::sqrt::simple_isqrt_lut;
use fracints::{Fracint, fi16};

pub fn main() {
    let mut s = r#"// The generating function is in the testcrate of the repo containing this
// crate.
use crate::impl_signed::*;

"#
    .to_owned();
    for w in [8usize, 16, 32, 64, 128] {
        let fi = format!("fi{w}");
        // FIXME
        let num_4divtau = 81;
        let num_4divtau_sqr = 52;
        let cos_taylor_iters = 0;
        let sin_taylor_iters = 0;
        writeln!(
            s,
            r#"pub struct Const{w} {{
    pub num_4divtau: {fi},
    pub num_4divtau_sqr: {fi},
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}}

pub const CONST{w}: Const{w} = Const{w} {{
    num_4divtau: {fi}({num_4divtau}),
    num_4divtau_sqr: {fi}({num_4divtau_sqr}),
    cos_taylor_iters: {cos_taylor_iters},
    sin_taylor_iters: {sin_taylor_iters},
}};
"#
        )
        .unwrap()
    }

    // isqrt table
    let n = 24;
    // find by setting to 1.0 and using the x value
    let cutoff = fi16!(0.99936);
    let (lut, bits) = simple_isqrt_lut(n, cutoff);
    writeln!(s, r#"pub const SIMPLE_ISQRT_LUT: [fi16; {n}] = ["#).unwrap();
    for entry in lut {
        writeln!(s, r#"    fi16({}),"#, entry.as_int()).unwrap();
    }
    writeln!(s, r#"];"#).unwrap();

    writeln!(
        s,
        r#"pub const SIMPLE_ISQRT_CUTOFF: fi16 = fi16({});"#,
        cutoff.as_int(),
    )
    .unwrap();
    writeln!(s, r#"pub const SIMPLE_ISQRT_BITS: usize = {bits};"#).unwrap();

    println!("\n\n\nBEGIN:\n{s}");
}
