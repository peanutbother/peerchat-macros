use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::Ident;

#[derive(Debug)]
pub(crate) struct ToStringBranch(pub Ident);
impl ToTokens for ToStringBranch {
    fn to_tokens(
        &self,
        tokens: &mut TokenStream,
    ) {
        let ident = self.0.clone();
        let value = self.0.to_string();
        let mapping = quote!(Self::#ident => #value);
        // println!("{:#?}", mapping);
        tokens.append_all(mapping);
    }
}

#[derive(Debug)]
pub(crate) struct FromStringBranch(pub Ident, pub Ident);
impl ToTokens for FromStringBranch {
    // todo implement
    fn to_tokens(
        &self,
        _tokens: &mut TokenStream,
    ) {
        // let ident = self.1.clone();
        // let value = self.1.to_string();

        // let mapping = quote!(#value => #ident);
        // tokens.append_all(mapping);
        todo!("From<String> needs self ref");
    }
}

#[derive(Debug)]
pub(crate) struct DescriptionBranch(pub Ident, pub Option<String>);
impl ToTokens for DescriptionBranch {
    fn to_tokens(
        &self,
        tokens: &mut TokenStream,
    ) {
        let ident = self.0.clone();
        let value = match self.1.clone() {
            | Some(value) => quote!(Some(#value)),
            | None => quote!(None),
        };
        let mapping = quote!(Self::#ident => #value);
        tokens.append_all(mapping);
    }
}

#[derive(Debug)]
pub(crate) struct HiddenBranch(pub Ident, pub bool);
impl ToTokens for HiddenBranch {
    fn to_tokens(
        &self,
        tokens: &mut TokenStream,
    ) {
        let ident = self.0.clone();
        let value = self.1;
        let mapping = quote!(Self::#ident => #value);
        tokens.append_all(mapping);
    }
}
