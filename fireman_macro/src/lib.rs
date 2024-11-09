use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// impl FromStr for enum, case will be ignored
#[proc_macro_derive(StrToEnum, attributes(str_to_enum_type, str_to_enum_item))]
pub fn str_to_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut items: Vec<syn::Ident> = Vec::new();
    let name = input.ident;
    let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data else {
        panic!("Not a enum!");
    };
    for item in variants {
        items.push(item.ident);
    }

    let str_to_enum_type: syn::Expr = input
        .attrs
        .iter()
        .find(|x| x.path().is_ident("str_to_enum_type"))
        .expect("str_to_enum_type attributes not served")
        .parse_args()
        .expect("str_to_enum error type not served. use #[str_to_enum_type(MyType)]");
    let str_to_enum_item: syn::Expr = input
        .attrs
        .iter()
        .find(|x| x.path().is_ident("str_to_enum_item"))
        .expect("str_to_enum_item attributes not served")
        .parse_args()
        .expect("str_to_enum error data not served. use #[str_to_enum_item(MyType::A)]");

    let expanded = quote! {
        impl ::core::str::FromStr for #name {
            type Err = #str_to_enum_type;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.to_ascii_lowercase();
                #(if s == stringify!(#items).to_ascii_lowercase() { Ok(Self::#items) }) else *
                else {
                    Err(#str_to_enum_item)
                }
            }
        }
    };
    TokenStream::from(expanded)
}
