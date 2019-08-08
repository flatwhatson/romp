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
    let struct_def = self.make_struct();
    let trait_def = self.make_trait();
    let struct_impl = self.make_struct_impl();
    let trait_impl = self.make_trait_impl();
    let parent_impl = self.make_parent_impl();

    quote! {
      #struct_def
      #trait_def
      #struct_impl
      #trait_impl
      #parent_impl
    }
  }

  fn make_trait(&self) -> TokenStream {
    let trait_name = format!("A{}", self.name);
    let trait_ident = Ident::new(&trait_name, self.name.span());

    let getters = self.fields.iter().map(|f| f.make_trait_getter());
    let setters = self.fields.iter().map(|f| f.make_trait_setter());

    quote! {
      pub trait #trait_ident {
        #( #getters #setters )*
      }
    }
  }

  fn make_trait_impl(&self) -> TokenStream {
    let trait_name = format!("A{}", self.name);
    let trait_ident = Ident::new(&trait_name, self.name.span());
    let struct_ident = &self.name;

    let getters = self.fields.iter().map(|f| f.make_field_getter());
    let setters = self.fields.iter().map(|f| f.make_field_setter());

    quote! {
      impl #trait_ident for #struct_ident {
        #( #getters #setters )*
      }
    }
  }

  fn make_parent_impl(&self) -> Option<TokenStream> {
    let parent = self.resolved_parent.as_ref()?;

    let parent_name = format!("A{}", parent.name);
    let parent_ident = Ident::new(&parent_name, parent.name.span());
    let struct_ident = &self.name;

    let getters = parent.fields.iter().map(|f| f.make_parent_getter());
    let setters = parent.fields.iter().map(|f| f.make_parent_setter());

    Some(quote! {
      impl #parent_ident for #struct_ident {
        #( #getters #setters )*
      }
    })
  }

  fn make_struct(&self) -> TokenStream {
    let struct_ident = &self.name;
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
      pub struct #struct_ident {
        #(#fields),*
      }
    }
  }

  fn make_struct_impl(&self) -> TokenStream {
    let struct_ident = &self.name;
    let constructor = self.make_constructor();

    quote! {
      impl #struct_ident {
        #constructor
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

      self_args.push(quote! {
        parent_: #parent_name::new(#(#parent_args),*)
      });
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
