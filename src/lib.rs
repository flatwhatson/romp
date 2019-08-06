extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input};
use syn::{Ident, Result, Token, Type};

struct Class {
  name: Ident,
  fields: Punctuated<Field, Token![,]>,
}

struct Field {
  fname: Ident,
  ftype: Type,
}

impl Parse for Class {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = input.parse()?;
    let content;
    braced!(content in input);
    let fields = content.parse_terminated(Field::parse)?;
    Ok(Class { name, fields })
  }
}

impl Parse for Field {
  fn parse(input: ParseStream) -> Result<Self> {
    let fname = input.parse()?;
    input.parse::<Token![:]>()?;
    let ftype = input.parse()?;
    Ok(Field { fname, ftype })
  }
}

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
  let Class { name, fields } = parse_macro_input!(input);

  let mut names = Vec::new();
  let mut types = Vec::new();
  let mut strings = Vec::new();
  let mut getters = Vec::new();
  let mut setters = Vec::new();

  for Field { fname, ftype } in fields {
    getters.push(fname.clone());
    strings.push(format!("set_{}", fname));
    setters.push(Ident::new(strings.last().unwrap(), fname.span()));
    names.push(fname);
    types.push(ftype);
  }

  let names1 = &names;
  let names2 = &names;
  let names3 = &names;
  let names4 = &names;
  let names5 = &names;
  let names6 = &names;
  let names7 = &names;
  let names8 = &names;

  let types1 = &types;
  let types2 = &types;
  let types3 = &types;
  let types4 = &types;

  let output = quote! {
    #[derive(Debug)]
    pub struct #name {
      #(#names1: #types1),*
    }
    impl #name {
      pub fn new(#(#names2: #types2),*) -> Self {
        Self { #(#names3: #names4),* }
      }
      #(
        pub fn #getters(&self) -> &#types3 {
          &self.#names5
        }
        pub fn #setters(&mut self, #names6: #types4) {
          self.#names7 = #names8
        }
      )*
    }
  };

  TokenStream::from(output)
}
