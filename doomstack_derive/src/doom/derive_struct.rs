use crate::doom::{Configuration, Description};

use proc_macro::TokenStream;

use quote::quote;

use syn::{Attribute, DataStruct, Fields, Ident};

pub(crate) fn derive_struct(name: Ident, attrs: Vec<Attribute>, data: DataStruct) -> TokenStream {
    let fields = match data.fields {
        Fields::Named(fields) => Some(fields),
        Fields::Unit => None,
        Fields::Unnamed(_) => panic!("unexpected unnamed fields"),
    };

    let configuration = Configuration::new(&attrs, &fields);

    let variants = quote! {
        const VARIANTS: &'static [&'static str] = &[stringify!(#name)];
    };

    let variant = quote! {
        fn variant(&self) -> usize {
            0
        }
    };

    let description = match configuration.description {
        Description::Static { description } => quote! {
            fn description(&self) -> doomstack::Description {
                #description.into()
            }
        },
        Description::Dynamic {
            description,
            arguments,
        } => {
            let arguments = arguments.iter().map(|argument| quote! {self.#argument});

            quote! {
                fn description(&self) -> doomstack::Description {
                    format!(#description, #(#arguments),*).into()
                }
            }
        }
    };

    let derive = quote! {
        const _: () = {
            static STORE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

            impl doomstack::Doom for #name {
                #variants

                fn acquire() {
                    STORE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                fn release() {
                    STORE.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                }

                fn store() -> usize {
                    STORE.load(std::sync::atomic::Ordering::Relaxed) > 0
                }

                #variant
                #description
            }
        };
    };

    derive.into()
}
