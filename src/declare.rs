use super::class::Class;
use super::field::Field;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Result, Token};

mod kw {
  syn::custom_keyword!(class);
  syn::custom_keyword!(extends);
}

pub struct Declarations {
  pub classes: Vec<Class>,
}

impl Parse for Declarations {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut classes = Vec::new();

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(kw::class) {
        classes.push(parse_class(input)?);
      } else {
        return Err(lookahead.error());
      }
    }

    Ok(Self { classes })
  }
}

fn parse_class(input: ParseStream) -> Result<Class> {
  input.parse::<kw::class>()?;

  let name = input.parse()?;

  let parent = if input.peek(kw::extends) {
    input.parse::<kw::extends>()?;
    Some(input.parse()?)
  } else {
    None
  };

  let fields = parse_fields(input)?;

  Ok(Class::new(name, parent, fields))
}

fn parse_fields(input: ParseStream) -> Result<Vec<Field>> {
  let block;
  braced!(block in input);
  let mut fields = Vec::new();

  loop {
    if block.is_empty() {
      break;
    }

    fields.push(parse_field(&block)?);

    if block.is_empty() {
      break;
    }

    block.parse::<Token![,]>()?;
  }

  Ok(fields)
}

fn parse_field(input: ParseStream) -> Result<Field> {
  let name = input.parse()?;
  input.parse::<Token![:]>()?;
  let ty = input.parse()?;

  Ok(Field { name, ty })
}
