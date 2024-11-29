extern crate proc_macro;
use fracints_internals::*;
use proc_macro::TokenStream;

macro_rules! impl_fracint_macros {
($($ty:ident $from_str: ident);*;) => {$(
    /// Literal conversion into the type. See `fiN::from_str` for the definition.
    #[proc_macro]
    pub fn $ty(input: TokenStream) -> TokenStream {
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
            Ok(x) => format!("{}({})", stringify!($ty), x).parse().unwrap(),
            Err(e) => panic!("Invalid `{}` string representation: {}", stringify!($ty), e,),
        }
    }
)*}
}

impl_fracint_macros!(
    fi8 i8_from_str;
    fi16 i16_from_str;
    fi32 i32_from_str;
    fi64 i64_from_str;
    fi128 i128_from_str;
);
