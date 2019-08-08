use super::field::{Field, RegField};
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::sync::Mutex;
use syn::{parse_str, Ident};

lazy_static! {
  static ref CLASS_REGISTRY: Mutex<HashMap<String, RegClass>> = { Mutex::new(HashMap::new()) };
}

pub struct Class {
  pub name: Ident,
  pub parent: Option<Ident>,
  pub fields: Vec<Field>,
  resolved_parent: Option<Box<Class>>,
}

#[derive(Clone, Debug)]
pub struct RegClass {
  pub name: String,
  pub parent: Option<String>,
  pub fields: Vec<RegField>,
}

impl From<&Class> for RegClass {
  fn from(class: &Class) -> RegClass {
    RegClass {
      name: format!("{}", class.name),
      parent: class.parent.as_ref().map(|p| format!("{}", p)),
      fields: class.fields.iter().map(|f| f.into()).collect(),
    }
  }
}

impl From<&RegClass> for Class {
  fn from(class: &RegClass) -> Class {
    Class {
      name: parse_str(&class.name).unwrap(),
      parent: class.parent.as_ref().map(|p| parse_str(p).unwrap()),
      fields: class.fields.iter().map(|f| f.into()).collect(),
      resolved_parent: None,
    }
  }
}

impl Class {
  pub fn new(name: Ident, parent: Option<Ident>, fields: Vec<Field>) -> Self {
    let mut this = Self {
      name,
      parent,
      fields,
      resolved_parent: None,
    };
    this.resolve();
    this.register();
    this
  }

  fn register(&self) {
    let class: RegClass = self.into();
    let name = format!("{}", self.name);

    let mut registry = CLASS_REGISTRY.lock().unwrap();
    registry.insert(name, class);
  }

  fn resolve(&mut self) {
    if let Some(parent) = &self.parent {
      let registry = CLASS_REGISTRY.lock().unwrap();

      let lookup = format!("{}", parent);
      if let Some(resolved) = registry.get(&lookup) {
        self.resolved_parent = Some(Box::new(resolved.into()));
        return;
      }

      panic!("unable to resolve parent {}", parent);
    }
  }

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
    let mut fields = Vec::new();

    if let Some(parent) = self.resolved_parent.as_ref() {
      let name = &parent.name;
      fields.push(quote!(parent_: #name));
    }

    for field in &self.fields {
      fields.push(field.make_definition());
    }

    quote! {
      #[derive(Debug)]
      pub struct #name {
        #(#fields),*
      }
    }
  }

  fn make_implementation(&self) -> TokenStream {
    let name = &self.name;
    let mut methods = Vec::new();

    methods.push(self.make_constructor());

    if let Some(parent) = self.resolved_parent.as_ref() {
      for field in &parent.fields {
        methods.push(field.make_parent_getter());
        methods.push(field.make_parent_setter());
      }
    }

    for field in &self.fields {
      methods.push(field.make_getter());
      methods.push(field.make_setter());
    }

    quote! {
      impl #name {
        #(#methods)*
      }
    }
  }

  fn make_constructor(&self) -> TokenStream {
    let mut func_args = Vec::new();
    let mut self_args = Vec::new();

    if let Some(parent) = self.resolved_parent.as_ref() {
      let parent_name = &parent.name;
      let mut parent_args = Vec::new();

      for Field { name, ty } in &parent.fields {
        func_args.push(quote!(#name: #ty));
        parent_args.push(quote!(#name));
      }

      self_args.push(quote!(parent_: #parent_name { #(#parent_args),* }));
    }

    for Field { name, ty } in &self.fields {
      func_args.push(quote!(#name: #ty));
      self_args.push(quote!(#name));
    }

    quote! {
      pub fn new(#(#func_args),*) -> Self {
        Self { #(#self_args),* }
      }
    }
  }
}
