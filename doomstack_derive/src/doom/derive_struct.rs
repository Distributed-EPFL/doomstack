use proc_macro::TokenStream;

use quote::quote;

use syn::{Attribute, DataStruct, Ident};

pub(crate) fn derive_struct(name: Ident, attrs: Vec<Attribute>, data: DataStruct) -> TokenStream {
    let derive = quote! {};
    derive.into()
}
