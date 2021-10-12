use proc_macro::TokenStream;

use quote::quote;

use syn::{DataEnum, Ident};

pub(crate) fn derive_enum(name: Ident, data: DataEnum) -> TokenStream {
    let derive = quote! {};
    derive.into()
}
