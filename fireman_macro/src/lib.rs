use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

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

/// impl name() method for enum, result is &'static str, case is undefined
#[proc_macro_derive(EnumToStr)]
pub fn enum_to_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut items: Vec<syn::Ident> = Vec::new();
    let name = input.ident;
    let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data else {
        panic!("Not a enum!");
    };
    for item in variants {
        items.push(item.ident);
    }

    let expanded = quote! {
        impl #name {
            pub fn name(&self) -> &'static str {
                match self {
                    #(Self::#items => stringify!(#items), )*
                }
            }
        }
    };
    TokenStream::from(expanded)
}

/// Turn Box<T> into &'static T
/// example:
/// ```rust, ignore
/// #[box_to_static_reference]
/// fn example() -> &static T {
///   Box::new(T) // or T.into()
/// }
/// ```
#[proc_macro_attribute]
pub fn box_to_static_reference(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ItemFn);
    let syn::ReturnType::Type(_, return_type) = item.sig.output.clone() else {
        panic!("Return Type Did Not Set");
    };
    let syn::Type::Reference(syn::TypeReference {
        elem: return_type, ..
    }) = return_type.as_ref()
    else {
        panic!("Return Type Is Not Reference")
    };
    let syn::Type::Slice(return_type) = return_type.as_ref() else {
        panic!("Return Type Is Not Slice Reference")
    };
    let attrs = item.attrs;
    let vis = item.vis;
    let sig = item.sig;
    let block = item.block;

    let return_type = return_type.to_token_stream();
    let result = quote! {
        #(#attrs)*
        #vis #sig {
            static ONCE: ::std::sync::LazyLock<::std::boxed::Box<#return_type>> = ::std::sync::LazyLock::new(|| #block);
            ONCE.deref()
        }
    };
    TokenStream::from(result)
}
