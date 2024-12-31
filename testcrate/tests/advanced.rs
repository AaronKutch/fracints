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
