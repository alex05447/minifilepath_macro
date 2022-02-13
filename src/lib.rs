//! Exports the `filepath!` macro to create compile-time checked file [`FilePath`](minifilepath::FilePath)'s.

use {
    minifilepath::FilePath,
    proc_macro::TokenStream,
    proc_macro2::{Literal as Literal2, TokenStream as TokenStream2, TokenTree as TokenTree2},
    quote::quote,
};

/// Creates a [`FilePath`](minifilepath::FilePath) from a compile-time checked non-empty quoted file path string literal.
#[proc_macro]
pub fn filepath(item: TokenStream) -> TokenStream {
    filepath_impl(item.into())
}

pub(crate) fn filepath_impl(item: TokenStream2) -> TokenStream {
    let macro_name = "filepath";

    let mut iter = item.into_iter();

    let string_tt = iter.next().expect(&format!(
        "`{}` macro takes one non-empty quoted file path string literal - none were provided",
        macro_name
    ));

    let result = match string_tt {
        TokenTree2::Literal(string_lit) => {
            // At least ["a"].
            let orig_string = string_lit.to_string();
            assert!(
                orig_string.len() >= 3,
                "`{}` macro takes one non-empty quoted file path string literal - `{}` was provided",
                macro_name,
                orig_string
            );

            // Trim quotes: ["asdf"] -> [asdf].
            if let Some(no_prefix_string) = orig_string.strip_prefix("\"") {
                if let Some(_) = no_prefix_string.strip_suffix("\"") {
                    match FilePath::new(&orig_string) {
                        Ok(_) => {
                            let string_lit: Literal2 = string_lit.into();

                            TokenStream::from(quote!(
                                unsafe { FilePath::new_unchecked(#string_lit) }
                            ))
                        }
                        Err(err) => {
                            panic!("`{}` macro takes one non-empty quoted file path string literal - `{}` is not a valid file path ({})", macro_name, orig_string, err);
                        }
                    }
                } else {
                    panic!("`{}` macro takes one non-empty quoted file path string literal - `{}` does not end with a quote", macro_name, orig_string);
                }
            } else {
                panic!("`{}` macro takes one non-empty quoted file path string literal - `{}` does not start with a quote", macro_name, orig_string);
            }
        }

        TokenTree2::Group(group) => filepath_impl(group.stream()),

        TokenTree2::Ident(ident) => {
            panic!(
                "`{}` macro takes one non-empty quoted file path string literal - ident `{}` was provided",
                macro_name, ident
            );
        }

        TokenTree2::Punct(punct) => {
            panic!(
                "`{}` macro takes one non-empty quoted file path string literal - punct `{}` was provided",
                macro_name, punct
            );
        }
    };

    assert!(
        iter.next().is_none(),
        "`{}` macro takes one non-empty quoted file path string literal - multiple were provided",
        macro_name
    );

    result
}
