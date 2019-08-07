extern crate proc_macro;

mod class;
mod declare;
mod field;

use declare::Declarations;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
  let declarations = parse_macro_input!(input as Declarations);
  let classes = declarations.classes.iter().map(|c| c.generate());
  (quote! {#(#classes)*}).into()
}
