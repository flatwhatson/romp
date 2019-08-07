use super::field::Field;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub struct Class {
  pub name: Ident,
  pub fields: Vec<Field>,
}

impl Class {
  pub fn generate(&self) -> TokenStream {
    let definition = self.make_definition();
    let implementation = self.make_implementation();
    quote! {
      #definition
      #implementation
    }
  }

  fn make_definition(&self) -> TokenStream {
    let name = &self.name;
    let definitions = self.fields.iter().map(|f| f.make_definition());
    quote! {
      #[derive(Debug)]
      pub struct #name {
        #(#definitions),*
      }
    }
  }

  fn make_implementation(&self) -> TokenStream {
    let name = &self.name;
    let constructor = self.make_constructor();
    let getters = self.fields.iter().map(|f| f.make_getter());
    let setters = self.fields.iter().map(|f| f.make_setter());
    quote! {
      impl #name {
        #constructor
        #(#getters)*
        #(#setters)*
      }
    }
  }

  fn make_constructor(&self) -> TokenStream {
    let names1 = self.fields.iter().map(|f| &f.name);
    let names2 = self.fields.iter().map(|f| &f.name);
    let names3 = self.fields.iter().map(|f| &f.name);
    let types1 = self.fields.iter().map(|f| &f.ty);
    quote! {
      pub fn new(#(#names1: #types1),*) -> Self {
        Self { #(#names2: #names3),* }
      }
    }
  }
}
