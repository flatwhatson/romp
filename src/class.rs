use super::field::Field;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Ident, Result, Token};

pub struct Class {
  name: Ident,
  fields: Vec<Field>,
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
    let names1 = self.fields.iter().map(|f| &f.fname);
    let names2 = self.fields.iter().map(|f| &f.fname);
    let names3 = self.fields.iter().map(|f| &f.fname);
    let types1 = self.fields.iter().map(|f| &f.ftype);
    quote! {
      pub fn new(#(#names1: #types1),*) -> Self {
        Self { #(#names2: #names3),* }
      }
    }
  }
}

impl Parse for Class {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = input.parse()?;
    let block;
    braced!(block in input);
    let items = Punctuated::<Field, Token![,]>::parse_terminated(&block)?;
    let fields = items.into_iter().collect();
    Ok(Class { name, fields })
  }
}
