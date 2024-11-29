// The generating function is in the testcrate of the repo containing this
// crate.
use crate::impl_signed::*;

pub struct Const8 {
    pub num_4divtau: fi8,
    pub num_4divtau_sqr: fi8,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST8: Const8 = Const8 {
    num_4divtau: fi8(81),
    num_4divtau_sqr: fi8(52),
    cos_taylor_iters: 0,
    sin_taylor_iters: 0,
};

pub struct Const16 {
    pub num_4divtau: fi16,
    pub num_4divtau_sqr: fi16,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST16: Const16 = Const16 {
    num_4divtau: fi16(81),
    num_4divtau_sqr: fi16(52),
    cos_taylor_iters: 0,
    sin_taylor_iters: 0,
};

pub struct Const32 {
    pub num_4divtau: fi32,
    pub num_4divtau_sqr: fi32,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST32: Const32 = Const32 {
    num_4divtau: fi32(81),
    num_4divtau_sqr: fi32(52),
    cos_taylor_iters: 0,
    sin_taylor_iters: 0,
};

pub struct Const64 {
    pub num_4divtau: fi64,
    pub num_4divtau_sqr: fi64,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST64: Const64 = Const64 {
    num_4divtau: fi64(81),
    num_4divtau_sqr: fi64(52),
    cos_taylor_iters: 0,
    sin_taylor_iters: 0,
};

pub struct Const128 {
    pub num_4divtau: fi128,
    pub num_4divtau_sqr: fi128,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST128: Const128 = Const128 {
    num_4divtau: fi128(81),
    num_4divtau_sqr: fi128(52),
    cos_taylor_iters: 0,
    sin_taylor_iters: 0,
};
