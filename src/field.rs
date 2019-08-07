use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token, Type};

pub struct Field {
  pub fname: Ident,
  pub ftype: Type,
}

impl Field {
  pub fn make_definition(&self) -> TokenStream {
    let name1 = &self.fname;
    let type1 = &self.ftype;
    quote! {
      #name1: #type1
    }
  }

  pub fn make_getter(&self) -> TokenStream {
    let getter = &self.fname;
    let name1 = &self.fname;
    let type1 = &self.ftype;
    quote! {
      pub fn #getter(&self) -> &#type1 {
        &self.#name1
      }
    }
  }

  pub fn make_setter(&self) -> TokenStream {
    let setter_name = format!("set_{}", self.fname);
    let setter = Ident::new(&setter_name, self.fname.span());
    let name1 = &self.fname;
    let name2 = &self.fname;
    let name3 = &self.fname;
    let type1 = &self.ftype;
    quote! {
      pub fn #setter(&mut self, #name1: #type1) {
        self.#name2 = #name3
      }
    }
  }
}

impl Parse for Field {
  fn parse(input: ParseStream) -> Result<Self> {
    let fname = input.parse()?;
    <Token![:]>::parse(input)?;
    let ftype = input.parse()?;
    Ok(Field { fname, ftype })
  }
}
