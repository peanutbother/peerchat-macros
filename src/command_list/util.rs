use super::{CommandListArgs, ATTRIBUTES_TO_COPY};
use std::collections::HashMap;
use syn::{punctuated::Punctuated, token::Comma, Variant};

pub(crate) fn parse_variants(
    variants: &mut Punctuated<Variant, Comma>,
    variants_parsed: &mut HashMap<String, CommandListArgs>,
) {
    variants.iter_mut().for_each(|variant| {
        let ident = variant.ident.clone();
        let mut args: CommandListArgs = CommandListArgs::default();

        // remove known attributes and parse them into args
        variant.attrs = variant
            .clone()
            .attrs
            .into_iter()
            .filter(|attr| {
                let filter = ATTRIBUTES_TO_COPY
                    .iter()
                    .any(|copy| attr.path.is_ident(copy));

                if filter {
                    if let syn::Meta::NameValue(attribute) =
                        attr.parse_meta().expect("failed to parse meta")
                    {
                        match attribute
                            .path
                            .get_ident()
                            .expect("failed to parse ident")
                            .to_string()
                            .as_str()
                        {
                            | "description" => {
                                let description = match attribute.lit {
                                    | syn::Lit::Str(str) => Some(str.value()),
                                    | _ => None,
                                };
                                args.description = description;
                            },
                            | "hidden" => {
                                let hidden = match attribute.lit {
                                    | syn::Lit::Bool(b) => b.value(),
                                    | _ => false,
                                };
                                args.hidden = hidden;
                            },
                            | _ => {},
                        }
                    }
                }
                !filter
            })
            .collect();

        // add variant ident to ::VARIANTS
        variants_parsed.insert(ident.to_string(), args);
    });
}
pub(crate) fn map_idents(variants_parsed: &HashMap<String, CommandListArgs>) -> Vec<String> {
    let variant_idents: Vec<String> = variants_parsed.keys().map(|k| k.to_owned()).collect();
    variant_idents
}
