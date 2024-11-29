extern crate proc_macro;
use fracints_internals::*;
use proc_macro::TokenStream;

/// Literal conversion into the type. See `fiN::from_str` for the definition.
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
    }
    match i8_from_str(&s) {
        Ok(x) => format!("{}({})", stringify!(fi8), x).parse().unwrap(),
        Err(e) => panic!("Invalid `{}` string representation: {}", stringify!(fi8), e,),
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
