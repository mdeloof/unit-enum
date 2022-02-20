use super::analyze::Model;
use syn::{parse_quote, Visibility};
use syn::{Arm, Ident, Variant, Fields, Generics};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Ir {
    pub visibility: Visibility,
    pub orig_enum_ident: Ident,
    pub orig_enum_generics: Generics,
    pub unit_enum_ident: Ident,
    pub variants: Vec<Variant>,
    pub derives: Vec<Ident>,
    pub variant_arms: Vec<Arm>,
}

pub fn lower(model: Model) -> Ir {
    let visibility = model.visibility;
    let orig_enum_ident = model.orig_enum_ident;
    let orig_enum_generics = model.orig_enum_generics;
    let unit_enum_ident = model.unit_enum_ident;
    let variants = model
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .map(|ident| parse_quote!(#ident))
        .collect();
    let derives = model.derives;
    let variant_arms = model.variants.iter().map(|variant| {
        let variant_ident = &variant.ident; 
        match variant.fields {
            Fields::Unit => parse_quote!(#orig_enum_ident::#variant_ident => #unit_enum_ident::#variant_ident),
            Fields::Named(_) => parse_quote!(#orig_enum_ident::#variant_ident { .. } => #unit_enum_ident::#variant_ident),
            Fields::Unnamed(_) => parse_quote!(#orig_enum_ident::#variant_ident(..) => #unit_enum_ident::#variant_ident)
        }
    }).collect();
    Ir {
        visibility,
        orig_enum_ident,
        orig_enum_generics,
        variants,
        unit_enum_ident,
        derives,
        variant_arms
    }
}

#[test]
fn test_lower() {
    let variants = vec![
        parse_quote!(Unit),
        parse_quote!(Tuple(bool, i32)),
        parse_quote!(Struct { x: i32, y: i32 }),
    ];

    let derives = vec![parse_quote!(PartialEq)];
    let visibility = parse_quote!(pub);
    let orig_enum_ident = parse_quote!(All);
    let orig_enum_generics = parse_quote!(<T>);
    let unit_enum_ident = parse_quote!(AllUnit);

    let model = Model {
        visibility,
        variants,
        derives,
        orig_enum_ident,
        orig_enum_generics,
        unit_enum_ident,
    };

    let actual = lower(model);

    let unit_enum_ident = parse_quote!(AllUnit);

    let variants = vec![
        parse_quote!(Unit),
        parse_quote!(Tuple),
        parse_quote!(Struct),
    ];

    let visibility = parse_quote!(pub);
    let orig_enum_ident = parse_quote!(All);
    let orig_enum_generics = parse_quote!(<T>);

    let derives = vec![parse_quote!(PartialEq)];

    let variant_arms = vec![
        parse_quote!(All::Unit => AllUnit::Unit),
        parse_quote!(All::Tuple(..) => AllUnit::Tuple),
        parse_quote!(All::Struct { .. } => AllUnit::Struct),
    ];

    let expected = Ir {
        visibility,
        orig_enum_ident,
        orig_enum_generics,
        variants,
        derives,
        unit_enum_ident,
        variant_arms
    };

    assert_eq!(actual, expected);
}
