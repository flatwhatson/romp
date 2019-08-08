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
  ancestors: Vec<Box<Class>>,
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
      ancestors: Vec::new(),
    }
  }
}

impl Class {
  pub fn new(name: Ident, parent: Option<Ident>, fields: Vec<Field>) -> Self {
    let mut this = Self {
      name,
      parent,
      fields,
      ancestors: Vec::new(),
    };
    this.resolve();
    this.register();
    this
  }

  fn register(&self) {
    let mut registry = CLASS_REGISTRY.lock().unwrap();
    let class: RegClass = self.into();
    registry.insert(class.name.clone(), class);
  }

  fn resolve(&mut self) {
    let registry = CLASS_REGISTRY.lock().unwrap();
    let mut current = &self.parent;
    while let Some(parent) = current {
      let lookup = format!("{}", parent);
      if let Some(resolved) = registry.get(&lookup) {
        self.ancestors.push(Box::new(resolved.into()));
        current = &self.ancestors.last().unwrap().parent;
      } else {
        panic!("unable to resolve parent {}", parent);
      }
    }
  }

  pub fn generate(&self) -> TokenStream {
    let struct_def = self.make_struct();
    let struct_impl = self.make_struct_impl();
    let trait_def = self.make_trait();
    let trait_impl = self.make_trait_impl();
    let parent_impls = self.make_parent_impls();

    quote! {
      #struct_def
      #struct_impl
      #trait_def
      #trait_impl
      #parent_impls
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
    let struct_ident = &self.name;
    let trait_name = format!("A{}", self.name);
    let trait_ident = Ident::new(&trait_name, self.name.span());

    let getters = self.fields.iter().map(|f| f.make_field_getter());
    let setters = self.fields.iter().map(|f| f.make_field_setter());

    quote! {
      impl #trait_ident for #struct_ident {
        #( #getters #setters )*
      }
    }
  }

  fn make_parent_impls(&self) -> TokenStream {
    let struct_ident = &self.name;
    let mut output = TokenStream::new();

    for parent in &self.ancestors {
      let trait_name = format!("A{}", parent.name);
      let trait_ident = Ident::new(&trait_name, parent.name.span());

      let getters = parent.fields.iter().map(|f| f.make_parent_getter());
      let setters = parent.fields.iter().map(|f| f.make_parent_setter());

      output = quote! {
        #output
        impl #trait_ident for #struct_ident {
          #( #getters #setters )*
        }
      };
    }

    output
  }

  fn make_struct(&self) -> TokenStream {
    let struct_ident = &self.name;

    let parent = self.ancestors.first().map(|p| {
      let name = &p.name;
      quote!(parent_: #name,)
    });

    let fields = self.fields.iter().map(|f| f.make_definition());

    quote! {
      #[derive(Debug)]
      pub struct #struct_ident {
        #parent #( #fields ),*
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
    let args = self
      .ancestors
      .iter()
      .rev()
      .flat_map(|p| p.fields.iter())
      .chain(self.fields.iter())
      .map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        quote!(#name: #ty)
      });

    let parent = self.ancestors.first().map(|p| {
      let name = &p.name;
      let fields = self
        .ancestors
        .iter()
        .rev()
        .flat_map(|p| p.fields.iter().map(|f| &f.name));

      quote!(parent_: #name::new(#( #fields ),*),)
    });

    let fields = self.fields.iter().map(|f| &f.name);

    quote! {
      pub fn new(#( #args ),*) -> Self {
        Self { #parent #( #fields ),* }
      }
    }
  }
}
