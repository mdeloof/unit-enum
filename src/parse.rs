use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{Item, ItemEnum};

pub fn parse(item: TokenStream) -> ItemEnum {
    let result: Result<Item, _> = syn::parse2(item);
    match result {
        Ok(Item::Enum(item_enum)) => item_enum,
        Ok(item) => abort!(
            item,
            "item is not an enum";
            help = "`ItemEnum` can only be derived from an enum"
        ),
        Err(_) => unreachable!(),
    }
}

#[test]
fn valid_input() {
    use quote::quote;

    parse(quote!(
        #[derive(UnitEnum)]
        enum State {
            On,
            Off,
        }
    ));
}

#[test]
#[should_panic]
fn invalid_input() {
    use quote::quote;

    parse(quote!(
        #[derive(UnitEnum)]
        struct Vec2 {
            x: i32,
            y: i32,
        }
    ));
}
