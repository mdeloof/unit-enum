use proc_macro_error::abort;
use quote::format_ident;
use syn::{Generics, Ident, ItemEnum, Lit, Meta, NestedMeta, Variant};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Model {
    pub variants: Vec<Variant>,
    pub derives: Vec<Ident>,
    pub orig_enum_ident: Ident,
    pub orig_enum_generics: Generics,
    pub unit_enum_ident: Ident,
}

pub fn analyze(item_enum: ItemEnum) -> Model {
    let variants: Vec<Variant> = item_enum.variants.into_iter().collect();
    let orig_enum_ident = item_enum.ident;
    let orig_enum_generics = item_enum.generics;

    let metas: Vec<Meta> = item_enum
        .attrs
        .into_iter()
        // Check if the attribute has an `unit_enum` tag.
        .filter(|attr| attr.path.is_ident("unit_enum"))
        // Try to parse the attribute with the meta format.
        .filter_map(|attr| attr.parse_meta().ok())
        // Check if the attribute is a meta list.
        .filter_map(|meta| match meta {
            Meta::List(list_meta) => Some(list_meta.nested),
            _ => None,
        })
        // Flatten the lists to a list of meta items.
        .flatten()
        // Check if the item is in the meta format.
        .filter_map(|nested| match nested {
            NestedMeta::Meta(meta) => Some(meta),
            _ => None,
        })
        .collect();

    let derives: Vec<Ident> = metas
        .iter()
        // Check if the item has a `derive` tag.
        .filter_map(|meta| match meta {
            Meta::List(meta_list) if meta_list.path.is_ident("derive") => Some(meta_list),
            _ => None,
        })
        // Flatten the lists to a list of meta items.
        .map(|meta_list| &meta_list.nested)
        .flatten()
        // Check if the item is a path.
        .map(|nested| match nested {
            NestedMeta::Meta(Meta::Path(path)) => path,
            meta => abort!(meta, "expected trait name"),
        })
        // Check if the path has an ident.
        .map(|path| match path.get_ident() {
            Some(ident) => ident.clone(),
            None => abort!(path, "expected trait name"),
        })
        .collect();

    let unit_enum_ident = metas
        .iter()
        // Check if the item has a `name` tag.
        .find_map(|meta| match meta {
            Meta::NameValue(name_value) if name_value.path.is_ident("name") => {
                Some(&name_value.lit)
            }
            _ => None,
        })
        // Check if the literal is a string.
        .map(|lit| match lit {
            Lit::Str(lit_str) => format_ident!("{}", lit_str.value()),
            lit => abort!(lit, "expected string literal"),
        })
        // If no name is given, we generate one.
        .unwrap_or(format_ident!("{}Unit", orig_enum_ident));

    Model {
        orig_enum_generics,
        variants,
        derives,
        orig_enum_ident,
        unit_enum_ident,
    }
}

#[test]
fn test_analyze() {
    use syn::parse_quote;

    let item_enum = parse_quote!(
        #[unit_enum(name = "AllUnit", derive(PartialEq))]
        enum _All<T> {
            Unit,
            Tuple(bool, i32),
            Struct { x: T, y: T },
        }
    );

    let actual = analyze(item_enum);

    let variants = vec![
        parse_quote!(Unit),
        parse_quote!(Tuple(bool, i32)),
        parse_quote!(Struct { x: T, y: T }),
    ];

    let derives = vec![parse_quote!(PartialEq)];
    let orig_enum_ident = parse_quote!(_All);
    let orig_enum_generics = parse_quote!(<T>);
    let unit_enum_ident = parse_quote!(AllUnit);

    let expected = Model {
        variants,
        derives,
        orig_enum_ident,
        orig_enum_generics,
        unit_enum_ident,
    };

    assert_eq!(actual, expected);
}
