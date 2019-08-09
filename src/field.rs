use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Ident, Type};

pub struct Field {
  pub name: Ident,
  pub ty: Type,
}

pub struct RegField {
  pub name: String,
  pub ty: String,
}

enum MethodStyle {
  Trait,
  Field,
  Parent,
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
  pub fn make_trait_getter(&self) -> TokenStream {
    self.make_getter_(MethodStyle::Trait)
  }

  pub fn make_field_getter(&self) -> TokenStream {
    self.make_getter_(MethodStyle::Field)
  }

  pub fn make_parent_getter(&self) -> TokenStream {
    self.make_getter_(MethodStyle::Parent)
  }

  fn make_getter_(&self, style: MethodStyle) -> TokenStream {
    let getter1 = &self.name;
    let getter2 = &self.name;
    let name1 = &self.name;
    let type1 = &self.ty;

    match style {
      MethodStyle::Trait => quote! {
        fn #getter1(&self) -> &#type1;
      },
      MethodStyle::Field => quote! {
        fn #getter1(&self) -> &#type1 {
          &self.#name1
        }
      },
      MethodStyle::Parent => quote! {
        fn #getter1(&self) -> &#type1 {
          &self.parent_.#getter2()
        }
      },
    }
  }

  pub fn make_trait_setter(&self) -> TokenStream {
    self.make_setter_(MethodStyle::Trait)
  }

  pub fn make_field_setter(&self) -> TokenStream {
    self.make_setter_(MethodStyle::Field)
  }

  pub fn make_parent_setter(&self) -> TokenStream {
    self.make_setter_(MethodStyle::Parent)
  }

  fn make_setter_(&self, style: MethodStyle) -> TokenStream {
    let setter_name = format!("set_{}", self.name);
    let setter_ident = Ident::new(&setter_name, self.name.span());

    let setter1 = &setter_ident;
    let setter2 = &setter_ident;
    let name1 = &self.name;
    let name2 = &self.name;
    let name3 = &self.name;
    let type1 = &self.ty;

    match style {
      MethodStyle::Trait => quote! {
        fn #setter1(&mut self, #name1: #type1);
      },
      MethodStyle::Field => quote! {
        fn #setter1(&mut self, #name1: #type1) {
          self.#name2 = #name3
        }
      },
      MethodStyle::Parent => quote! {
        fn #setter1(&mut self, #name1: #type1) {
          self.parent_.#setter2(#name2)
        }
      },
    }
  }
}
