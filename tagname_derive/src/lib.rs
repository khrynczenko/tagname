use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Variant};

#[proc_macro_derive(Variant)]
pub fn variant_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_variant_derive_macro(ast)
}

fn impl_variant_derive_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let variants: Vec<Variant> = match ast.data {
        syn::Data::Enum(enum_data) => enum_data
            .variants
            .into_pairs()
            .into_iter()
            .map(|pair| pair.into_value())
            .collect(),
        _ => panic!("cannot derive Variant for non-enum types"),
    };

    let variants_with_fields = variants.iter().cloned().filter(|v| !v.fields.is_empty());
    let variants_without_fields = variants.iter().cloned().filter(|v| v.fields.is_empty());
    let with_field_tags: Vec<Ident> = variants_with_fields.into_iter().map(|v| v.ident).collect();
    let no_field_tags: Vec<Ident> = variants_without_fields
        .into_iter()
        .map(|v| v.ident)
        .collect();

    let gen = quote! {
        impl Variant for #name {
            fn tag_name(&self) -> &'static str {
                match self {
                    #(
                        #name::#with_field_tags(_) => stringify!(#with_field_tags)
                    ),*,
                    #(
                        #name::#no_field_tags => stringify!(#no_field_tags)
                    ),*
                }
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
