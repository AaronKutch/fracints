/*
#[cfg(test)]
#[test]
fn test() {
    assert_eq!($ty($iX::MIN / 2).to_string(), "-0.5".to_string());
    assert_eq!($ty($iX::MAX / 2 + 1).to_string(), "0.5".to_string());
    assert_eq!($ty::from_str_radix("-0.5", 10).unwrap(), $ty($iX::MIN / 2));
    assert_eq!(
        $ty::from_str_radix("0.5", 10).unwrap(),
        $ty($iX::MAX / 2 + 1)
    );
    assert_eq!($ty::from_str_radix("-0.5", 10).unwrap(), $ty($iX::MIN / 2));
    // should work in all availiable bases
    // TODO there is early returns in the functions for these meaning I need to write some
    // more
    for r in 2..37 {
        assert_eq!($ty::MIN.to_string_radix(r), "-1.".to_string());
        assert_eq!($ty::NEG_ONE.to_string_radix(r), "-1.".to_string());
        assert_eq!($ty::ZERO.to_string_radix(r), "0.".to_string());
        assert_eq!($ty::ONE.to_string_radix(r), "1.".to_string());
        assert_eq!($ty::MAX.to_string_radix(r), "1.".to_string());
        assert_eq!($ty::from_str_radix("-1.", r).unwrap(), $ty::NEG_ONE);
        assert_eq!($ty::from_str_radix("0.", r).unwrap(), $ty::ZERO);
        assert_eq!($ty::from_str_radix("1.", r).unwrap(), $ty::ONE);
    }
    // some of this is mainly to check that `apint` is working as expected
    assert_eq!($ty($iX::MAX / 4 + 1) + $ty($iX::MAX / 4), $ty($iX::MAX / 2));
    assert_eq!($ty($iX::MIN / 4) + $ty($iX::MIN / 4), $ty($iX::MIN / 2));
    assert_eq!($ty($iX::MAX / 4 + 1) + $ty($iX::MIN / 4), $ty(0));
    assert_eq!($ty($iX::MIN / 4) + $ty($iX::MAX / 4 + 1), $ty(0));
    assert_eq!($ty($iX::MAX / 4) - $ty($iX::MAX / 4), $ty(0));
    assert_eq!($ty($iX::MIN / 4) - $ty($iX::MIN / 4), $ty(0));
    assert_eq!(
        $ty($iX::MAX / 4 + 1) - $ty($iX::MIN / 4),
        $ty($iX::MAX / 2 + 1)
    );
    assert_eq!($ty($iX::MIN / 4) - $ty($iX::MAX / 4 + 1), $ty($iX::MIN / 2));

    assert_eq!($ty($iX::MAX).wrapping_add($ty($iX::MAX)), $ty(-2));
    assert_eq!($ty($iX::MAX).wrapping_add($ty($iX::MIN)), $ty(-1));
    assert_eq!($ty($iX::MIN).wrapping_add($ty($iX::MAX)), $ty(-1));
    assert_eq!($ty($iX::MIN).wrapping_add($ty($iX::MIN)), $ty(0));
    assert_eq!($ty($iX::MAX).wrapping_sub($ty($iX::MAX)), $ty(0));
    assert_eq!($ty($iX::MAX).wrapping_sub($ty($iX::MIN)), $ty(-1));
    assert_eq!($ty($iX::MIN).wrapping_sub($ty($iX::MAX)), $ty(1));
    assert_eq!($ty($iX::MIN).wrapping_sub($ty($iX::MIN)), $ty(0));

    assert_eq!($ty($iX::MIN / 2) * $ty($iX::MIN / 2), $ty($iX::MAX / 4 + 1));
    assert_eq!($ty($iX::MIN / 2) * $ty($iX::MAX / 2 + 1), $ty($iX::MIN / 4));
    assert_eq!($ty($iX::MAX / 2 + 1) * $ty($iX::MIN / 2), $ty($iX::MIN / 4));
    assert_eq!(
        $ty($iX::MAX / 2 + 1) * $ty($iX::MAX / 2 + 1),
        $ty($iX::MAX / 4 + 1)
    );

    assert_eq!($ty($iX::MIN / 2).wrapping_div($ty($iX::MIN / 2)), $ty::MIN);
    assert_eq!(
        $ty($iX::MIN / 2).wrapping_div($ty($iX::MAX / 2 + 1)),
        $ty::MIN
    );
    assert_eq!(
        $ty($iX::MAX / 2 + 1).wrapping_div($ty($iX::MAX / 2 + 1)),
        $ty::MIN
    );
    assert_eq!(
        $ty($iX::MAX / 4 + 1).wrapping_div($ty($iX::MAX / 2 + 1)),
        $ty($iX::MAX / 2 + 1)
    );
    assert_eq!(
        $ty($iX::MIN / 2).saturating_div($ty($iX::MIN / 2)),
        $ty::ONE
    );
    assert_eq!(
        $ty($iX::MIN / 2).saturating_div($ty($iX::MAX / 2 + 1)),
        $ty::NEG_ONE
    );
    assert_eq!(
        $ty($iX::MAX / 2 + 1).saturating_div($ty($iX::MAX / 2 + 1)),
        $ty::ONE
    );
    assert_eq!(
        $ty($iX::MAX / 4 + 1).saturating_div($ty($iX::MAX / 2 + 1)),
        $ty($iX::MAX / 2 + 1)
    );

    assert_eq!($ty($iX::MIN).wrapping_mul($ty($iX::MIN)), $ty::MIN);
    assert_eq!($ty($iX::MIN).wrapping_mul($ty($iX::MAX)), $ty::NEG_ONE);
    assert_eq!($ty($iX::MAX).wrapping_mul($ty($iX::MIN)), $ty::NEG_ONE);
    assert_eq!($ty($iX::MAX).wrapping_mul($ty($iX::MAX)), $ty($iX::MAX - 1));
    assert_eq!($ty($iX::MIN).wrapping_div($ty($iX::MIN)), $ty::MIN);
    assert_eq!($ty($iX::MIN).wrapping_div($ty($iX::MAX)), $ty::MAX);
    assert_eq!($ty($iX::MAX).wrapping_div($ty($iX::MAX)), $ty::MIN);
    assert_eq!($ty($iX::MAX).wrapping_div($ty($iX::MAX)), $ty::MIN);
    assert_eq!($ty::MIN.saturating_div($ty::MIN), $ty::ONE);
    assert_eq!($ty::MIN.saturating_div($ty::NEG_ONE), $ty::ONE);
    assert_eq!($ty::NEG_ONE.saturating_div($ty::MIN), $ty::ONE);
    assert_eq!($ty::NEG_ONE.saturating_div($ty::NEG_ONE), $ty::ONE);
    assert_eq!($ty::MIN.saturating_div($ty::MAX), $ty::NEG_ONE);
    assert_eq!($ty::NEG_ONE.saturating_div($ty::MAX), $ty::NEG_ONE);
    assert_eq!($ty::MAX.saturating_div($ty::MIN), $ty::NEG_ONE);
    assert_eq!($ty::MAX.saturating_div($ty::NEG_ONE), $ty::NEG_ONE);
    assert_eq!($ty::MAX.saturating_div($ty::MAX), $ty::ONE);
}
*/
