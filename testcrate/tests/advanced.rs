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
}
