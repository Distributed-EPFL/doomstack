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

    let wrap = match configuration.wrap {
        Some(wrap) => {
            let constructor = &wrap.constructor;

            let field = fields.as_ref().unwrap().named.first().unwrap();
            let field_ident = field.ident.as_ref().unwrap();
            let field_ty = &field.ty;

            quote! {
                pub fn #constructor(#field_ident: #field_ty) -> Self {
                    #name { #field_ident }
                }
            }
        }
        None => {
            quote! {}
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

                fn store() -> bool {
                    STORE.load(std::sync::atomic::Ordering::Relaxed) > 0
                }

                #variant
                #description
            }

            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    write!(f, "{}", Doom::description(self))
                }
            }

            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    write!(f, "{}", Doom::description(self))
                }
            }

            impl std::error::Error for #name {}

            impl #name {
                #wrap
            }
        };
    };

    derive.into()
}
