mod reserved_names;
pub use reserved_names::*;

mod reserved_numbers;
pub use reserved_numbers::*;

use crate::*;

pub(crate) struct ContainerAttrs {
  pub reserved_names: ReservedNames,
  pub reserved_numbers: ReservedNumbers,
  pub options: Options,
  pub proto_name: String,
}

pub(crate) fn process_container_attr(rust_name: &Ident, attrs: &Vec<Attribute>) -> ContainerAttrs {
  let mut reserved_names = ReservedNames::default();
  let mut reserved_numbers = ReservedNumbers::default();
  let mut options: Option<TokenStream2> = None;
  let mut proto_name: Option<String> = None;

  for attr in attrs {
    if !attr.path().is_ident("proto") {
      continue;
    }

    let args = attr.parse_args::<PunctuatedParser<Meta>>().unwrap();

    for arg in args.inner {
      match arg {
        Meta::Path(path) => todo!(),
        Meta::List(list) => {
          if list.path.is_ident("reserved_names") {
            let names = list.parse_args::<StringList>().unwrap();

            reserved_names = ReservedNames::List(names.list);
          } else if list.path.is_ident("reserved_numbers") {
            let numbers = list.parse_args::<ReservedNumbers>().unwrap();

            reserved_numbers = numbers;
          } else if list.path.is_ident("options") {
            let exprs = list.parse_args::<PunctuatedParser<Expr>>().unwrap().inner;

            options = Some(quote! { vec! [ #exprs ] });
          }
        }
        Meta::NameValue(nameval) => {
          if nameval.path.is_ident("options") {
            let func_call = nameval.value;

            options = Some(quote! { #func_call });
          } else if nameval.path.is_ident("name") {
            proto_name = Some(extract_string_lit(&nameval.value).unwrap());
          } else if nameval.path.is_ident("reserved_names") {
            reserved_names = ReservedNames::Expr(nameval.value);
          }
        }
      }
    }
  }

  ContainerAttrs {
    reserved_names,
    reserved_numbers,
    options: attributes::Options(options),
    proto_name: proto_name.unwrap_or_else(|| ccase!(pascal, rust_name.to_string())),
  }
}
