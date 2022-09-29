use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{token::Comma, AttributeArgs, ItemEnum};

mod impls;
mod tokens;
mod util;

pub(crate) const DESCRIPTION_ATTR_NAME: &'static str = "description";
pub(crate) const HIDDEN_ATTR_NAME: &'static str = "hidden";
pub(crate) const ATTRIBUTES_TO_COPY: &'static [&'static str] =
    &[DESCRIPTION_ATTR_NAME, HIDDEN_ATTR_NAME];

#[derive(Debug, Default)]
pub(crate) struct CommandListArgs {
    description: Option<String>,
    hidden: bool,
}

pub(crate) fn impl_command_list(
    input: ItemEnum,
    _args: AttributeArgs,
) -> proc_macro::TokenStream {
    let ItemEnum {
        ident,
        mut variants,
        generics,
        vis,
        ..
    } = input;
    let mut variants_parsed: HashMap<String, CommandListArgs> = HashMap::new();

    // remove _Unknown variant if it exists
    variants = variants
        .into_iter()
        .filter(|v| v.ident != *"_Unknown")
        .collect();

    // ensure trailing punctuation
    if !variants.trailing_punct() {
        let spans = variants.last().expect("empty variant");
        let spans = match spans.discriminant {
            | Some((eq, _)) => eq.spans,
            | None => [spans.ident.span()],
        };

        variants.push_punct(Comma(spans));
    }

    // parse variants with attributes
    util::parse_variants(&mut variants, &mut variants_parsed);

    // map variables to serializable structures
    let variant_idents = util::map_idents(&variants_parsed);
    let impl_to_string = impls::create_to_string_impl(&variants);
    let impl_to_description = impls::create_description_impl(&variants, &variants_parsed);
    let impl_to_hidden = impls::create_is_hidden_impl(&variants, &variants_parsed);
    // let branches_from_string: Vec<BranchMapFromString> = map_from_string(&ident, &variants);
    let unknown_ident = format_ident!("_Unknown");

    let output = quote! {
        #vis enum #ident #generics {
            #variants
            #unknown_ident
        }
        impl #ident #generics {
            /// static array with every variant of this enum
            pub const VARIANTS: &'static[&'static str] = &[ #(#variant_idents),* ];
            /// returns a string representation for this variant
            #impl_to_string

            /// returns the description set on variant with `#[description = ""]`
            #impl_to_description

            /**
                returns `false` for every variant if not set otherwise
                `_Unknown` is a helper variant which is hidden by default
            */
            #impl_to_hidden
        }
        // impl From<String> for #ident {
        //     fn from(variant: String) -> #ident {
        //         match variant.as_str() {
        //             #(#branches_from_string),*,
        //             _ => #_Unknown,
        //         }
        //     }
        // }
    };

    // println!("{}", output.to_string());

    output.into()
}
