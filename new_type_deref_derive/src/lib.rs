#![recursion_limit = "1024"]

extern crate heck;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::Span;
use syn::{Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Deref)]
pub fn derive_diesel_types(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let type_name = format!("deref_impl_for_{}", name);
    let mod_name = Ident::new(type_name.to_lowercase().as_ref(), Span::call_site());
    let deref_impl = get_deref_impl(&input.data, &name);
    let expanded = quote! {
        mod #mod_name {
            use core::ops::Deref;

            use super::#name;

            #deref_impl

        }
    };
    expanded.into()
}

fn get_deref_impl(data: &Data, name: &Ident) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unnamed(ref fields) => fields
                .unnamed
                .iter()
                .map(|f| {
                    let inner_type = &f.ty;
                    quote! {
                        impl Deref for #name {
                            type Target = #inner_type;

                            fn deref(&self) -> &#inner_type {
                                &self.0
                            }
                        }
                    }
                }).nth(0)
                .unwrap(),
            _ => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}
