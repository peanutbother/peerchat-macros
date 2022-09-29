use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn impl_from_string_default(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<String> for #name {
            fn from(json: String) -> Self {
                serde_json::from_str::<Self>(&json).unwrap_or_default()
            }
        }
    };

    gen.into()
}
