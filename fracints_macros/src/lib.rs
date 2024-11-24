extern crate proc_macro;
use proc_macro::TokenStream;
use fracints_internals::*;

/// Literal conversion into the type.
///
/// See `fiN::from_str_radix` for definition, and `fracintParseError` for more error
/// information.
///
/// Note that `fiN!(0)` purposely does not work, you must use at least `fiN!(0.)` in order to prevent confusing the macro with the tuple struct.
#[proc_macro]
pub fn fi8(input: TokenStream) -> TokenStream {
    // we can't get an exact char-for-char copy but this gets close enough
    let mut s = String::new();
    for token in input {
        s.push_str(&token.to_string())
    }
    for c in s.chars() {
        if c == '"' {
            panic!("The literal input to this macro should not include quotation marks");
        }
        if c.is_alphabetic() {
            panic!("Detected non-numeric-literal input, `_::from_str(variable)` should be used instead if this is intended to be a variable input");
        }
    }
    //awint::;
    /*match $ty::from_str_radix(&unaltered.to_string(), 10) {
        Ok(ok) => format!("{:?}", ok).parse().unwrap(),
        Err(e) => {
            panic!(
                "Invalid fiN string representation: {:?} in \"{}\".",
                e,
                &s.to_string()
            )
        }
    }*/
    match i8_from_str(&s) {
        Ok(x) => {
            format!("{}({})", stringify!(fi8), x).parse().unwrap()
        },
        Err(e) => panic!(
            "Invalid `{}` string representation: {}",
            stringify!(fi8),
            e,
        ),
    }
}

/*
macro_rules! impl_fracint_macros {
    ($($ty:ident),*) => {$(
    )*
    }
}

impl_fracint_macros!(fi8, fi16, fi32, fi64, fi128);
*/
