use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Type, parse_macro_input};

#[proc_macro_derive(Versioned)]
pub fn my_versioned_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    if let Some(err) = check_version_field(&input, name) {
        return err;
    }

    // --- Generate something if needed ---
    let expanded = quote! {
        impl Into<serde_json::Value> for #name {
            fn into(self) -> serde_json::Value {
                return serde_json::to_value(self).expect("Serialize to_value failed!");
            }
        }

        impl From<serde_json::Value> for #name {
            fn from(value: serde_json::Value) -> Self {
                return serde_json::from_value(value).expect("Deserialize from_value failed!");
            }
        }

        impl rs_utils::config::Versioned for #name {
            fn get_version(&self) -> i64 {
                self.version.into()
            }
        }
    };

    expanded.into()
}

fn check_version_field(input: &DeriveInput, struct_ident: &Ident) -> Option<TokenStream> {
    // --- CHECK STRUCT TYPE ---
    let data_struct = match &input.data {
        Data::Struct(s) => s,
        _ => {
            let err =
                syn::Error::new_spanned(struct_ident, "`Versioned` can only be used on structs")
                    .to_compile_error()
                    .into();

            return Some(err);
        }
    };

    // --- CHECK FIELDS ---
    let mut version_ty: Option<Type> = None;

    match &data_struct.fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                if let Some(ident) = &field.ident {
                    if ident == "version" {
                        version_ty = Some(field.ty.clone());
                        break;
                    }
                }
            }
        }
        _ => {}
    }

    let has_version = version_ty.is_some();

    // --- ERROR IF MISSING VERSION FIELD ---
    if !has_version {
        let err = syn::Error::new_spanned(
            struct_ident,
            "Struct deriving `Versioned` must have a `version` field",
        )
        .to_compile_error()
        .into();

        return Some(err);
    }

    let version_ty = version_ty.unwrap();

    let is_numeric = match &version_ty {
        Type::Path(tp) => {
            let ident = tp.path.segments.last().unwrap().ident.to_string();

            matches!(
                ident.as_str(),
                "u8" | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "usize"
                    | "isize"
                    | "f32"
                    | "f64" // remove these if float is not allowed
            )
        }
        _ => false,
    };

    if !is_numeric {
        let err = syn::Error::new_spanned(version_ty, "`version` field must be a numeric type")
            .to_compile_error()
            .into();

        return Some(err);
    }

    return None;
}
