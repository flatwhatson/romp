use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct Field {
  pub name: Ident,
  pub ty: Type,
}

impl Field {
  pub fn make_definition(&self) -> TokenStream {
    let name1 = &self.name;
    let type1 = &self.ty;
    quote! {
      #name1: #type1
    }
  }

  pub fn make_getter(&self) -> TokenStream {
    let getter = &self.name;
    let name1 = &self.name;
    let type1 = &self.ty;
    quote! {
      pub fn #getter(&self) -> &#type1 {
        &self.#name1
      }
    }
  }

  pub fn make_setter(&self) -> TokenStream {
    let setter_name = format!("set_{}", self.name);
    let setter = Ident::new(&setter_name, self.name.span());
    let name1 = &self.name;
    let name2 = &self.name;
    let name3 = &self.name;
    let type1 = &self.ty;
    quote! {
      pub fn #setter(&mut self, #name1: #type1) {
        self.#name2 = #name3
      }
    }
  }
}
