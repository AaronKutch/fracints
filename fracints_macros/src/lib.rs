#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use proc_macro::TokenStream;

macro_rules! impl_normint_macros {
    ($($ty:ident),*) => {$(
        /// Literal conversion into the type. 
        ///
        /// See `fiN::from_str_radix` for definition, and `NormintParseError` for more error
        /// information.
        ///
        /// Warning: Confusing errors may be produced if what entered into the macro is not
        /// something like `0.321532` or `-1.`:
        ///     *`fi32!("0.2345")` does not work
        ///     *`fi32!(variable)` does not work (use `fi32::from_str_radix(variable, 10)` for that)
        ///     *`fi32!(0.4366236236)` works
        ///     *`fi32!(0)` does not work (to prevent confusing the macro with the tuple struct)
        ///     *`fi32!(0.)` works
        #[proc_macro]
        pub fn $ty(input: TokenStream) -> TokenStream {
            // we can't get an exact char-for-char copy but this gets close enough
            let mut s = String::new();
            for token in input {
                s.push_str(&token.to_string())
            }
            if s.contains("\"") {
                panic!("The literal should not include quotation marks");
            }
            if s.is_alpha() {
                panic!("Detected non-numeric-literal input, `fiN::from_str_radix(variable, 10)` should be used instead if this is intended to be a variable input").
            }
            awint::;
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
        }
    )*
    }
}

impl_normint_macros!(fi8, fi16, fi32, fi64, fi128);
