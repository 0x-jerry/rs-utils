use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Ident, parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma};

pub struct ChainFromInputData {
    pub value: Expr,
    pub types: Punctuated<Ident, Comma>,
}

impl Parse for ChainFromInputData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value: Expr = input.parse()?;
        let _: Comma = input.parse()?;

        let types = input.parse_terminated(Ident::parse, Comma)?;

        return Ok(Self { value, types });
    }
}

///
/// Example:
///
/// ```no_run
/// chain_from!(value, Type1, Type2, ..., TypeN);
/// ```
///
/// Each type must implement `From<T>` where `T` is the previous type.
// #[proc_macro]
pub fn chain_from(input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as ChainFromInputData);

    let value = data.value;
    let types = data.types.iter();

    let expanded = quote! {
        {
            let value = #value;

            #(
                let value = #types::from(value);
            )*

            value
        }
    };

    return expanded.into();
}
