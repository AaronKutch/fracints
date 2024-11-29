use std::str::FromStr;

use fracints::{FracintSerdeError::*, *};

#[test]
fn from_str() {
    macro_rules! a {
        ($lhs:expr, $rhs:expr) => {
            assert_eq!(fi8::from_str($lhs), $rhs);
        };
    }
    a!("", Err(Empty));
    a!("_", Err(Empty));
    a!("-", Err(EmptyInteger));
    a!("x", Err(InvalidCharInInteger));
    a!("0a", Err(InvalidCharInInteger));
    a!("0b", Err(EmptyInteger));
    a!("0o", Err(EmptyInteger));
    a!("0x", Err(EmptyInteger));
    a!(".", Err(EmptyInteger));
    a!("e", Err(EmptyInteger));
    a!("p", Err(EmptyInteger));
    a!("0.", Err(EmptyFraction));
    a!("0._", Err(EmptyFraction));
    a!("0.a", Err(InvalidCharInFraction));
    a!("0e", Err(EmptyExponent));
    a!("0.0e", Err(EmptyExponent));
    a!("0.0e-", Err(EmptyExponent));
    a!("0.0e-_", Err(EmptyExponent));
    a!("0.0ea", Err(InvalidCharInExponent));
    a!("0b2", Err(InvalidCharInInteger));
    a!("0o9", Err(InvalidCharInInteger));
    a!("0xg", Err(InvalidCharInInteger));

    a!("0", Ok(fi8(0)));
    a!("-0", Ok(fi8(0)));
    a!("-_0_00_._00_0_e-_00_0_", Ok(fi8(0)));
    a!("1", Ok(fi8::ONE));
    a!("-1", Ok(fi8::NEG_ONE));

    a!("0.5", Ok(fi8(64)));
    a!("0.00390625", Ok(fi8(0)));
    a!("0.00390626", Ok(fi8(1)));
    a!("0.01171874", Ok(fi8(1)));
    a!("0.01171875", Ok(fi8(2)));
    a!("0b1e-111", Ok(fi8(1)));
    a!("0b10e-1000", Ok(fi8(1)));
    a!("-0.001p3", Ok(fi8::NEG_ONE));
    a!("-0.999", Ok(fi8::NEG_ONE));

    assert_eq!(
        fi128::from_str("0.636619772367581343075535053490057448137_8"),
        Ok(fi128(108315241484954818046902227470560947936))
    );
}

#[test]
fn to_str() {
    macro_rules! a {
        ($lhs:expr, $rhs:expr) => {
            assert_eq!($lhs.to_string(), $rhs);
        };
    }
    a!(fi8::ZERO, "0.0");
    a!(fi8::ONE, "1.0");
    a!(fi8::NEG_ONE, "-1.0");
    a!(fi8::MIN, "-1.0");
    a!(fi8(1), "0.008");
    a!(fi8(2), "0.016");

    a!(fi16(1), "0.00003");
    a!(fi32(1), "0.0000000005");
    a!(fi64(1), "0.0000000000000000001");
    a!(fi128(1), "0.000000000000000000000000000000000000006");

    a!(
        fi128(108315241484954818046902227470560947936),
        "0.636619772367581343075535053490057448135"
    );
}
