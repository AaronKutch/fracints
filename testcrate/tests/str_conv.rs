use std::str::FromStr;

use fracints::FracintSerdeError::*;
use fracints::*;

#[test]
fn test_fi8() {
    assert_eq!(fi8::from_str(""), Err(Empty));
    assert_eq!(fi8::from_str("0"), Ok(fi8(0)));
    assert_eq!(fi8::from_str("-0"), Ok(fi8(0)));
    assert_eq!(fi8::from_str("0.0"), Ok(fi8(0)));

}
