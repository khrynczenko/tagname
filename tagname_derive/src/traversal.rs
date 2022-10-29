use syn::punctuated::Pair;
use syn::Attribute;
use syn::{Expr, ExprLit, Lit, Variant};

use super::{Case, Tag, TagData, TaggedUnion};

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
            if !v.attrs.is_empty() {}
            if v.fields.is_empty() {
                Tag::Empty(TagData {
                    ident: v.ident.clone(),
                    case: traverse_attribute(&v),
                })
            } else {
                Tag::NotEmpty(TagData {
                    ident: v.ident.clone(),
                    case: traverse_attribute(&v),
                })
            }
        })
        .collect()
}

fn traverse_attribute(variant: &Variant) -> Case {
    if variant.attrs.is_empty() {
        return Case::Unchanged;
    }
    let tag_attribute = variant.attrs.iter().find(|attr| attr.path.is_ident("tag"));

    if tag_attribute.is_none() {
        return Case::Unchanged;
    }

    traverse_tag_attribute_argument(tag_attribute.unwrap())
}

fn traverse_tag_attribute_argument(attribute: &Attribute) -> Case {
    let expr: Expr = attribute.parse_args().expect(
        r#"`tag` attribute expectes an assignment expression `[tag(case = "lower" | "upper">]`"#,
    );

    let assign_expr = match expr {
        Expr::Assign(assign_expr) => assign_expr,
        _ => {
            panic!(
                r#"`tag` attribute expectes an assignment expression `[tag(case = "lower" | "upper">]`"#
            );
        }
    };

    match *assign_expr.left {
        Expr::Path(_) => {}
        _ => panic!("left handside of the assignme in tag attribute must be `case`"),
    }

    match *assign_expr.right {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s),
            attrs: _,
        }) => match s.value().as_str() {
            "lower" => Case::Lower,
            "upper" => Case::Upper,
            _ => panic!("case value must be either \"upper\" or \"lower\""),
        },
        _ => panic!(
            r#"`tag` attribute expectes an assignment expression `[tag(case = "lower" | "upper">]`"#
        ),
    }
}
