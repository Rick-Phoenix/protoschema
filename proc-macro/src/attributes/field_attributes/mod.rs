mod type_extraction;

use std::{cmp::Ordering, ops::Range};

use convert_case::ccase;
use quote::ToTokens;
use syn::{token::Token, ExprCall, Path, TypePath};
pub use type_extraction::*;

use crate::*;

pub(crate) struct FieldAttrs {
  pub tag: u32,
  pub validator: Option<ValidatorExpr>,
  pub options: Options,
  pub name: String,
  pub type_: Option<Path>,
}

pub(crate) enum ValidatorExpr {
  Closure(ExprClosure),
  Call(ExprCall),
}

pub(crate) fn process_field_attrs(
  original_name: &Ident,
  reserved_numbers: &ReservedNumbers,
  attrs: &Vec<Attribute>,
) -> FieldAttrs {
  let mut validator: Option<ValidatorExpr> = None;
  let mut tag: Option<u32> = None;
  let mut options: Option<TokenStream2> = None;
  let mut name: Option<String> = None;
  let mut type_: Option<Path> = None;

  let mut incr_counter: u32 = 1;

  for attr in attrs {
    if !attr.path().is_ident("proto") {
      continue;
    }

    let args = attr.parse_args::<PunctuatedParser<Meta>>().unwrap();

    for meta in args.inner {
      match meta {
        Meta::NameValue(nameval) => {
          if nameval.path.is_ident("validate") {
            if let Expr::Closure(closure) = nameval.value {
              validator = Some(ValidatorExpr::Closure(closure));
            } else if let Expr::Call(call) = nameval.value {
              validator = Some(ValidatorExpr::Call(call));
            } else {
              panic!("Invalid");
            }
          } else if nameval.path.is_ident("tag") {
            tag = Some(extract_u32(&nameval.value).unwrap());
          } else if nameval.path.is_ident("options") {
            let func_call = nameval.value;

            options = Some(quote! { #func_call });
          } else if nameval.path.is_ident("name") {
            name = Some(extract_string_lit(&nameval.value).unwrap());
          }
        }
        Meta::Path(path) => {}
        Meta::List(list) => {
          if list.path.is_ident("options") {
            let exprs = list.parse_args::<PunctuatedParser<Expr>>().unwrap().inner;

            options = Some(quote! { vec! [ #exprs ] });
          } else if list.path.is_ident("type_") {
            type_ = Some(list.parse_args::<TypePath>().unwrap().path);
          }
        }

        _ => {}
      };
    }
  }

  let tag = tag.unwrap_or_else(|| {
    while reserved_numbers.contains(&incr_counter) {
      incr_counter += 1;
    }

    let found = incr_counter;

    incr_counter += 1;

    found
  });

  FieldAttrs {
    validator,
    tag,
    options: attributes::Options(options),
    name: name.unwrap_or_else(|| ccase!(snake, original_name.to_string())),
    type_,
  }
}
