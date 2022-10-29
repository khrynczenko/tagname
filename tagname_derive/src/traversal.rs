use syn::punctuated::Pair;
use syn::Variant;

use super::{Tag, TagData, TaggedUnion};

pub(crate) fn traverse_ast(ast: syn::DeriveInput) -> TaggedUnion {
    let name = ast.ident.clone();
    let variants = traverse_enum(ast);

    assert!(
        !variants.is_empty(),
        "cannot derive TagName for empty enum types"
    );

    let tags = traverse_variants(variants);

    TaggedUnion { name, tags }
}

fn traverse_enum(ast: syn::DeriveInput) -> Vec<Variant> {
    match ast.data {
        syn::Data::Enum(enum_data) => enum_data
            .variants
            .into_pairs()
            .into_iter()
            .map(Pair::into_value)
            .collect(),
        _ => panic!("cannot derive TagName for non-enum types"),
    }
}

fn traverse_variants(variants: Vec<Variant>) -> Vec<Tag> {
    variants
        .into_iter()
        .map(|v| {
            if v.fields.is_empty() {
                Tag::Empty(TagData { ident: v.ident })
            } else {
                Tag::NotEmpty(TagData { ident: v.ident })
            }
        })
        .collect()
}
