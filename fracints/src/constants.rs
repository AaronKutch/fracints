/// The generating function is in `main.rs`.
use crate::impl_signed::*;

pub struct Const8 {
    pub num_4divtau: fi8,
    pub num_4divtau_sqr: fi8,
    pub sqrt2div2: fi8,
    pub sqrt2minus1: fi8,
    pub costaudiv16: fi8,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST8: Const8 = Const8 {
    num_4divtau: fi8(81),
    num_4divtau_sqr: fi8(52),
    sqrt2div2: fi8(91),
    sqrt2minus1: fi8(53),
    costaudiv16: fi8(118),
    cos_taylor_iters: 0,
    sin_taylor_iters: 0,
};

pub struct Const16 {
    pub num_4divtau: fi16,
    pub num_4divtau_sqr: fi16,
    pub sqrt2div2: fi16,
    pub sqrt2minus1: fi16,
    pub costaudiv16: fi16,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST16: Const16 = Const16 {
    num_4divtau: fi16(20861),
    num_4divtau_sqr: fi16(13280),
    sqrt2div2: fi16(23170),
    sqrt2minus1: fi16(13573),
    costaudiv16: fi16(30274),
    cos_taylor_iters: 2,
    sin_taylor_iters: 1,
};

pub struct Const32 {
    pub num_4divtau: fi32,
    pub num_4divtau_sqr: fi32,
    pub sqrt2div2: fi32,
    pub sqrt2minus1: fi32,
    pub costaudiv16: fi32,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST32: Const32 = Const32 {
    num_4divtau: fi32(1367130551),
    num_4divtau_sqr: fi32(870342340),
    sqrt2div2: fi32(1518500250),
    sqrt2minus1: fi32(889516852),
    costaudiv16: fi32(1984016189),
    cos_taylor_iters: 4,
    sin_taylor_iters: 3,
};

pub struct Const64 {
    pub num_4divtau: fi64,
    pub num_4divtau_sqr: fi64,
    pub sqrt2div2: fi64,
    pub sqrt2minus1: fi64,
    pub costaudiv16: fi64,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST64: Const64 = Const64 {
    num_4divtau: fi64(5871781006564002453),
    num_4divtau_sqr: fi64(3738091887791062894),
    sqrt2div2: fi64(6521908912666391106),
    sqrt2minus1: fi64(3820445788478006404),
    costaudiv16: fi64(8521284645587064995),
    cos_taylor_iters: 7,
    sin_taylor_iters: 7,
};

pub struct Const128 {
    pub num_4divtau: fi128,
    pub num_4divtau_sqr: fi128,
    pub sqrt2div2: fi128,
    pub sqrt2minus1: fi128,
    pub costaudiv16: fi128,
    pub cos_taylor_iters: usize,
    pub sin_taylor_iters: usize,
}

pub const CONST128: Const128 = Const128 {
    num_4divtau: fi128(108315241484954818046902227470560947936),
    num_4divtau_sqr: fi128(68955624378091539635910985229956156871),
    sqrt2div2: fi128(120307984584002255772516886238812528464),
    sqrt2minus1: fi128(70474785707535279813346468761740951199),
    costaudiv16: fi128(157189957036375388087984223000986562752),
    cos_taylor_iters: 13,
    sin_taylor_iters: 12,
};
