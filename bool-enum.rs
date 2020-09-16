//! A macro for creating semantic wrappers over a boolean value
//! 
//! The simple way to create the enum of two possible states,
//! 
//! The bool_enums implement From<bool> traits and are clonable
//! 
//! ```
//! // This private item:
//! bool_enum!(
//!     #[metadata]
//!     MyEnum: MyFalse=0, MyTrue=1
//! )
//! 
//! // Is equivalent to
//! #[metadata]
//! #derive(Clone)
//! enum MyEnum {MyFalse=0, MyTrue=1}
//! impl core::convert::From<bool> for MyEnum {
//!     ...
//! }
//! 
//! // Also public item definitions are possible:
//! bool_enum!{
//!     /// Documentation
//!     pub MyPublicEnum: MyFalse=0, MyTrue=1
//! }
//! ```

#![no_std]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


fn build_enum (
    original: syn::ItemEnum,

    name: syn::Ident,
    variant_1: syn::Ident,
    variant_0: syn::Ident,
) -> TokenStream {
    let text = quote! {
        #original

        impl core::convert::From<bool> for #name {
            fn from(val: bool) -> Self {
                match val {
                    false => Self::#variant_0,
                    true => Self::#variant_1,
                }
            }
        }
    };

    text.into()
}


#[proc_macro_attribute]
pub fn bool_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item : syn::ItemEnum = syn::parse(item).expect("Expected enum");

    let mut full_item = item.clone();

    let name = item.ident;

    if item.variants.len() != 2 {
        panic!("Enum must have 2 variants");
    }

    let v1 = item.variants[0].clone();
    let v2 = item.variants[1].clone();

    let itemstruct : syn::ItemStruct = syn::parse(quote!{ #[allow(missing_docs)] struct S; }.into()).unwrap();

    full_item.variants[0].attrs.push(itemstruct.attrs[0].clone());
    full_item.variants[1].attrs.push(itemstruct.attrs[0].clone());

    match (v1.discriminant, v2.discriminant) {
        (Some((syn::token::Eq{spans:_}, syn::Expr::Lit(l1))), 
        Some((syn::token::Eq{spans:_}, syn::Expr::Lit(l2)))) => {
            if !(l1.attrs.is_empty() && l2.attrs.is_empty()) {
                panic!("No attributes at discriminants please")
            }

            match (l1.lit, l2.lit) {
                (syn::Lit::Int(l1i), syn::Lit::Int(l2i)) => {
                    match (l1i.base10_digits(), l2i.base10_digits()) {
                        ("0","1") => {
                            build_enum(full_item, name, v2.ident, v1.ident)
                        }
                        ("1","0") => {
                            build_enum(full_item, name, v1.ident, v2.ident)
                        }

                        _ => {
                            panic!("The discriminants must be 1 and 0")
                        }
                    }
                }
                _ => {
                    panic!("The discriminants must be integer literals")
                }
            }
        }

        _ => {
            panic!("provide the discriminants for values")
        }
    }
}
