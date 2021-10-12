// Modules

mod doom;
mod parse;

// Macros

use doom::doom;

// Interface

use proc_macro::TokenStream;

#[proc_macro_derive(Doom, attributes(doom))]
pub fn doom_derive(input: TokenStream) -> TokenStream {
    doom(input)
}
