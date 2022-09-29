use super::{
    tokens::{DescriptionBranch, FromStringBranch, HiddenBranch, ToStringBranch},
    CommandListArgs,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{punctuated::Punctuated, token::Comma, Variant};

// TODO implement From<String> => Self
pub(crate) fn _create_from_string_impl(
    name: &syn::Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, Comma>,
) -> TokenStream {
    let branches: Vec<FromStringBranch> = variants
        .iter()
        .map(|variant| FromStringBranch(name.clone(), variant.ident.clone()))
        .collect();
    quote! {
        impl From<String> for #name { // <- wil not work, missing self reference
            pub fn from_string(variant: String) -> Self {
                match self.to_owned() {
                    #(#branches),*,
                    _ => "_Unknown"
                }
            }
        }
    }
}

pub fn create_to_string_impl(variants: &Punctuated<Variant, Comma>) -> TokenStream {
    let branches: Vec<ToStringBranch> = variants
        .iter()
        .map(|variant| ToStringBranch(variant.ident.clone()))
        .collect();
    quote! {
        pub fn to_string(&self) -> &'static str {
            match self.to_owned() {
                #(#branches),*,
                _ => "_Unknown"
            }
        }
    }
}

pub(crate) fn create_description_impl(
    variants: &Punctuated<Variant, Comma>,
    variants_parsed: &HashMap<String, CommandListArgs>,
) -> TokenStream {
    let branches: Vec<DescriptionBranch> = variants
        .iter()
        .zip(variants_parsed.values())
        .map(|(variant, args)| {
            DescriptionBranch(variant.ident.to_owned(), args.description.to_owned())
        })
        .collect();
    quote! {
        pub fn description(&self) -> Option<&'static str> {
            match self.to_owned() {
                #(#branches),*,
                // &Self::_Unknown => None,
                _ => None
            }
        }
    }
}

pub(crate) fn create_is_hidden_impl(
    variants: &Punctuated<Variant, Comma>,
    variants_parsed: &HashMap<String, CommandListArgs>,
) -> TokenStream {
    let branches: Vec<HiddenBranch> = variants
        .iter()
        .zip(variants_parsed.values())
        .map(|(variant, args)| HiddenBranch(variant.ident.clone(), args.hidden))
        .collect();
    quote! {
        pub fn is_hidden(&self) -> bool {
            match self.to_owned() {
                #(#branches),*,
                // &Self::_Unknown => true,
                _ => false
            }
        }
    }
}
