use proc_macro2::TokenStream;
use quote::quote;

use super::lower::Ir;

pub fn codegen(ir: Ir) -> TokenStream {
    let visibility = ir.visibility;
    let orig_enum_ident = ir.orig_enum_ident;
    let orig_enum_generics = ir.orig_enum_generics;
    let unit_enum_ident = ir.unit_enum_ident;
    let variants = ir.variants;
    let derives = ir.derives;
    let variant_arms = ir.variant_arms;

    let tokens = quote!(
        #[derive(#(#derives),*)]
        #visibility enum #unit_enum_ident {
            #(#variants),*
        }

        impl #orig_enum_generics From<&#orig_enum_ident #orig_enum_generics> for #unit_enum_ident {
            fn from(orig: &#orig_enum_ident #orig_enum_generics) -> Self {
                match orig {
                    #(#variant_arms),*
                }
            }

        }
    );
    tokens
}

#[test]
fn test_codegen() {
    use syn::{parse2, parse_quote, File};

    let visibility = parse_quote!(pub);
    let orig_enum_ident = parse_quote!(State);
    let orig_enum_generics = parse_quote!(<>);
    let unit_enum_ident = parse_quote!(StateUnit);
    let variants = vec![parse_quote!(On), parse_quote!(Off)];
    let derives = vec![parse_quote!(PartialEq)];
    let variant_arms = vec![
        parse_quote!(State::On => StateUnit::On),
        parse_quote!(State::Off => StateUnit::Off),
    ];

    let ir = Ir {
        visibility,
        orig_enum_ident,
        orig_enum_generics,
        unit_enum_ident,
        variants,
        derives,
        variant_arms,
    };
    let rust = codegen(ir);

    assert!(parse2::<File>(rust).is_ok());
}
