mod reserved_names;
pub use reserved_names::*;

mod reserved_numbers;
pub use reserved_numbers::*;

use crate::*;

pub(crate) struct MessageAttrs {
  pub reserved_names: ReservedNames,
  pub reserved_numbers: ReservedNumbers,
  pub options: ProtoOptions,
  pub proto_name: String,
  pub file: String,
  pub package: String,
  pub nested_messages: Vec<Path>,
  pub parent_message: Option<Path>,
}

pub(crate) fn process_message_attrs(
  rust_name: &Ident,
  attrs: &Vec<Attribute>,
) -> Result<MessageAttrs, Error> {
  let mut reserved_names = ReservedNames::default();
  let mut reserved_numbers = ReservedNumbers::default();
  let mut options: Option<TokenStream2> = None;
  let mut proto_name: Option<String> = None;
  let mut file: Option<String> = None;
  let mut package: Option<String> = None;
  let mut nested_messages: Vec<Path> = Vec::new();
  let mut parent_message: Option<Path> = None;

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
          } else if list.path.is_ident("nested_messages") {
            let paths = list.parse_args::<PunctuatedParser<Path>>().unwrap().inner;

            nested_messages = paths.into_iter().collect();
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
          } else if nameval.path.is_ident("file") {
            file = Some(extract_string_lit(&nameval.value)?);
          } else if nameval.path.is_ident("package") {
            package = Some(extract_string_lit(&nameval.value)?);
          } else if nameval.path.is_ident("parent_message") {
            if let Expr::Path(expr_path) = nameval.value {
              parent_message = Some(expr_path.path);
            } else {
              panic!("parent_message must be a path");
            }
          }
        }
      }
    }
  }

  let file = file.ok_or(error!(Span::call_site(), "File attribute is missing"))?;
  let package = package.ok_or(error!(Span::call_site(), "Package attribute is missing"))?;

  Ok(MessageAttrs {
    reserved_names,
    reserved_numbers,
    options: attributes::ProtoOptions(options),
    proto_name: proto_name.unwrap_or_else(|| ccase!(pascal, rust_name.to_string())),
    file,
    package,
    nested_messages,
    parent_message,
  })
}
