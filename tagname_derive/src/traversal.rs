use proc_macro2::TokenStream;
use syn::punctuated::Pair;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::{Expr, ExprLit, Fields, Lit, Variant};

use super::{Case, Tag, TagData, TaggedUnion};

#[derive(Debug)]
pub struct Error(pub TokenStream);

pub(crate) fn traverse_ast(ast: syn::DeriveInput) -> Result<TaggedUnion, Error> {
    let name = ast.ident.clone();
    let variants = traverse_enum(ast)?;

    if variants.is_empty() {
        return Err(Error(
            quote::quote_spanned! {name.span()=> compile_error!("cannot derive `TagName` for empty enum types"); },
        ));
    }

    let tags = traverse_variants(variants)?;

    Ok(TaggedUnion { name, tags })
}

fn traverse_enum(ast: syn::DeriveInput) -> Result<Vec<Variant>, Error> {
    match ast.data {
        syn::Data::Enum(enum_data) => Ok(enum_data
            .variants
            .into_pairs()
            .into_iter()
            .map(Pair::into_value)
            .collect()),
        syn::Data::Struct(s) => Err(Error(
            quote::quote_spanned! {s.struct_token.span()=> compile_error!("cannot derive `TagName` for struct types"); },
        )),
        syn::Data::Union(u) => Err(Error(
            quote::quote_spanned! {u.union_token.span()=> compile_error!("cannot derive `TagName` for union types"); },
        )),
    }
}

fn traverse_variants(variants: Vec<Variant>) -> Result<Vec<Tag>, Error> {
    let mut tags = Vec::new();
    for v in variants {
        if !v.attrs.is_empty() {}
        tags.push(match v.fields {
            Fields::Unit => Tag::Unit(TagData {
                ident: v.ident.clone(),
                case: traverse_attribute(&v)?,
            }),
            Fields::Unnamed(_) => Tag::Unnamed(TagData {
                ident: v.ident.clone(),
                case: traverse_attribute(&v)?,
            }),
            Fields::Named(_) => Tag::Named(TagData {
                ident: v.ident.clone(),
                case: traverse_attribute(&v)?,
            }),
        });
    }
    Ok(tags)
}

fn traverse_attribute(variant: &Variant) -> Result<Case, Error> {
    if variant.attrs.is_empty() {
        return Ok(Case::Unchanged);
    }
    let tag_attribute = variant.attrs.iter().find(|attr| attr.path.is_ident("tag"));

    if tag_attribute.is_none() {
        return Ok(Case::Unchanged);
    }

    traverse_tag_attribute_argument(tag_attribute.unwrap())
}

fn traverse_tag_attribute_argument(attribute: &Attribute) -> Result<Case, Error> {
    let expr: Expr = attribute.parse_args().map_err(move |_| {
        Error(quote::quote_spanned! {attribute.span()=> compile_error!(r#"`tag` attribute expects an assignment expression `[tag(case = "lower" | "upper")]`"#); })
    })?;

    let assign_expr = match expr {
        Expr::Assign(assign_expr) => assign_expr,
        _ => {
            return Err(Error(
                quote::quote_spanned! {expr.span()=> compile_error!(r#"`tag` attribute expects an assignment expression `[tag(case = "lower" | "upper")]`"#); },
            ));
        }
    };

    match *assign_expr.left {
        Expr::Path(ref p)
            if !p.path.segments.is_empty() && p.path.segments.first().unwrap().ident == "case" => {}
        _ => {
            return Err(Error(
                quote::quote_spanned! {assign_expr.span()=> compile_error!("left handside of the assignment in `tag` attribute must be `case`");},
            ))
        }
    }

    match *assign_expr.right {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s),
            attrs: _,
        }) => match s.value().as_str() {
            "lower" => Ok(Case::Lower),
            "upper" => Ok(Case::Upper),
            _ => Err(Error(
                quote::quote_spanned! {s.span()=> compile_error!("`case` value must be either \"upper\" or \"lower\"");},
            )),
        },
        _ => Err(Error(
            quote::quote_spanned! {assign_expr.right.span()=> compile_error!(r#"right handside of the assignment expression is expected to be a string literal "lower" or "upper""#);},
        )),
    }
}
