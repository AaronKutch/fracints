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

pub const SIMPLE_ISQRT_LUT: [fi16; 24] = [
    fi16(32684),
    fi16(28936),
    fi16(25766),
    fi16(23038),
    fi16(20658),
    fi16(18559),
    fi16(16689),
    fi16(15009),
    fi16(13489),
    fi16(12106),
    fi16(10839),
    fi16(9674),
    fi16(8597),
    fi16(7598),
    fi16(6668),
    fi16(5800),
    fi16(4986),
    fi16(4221),
    fi16(3501),
    fi16(2822),
    fi16(2179),
    fi16(1570),
    fi16(991),
    fi16(441),
];
pub const SIMPLE_ISQRT_CUTOFF: fi16 = fi16(32747);
pub const SIMPLE_ISQRT_BITS: usize = 5;
