#[test]
fn general_sanity_test() {
    //It is very important to know the exact behavior of casting and shifts. The following is to have as reference every time `as` casting or shifting is used.
    //iX => signed main type
    //uX => unsigned main type
    //iD => double the bits of iX
    //uX => double the bits of uX

    //iX >> i/uX is arithmetic shift.
    //it is equivalent to dividing by a power of two and rounding towards -infinity
    assert_eq!(123i8 >> 3u8, 15);
    assert_eq!(-123i8 >> 3u8, -16);
    assert_eq!(123i8 >> 3i8, 15);
    //assert_eq!(-123i8 >> 3i8, -16); overflow!

    //uX >> i/uX logical shift, adding zeros on end
    assert_eq!(123u8 >> 3, 15);
    assert_eq!(234u8 >> 3u8, 29);
    assert_eq!(234u8 >> 3i8, 29);
    //assert_eq!(234u8 >> -3i8, 29); overflow!

    //i/uX << i/uX logical shift, adding zeros on end
    assert_eq!(13i8 << 3, 104);
    assert_eq!(-13i8 << 3, -104);
    assert_eq!(13u8 << 3, 104);
    //assert_eq!(13u8 << -3, 104); overflow!
    //interesting: 26i8(00011010) << 3i8 = -48 , -26i8(11100110) << 3i8 = 48

    //CASTING BEHAVIOR
    //iX as uX and uX as iX no change to bits
    //iX as uX numerically equal if iX >= 0
    assert_eq!(127i8 as u8, 127u8);
    assert_eq!(-120i8 as u8, 136u8);

    //uX as iX eq if uX <= iX::MAX
    assert_eq!(63u8 as i8, 63i8);
    assert_eq!(200u8 as i8, -56i8);

    //iX as iD sign extension so that -122i8(10000110) -> -122i16(11111111 10000110)
    //always eq
    assert_eq!(i16::from(-128i8), -128i16);
    assert_eq!(i16::from(-120i8), -120i16);
    assert_eq!(i16::from(127i8), 127i16);

    //iX as uD behaves just like above to the bits
    //eq if iX >= 0
    assert_eq!(127i8 as u16, 127u16);
    assert_eq!(-100i8 as u16, 65436u16);

    //uX as uD zero-extend
    //always eq
    assert_eq!(u16::from(255u8), 255u16);

    //uX as iD zero-extend
    //always eq
    assert_eq!(255u8 as i16, 255i16);

    //i/uD as i/uX plain truncation, no bit changing
    //uD as uX eq if uD <= uX::MAX
    assert_eq!(255u16 as u8, 255u8);
    assert_eq!(9001u16 as u8, 41u8);

    //iD as uX eq if 0 <= iD <= uX::MAX
    assert_eq!(100i16 as u8, 100u8);
    assert_eq!(-3i16 as u8, 253u8);
    assert_eq!(1000i16 as u8, 232u8);

    //uD as iX eq if uD <= iX::MAX
    assert_eq!(100u16 as i8, 100i8);
    assert_eq!(3u16 as i8, 3i8);
    assert_eq!(1000u16 as i8, -24i8);

    //iD as iX eq if iX::MIN <= iD <= iX::MAX
    assert_eq!(-120i16 as i8, -120i8);
    assert_eq!(127i16 as i8, 127i8);
    assert_eq!(1000i16 as i8, -24i8);
}

#[cfg(test)]
pub mod fi32_tests {
    use fracints::fracintParseError::*;
    use fracints::*;

    #[test]
    fn ni_generally_ok() {
        assert_eq!(fi32::from_str_radix("0.", 1), Err(RadixOutOfRange));
        assert_eq!(fi32::from_str_radix("0.", 37), Err(RadixOutOfRange));
        assert_eq!(fi32::from_str_radix("", 10), Err(EmptyInput));
        assert_eq!(fi32::from_str_radix("2", 10), Err(InvalidBeginningChar));
        assert_eq!(fi32::from_str_radix("-", 10), Err(SingleNeg));
        assert_eq!(fi32::from_str_radix("0", 10), Err(NoDecimalPoint));
        assert_eq!(fi32::from_str_radix("1", 10), Err(NoDecimalPoint));
        assert_eq!(fi32::from_str_radix("-1", 10), Err(NoDecimalPoint));
        assert_eq!(fi32::from_str_radix("-?.", 10), Err(InvalidCharAfterNeg));
        assert_eq!(fi32::from_str_radix("-2.", 10), Err(InvalidCharAfterNeg));
        assert_eq!(fi32::from_str_radix("-02", 10), Err(InvalidCharAfterZero));
        assert_eq!(fi32::from_str_radix("12.", 10), Err(InvalidCharAfterOne));
        assert_eq!(fi32::from_str_radix("-12", 10), Err(InvalidCharAfterOne));
        assert_eq!(fi32::from_str_radix("-1.1", 10), Err(InvalidCharInFraction));
        assert_eq!(fi32::from_str_radix("1.1", 10), Err(InvalidCharInFraction));
        assert_eq!(fi32::from_str_radix("1.?", 10), Err(InvalidCharInFraction));
        assert_eq!(
            fi32::from_str_radix("0.123456789abcd", 13),
            Err(InvalidCharInFraction)
        );
        //assert_eq!(fi32::from_str_radix("0.123456789abc", 13), Ok(fi32(245)));
        // test some basic things about the fiN! macro. If it works for fi32! it will almost surely
        // work for the rest.
        /*assert_eq!(fi32!(1.), fi32::ONE);
        //assert_eq!(fi32!(-1.), fi32::NEG_ONE);
        assert_eq!(fi32!(0.), fi32::ZERO);
        //assert_eq!(fi32!(-0.), fi32::ZERO);
        assert_eq!(fi8!(0.000000000000000000000), fi8::ZERO);
        //assert_eq!(fi8!(-0.000000000000000000000), fi8::ZERO);
        assert_eq!(fi8!(1.000000000000000000000), fi8::ONE);
        //assert_eq!(fi8!(-1.000000000000000000000), fi8::NEG_ONE);
        assert_eq!(fi32::MIN.to_string(), "-1.".to_string());
        assert_eq!(fi32::NEG_ONE.to_string(), "-1.".to_string());
        assert_eq!(fi32::ZERO.to_string(), "0.".to_string());*/
    }
}
