use proc_macro::TokenStream;

mod chain_from;
mod migration;

///
/// Example:
///
/// ```no_run
/// chain_from!(value, Type1, Type2, ..., TypeN);
/// ```
///
/// Each type must implement `From<T>` where `T` is the previous type.
#[proc_macro]
pub fn chain_from(input: TokenStream) -> TokenStream {
    chain_from::chain_from(input)
}

#[proc_macro_derive(Versioned)]
pub fn migration_versioned_derive(input: TokenStream) -> TokenStream {
    migration::migration_versioned_derive(input)
}

///
/// Example:
///
/// ```no_run
/// migration!(value, Type1, Type2, ..., TypeN);
/// ```
///
/// Each type must implement `From<T>` where `T` is the previous type.
#[proc_macro]
pub fn migration(input: TokenStream) -> TokenStream {
    migration::migration_macro(input)
}
