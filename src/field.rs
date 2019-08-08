use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Ident, Type};

pub struct Field {
  pub name: Ident,
  pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct RegField {
  pub name: String,
  pub ty: String,
}

impl From<&Field> for RegField {
  fn from(field: &Field) -> RegField {
    let ty = &field.ty;
    RegField {
      name: format!("{}", field.name),
      ty: format!("{}", quote!(#ty)),
    }
  }
}

impl From<&RegField> for Field {
  fn from(field: &RegField) -> Field {
    Field {
      name: parse_str(&field.name).unwrap(),
      ty: parse_str(&field.ty).unwrap(),
    }
  }
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

  pub fn make_parent_getter(&self) -> TokenStream {
    let getter1 = &self.name;
    let getter2 = &self.name;
    let type1 = &self.ty;

    quote! {
      pub fn #getter1(&self) -> &#type1 {
        &self.parent_.#getter2()
      }
    }
  }

  pub fn make_parent_setter(&self) -> TokenStream {
    let setter_name = format!("set_{}", self.name);
    let setter = Ident::new(&setter_name, self.name.span());

    let setter1 = &setter;
    let setter2 = &setter;
    let name1 = &self.name;
    let name2 = &self.name;
    let type1 = &self.ty;

    quote! {
      pub fn #setter1(&mut self, #name1: #type1) {
        self.parent_.#setter2(#name2)
      }
    }
  }
}
