use crate::doom::{Configuration, Description};

use proc_macro::TokenStream;

use proc_macro2::Span;

use quote::quote;

use std::collections::HashSet;

use syn::{DataEnum, Fields, Ident, LitStr};

pub(crate) fn derive_enum(name: Ident, data: DataEnum) -> TokenStream {
    let mut variant_idents = Vec::new();
    let mut variant_fields = Vec::new();
    let mut configurations = Vec::new();

    for variant in data.variants {
        let fields = match variant.fields {
            Fields::Named(fields) => Some(fields),
            Fields::Unit => None,
            Fields::Unnamed(_) => panic!("unexpected unnamed fields"),
        };

        let configuration = Configuration::new(&variant.attrs, &fields);

        variant_idents.push(variant.ident);
        variant_fields.push(fields);
        configurations.push(configuration);
    }

    let variant_strs = variant_idents
        .iter()
        .map(|variant| {
            // Using `stringify!` here would insert whitespaces around `::`
            LitStr::new(
                &format!("{}::{}", name.to_string(), variant.to_string()),
                Span::call_site(),
            )
        })
        .collect::<Vec<_>>();

    let variants = quote! {
        const VARIANTS: &'static [&'static str] = &[#(#variant_strs),*];
    };

    let variant_arms = variant_idents
        .iter()
        .enumerate()
        .map(|(index, variant)| quote! {#name::#variant { .. } => #index});

    let variant = quote! {
        fn variant(&self) -> usize {
            match self {
                #(#variant_arms),*
            }
        }
    };

    let binds = configurations
        .iter()
        .map(|configuration| match &configuration.description {
            Description::Static { .. } => Vec::new(),
            Description::Dynamic { arguments, .. } => arguments
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();

    let description_arms = variant_idents.iter().zip(binds.iter()).zip(configurations.iter()).map(|((variant, bind), configuration)| {
        match &configuration.description {
            Description::Static{description} => quote! {
                #name::#variant { .. } => doomstack::Description::Static(#description)
            },
            Description::Dynamic{description, arguments} => quote! {
                #name::#variant{ #(#bind),*, .. } => doomstack::Description::Dynamic(format!(#description, #(#arguments),*))
            }
        }
    });

    let description = quote! {
        fn description(&self) -> doomstack::Description {
            match self {
                #(#description_arms),*
            }
        }
    };

    let wrap_constructors = configurations
        .iter()
        .filter_map(|configuration| configuration.wrap.as_ref())
        .map(|wrap| &wrap.constructor)
        .collect::<Vec<_>>();

    let distinct_wrap_constructors = wrap_constructors.iter().collect::<HashSet<_>>();

    if distinct_wrap_constructors.len() < wrap_constructors.len() {
        panic!("multiple items are `wrap`ed by the same constructor");
    }

    let wraps = variant_idents
        .iter()
        .zip(variant_fields.iter())
        .zip(configurations.iter())
        .map(
            |((variant, fields), configuration)| match &configuration.wrap {
                Some(wrap) => {
                    let constructor = &wrap.constructor;

                    let field = fields.as_ref().unwrap().named.first().unwrap();
                    let field_ident = field.ident.as_ref().unwrap();
                    let field_ty = &field.ty;

                    quote! {
                        fn #constructor(#field_ident: #field_ty) -> Self {
                            #name::#variant { #field_ident }
                        }
                    }
                }
                None => quote! {},
            },
        );

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
                #(#wraps)*
            }
        };
    };

    derive.into()
}
