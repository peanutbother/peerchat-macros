use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemEnum};

mod command_list;
mod from_string_default;

#[proc_macro_derive(FromJSON)]
pub fn from_string_default(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    from_string_default::impl_from_string_default(&ast)
}

#[proc_macro_attribute]
pub fn command_list(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let args = parse_macro_input!(args as AttributeArgs);
    command_list::impl_command_list(input, args)
}
