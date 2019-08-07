extern crate proc_macro;

mod class;
mod field;

use class::Class;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
  parse_macro_input!(input as Class).generate().into()
}
