use common::sqrt::isqrt_sub1;
use fracints::prelude::*;

#[test]
fn sqrt() {
    assert_eq!(fi64!(0.0).sqrt_slow(), fi64!(0.0));
    assert_eq!(fi64::ULP.sqrt_slow(), fi64!(0.0000000003292722539));
    assert_eq!(
        fi64!(0.000000000001).sqrt_slow(),
        fi64!(0.0000009999999980021)
    );
    assert_eq!(fi64!(0.5).sqrt_slow(), fi64!(0.7071067811865475244));
    assert_eq!(fi64!(1.0).sqrt_slow(), fi64!(1.0));

    assert_eq!(isqrt_sub1(fi16!(0.25)), fi16!(1.0));
    assert_eq!(isqrt_sub1(fi16!(0.5)), fi16!(0.41418));
    assert_eq!(isqrt_sub1(fi16!(0.75)), fi16!(0.15469));
    assert_eq!(isqrt_sub1(fi16!(1.0)), fi16!(0.0));

    assert_eq!(
        fi128!(0.07).sqrt_fast(),
        fi128!(0.264575131106459059050161575363926042568)
    );
    assert_eq!(fi64!(0.07).sqrt_fast(), fi64!(0.2645751311064590591));
    assert_eq!(fi32!(0.07).sqrt_fast(), fi32!(0.2645751303));
    assert_eq!(fi16!(0.07).sqrt_fast(), fi16!(0.26459));
    assert_eq!(fi8!(0.07).sqrt_fast(), fi8!(0.258));

    // TODO we need a full `awint` style fuzz test for square roots
    assert_eq!(
        fi128::ULP.sqrt_fast(),
        fi128!(0.00000000000000000007666467083416870407)
    );
    assert_eq!(
        fi128!(0.11).sqrt_fast(),
        fi128!(0.331662479035539984911493273667068668386)
    );
    assert_eq!(fi64!(0.0).sqrt_fast(), fi64!(0.0));
    assert_eq!(fi64!(1.0).sqrt_fast(), fi64!(1.0));
}

#[test]
fn float_conv() {
    assert!(fi16::from_f64(1.0000001).is_none());
    assert!(fi16::from_f64(-1.0000001).is_none());
    assert_eq!(fi16::from_f64(1.0).unwrap(), fi16!(1.0));
    assert_eq!(fi16::from_f64(-1.0).unwrap(), fi16!(-1.0));
    assert_eq!(fi16::from_f64(0.999999999999999999).unwrap(), fi16!(1.0));
    assert_eq!(fi16::from_f64(-0.999999999999999999).unwrap(), fi16!(-1.0));
    assert_eq!(fi16::from_f64(-0.999999999).unwrap(), fi16!(-1.0));

    assert!(fi16::from_f32(1.0000001).is_none());
    assert!(fi16::from_f32(-1.0000001).is_none());
    assert_eq!(fi16::from_f32(1.0).unwrap(), fi16!(1.0));
    assert_eq!(fi16::from_f32(-1.0).unwrap(), fi16!(-1.0));
    assert_eq!(fi16::from_f32(0.999999999999999999).unwrap(), fi16!(1.0));
    assert_eq!(fi16::from_f32(-0.999999999999999999).unwrap(), fi16!(-1.0));
    assert_eq!(fi16::from_f32(-0.999999999).unwrap(), fi16!(-1.0));

    assert_eq!(fi128!(0.123).to_f32(), 0.122999996);
    assert_eq!(fi128!(0.123).to_f64(), 0.123);
}
