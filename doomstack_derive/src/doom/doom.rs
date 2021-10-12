use proc_macro::TokenStream;

use quote::quote;

pub(crate) fn doom(_input: TokenStream) -> TokenStream {
    let derive = quote! {};
    derive.into()
}
