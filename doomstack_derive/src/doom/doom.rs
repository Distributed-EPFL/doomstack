use crate::doom::{derive_enum, derive_struct};

use proc_macro::TokenStream;

use quote::quote;

use syn::{Data, DeriveInput};

pub(crate) fn doom(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Struct(data) => derive_struct(ast.ident, ast.attrs, data),
        Data::Enum(data) => derive_enum(ast.ident, data),
        Data::Union(_) => panic!("cannot derive `Doom` on a `union` type"),
    }
}
