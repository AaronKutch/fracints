// TODO use `thiserror`, it also results in good docs (?)

/// The error enum used to specify what parsing error happened when parsing a fracint
/// -`RadixOutOfRange` => `radix` is out of range
/// -`EmptyInput` => `src` is an empty string
/// -`InvalidBeginningChar` => beginning char of `src` is not '-','.','0',or '1'
/// -`SingleNeg` => `src` is just "-"
/// -`NoDecimalPoint` => there is no decimal point
/// -`InvalidCharAfterNeg` => char after '-' is not '.','0', or '1'
/// -`InvalidCharAfterOne` => char after the starting "1" or "-1" is not '.'
/// -`InvalidCharAfterZero` => char after the starting "0" is not '.'
/// -`InvalidCharInFraction` => there was some char in the fraction that was not valid in the
///      given `radix`, or the number was out of range (i.e. it was larger than fiN::ONE or smaller
///      than fiN::NEG_ONE)
///
/// TODO
/// ```
/// use fracints::fi64;
/// assert_eq!(fi64::from_str_radix(&"-1.0",10).unwrap(),fi64::NEG_ONE);
///
/// assert_eq!(fi16::from_str_radix(&"0.123456789",10).unwrap(),fi16(4045));
/// assert_eq!(fi16(4045).to_string(), "0.12344".to_string());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FracintParseError {
    RadixOutOfRange,
    EmptyInput,
    InvalidBeginningChar,
    SingleNeg,
    NoDecimalPoint,
    InvalidCharAfterNeg,
    InvalidCharAfterOne,
    InvalidCharAfterZero,
    InvalidCharInFraction,
}
