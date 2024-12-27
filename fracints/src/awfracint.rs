use fracints_internals::awint::{Bits, InlAwi};
use crate::Fracint;

// TODO this can't be made to work currently

//#![cfg_attr(feature = "unstable", feature(generic_const_exprs))]
//#![cfg_attr(feature = "unstable", allow(incomplete_features))]

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Awfi<const N: usize, const LEN: usize>(InlAwi<N, LEN>);

impl<const N: usize, const LEN: usize> Awfi<N, LEN> {
    const MAX: Self = Self(InlAwi::<N, LEN>::umax());
}

/*
impl<const N: usize, const LEN: usize> Fracint for Awfi<N, LEN> {
    type Int = InlAwi::<N, LEN>;

    const BITS: usize = N;
    const MAX: Self = Self(InlAwi::<N, LEN>::MAX);
    const MIN: Self = Self(InlAwi::MIN);
    const NEG_ONE: Self = Self(-InlAwi::MAX);
}*/
