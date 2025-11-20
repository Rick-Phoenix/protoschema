use crate::*;

pub(crate) struct OneofAttrs {
  pub options: ProtoOptions,
  pub name: String,
  pub reserved_numbers: ReservedNumbers,
  pub required: bool,
}

pub(crate) fn process_oneof_attrs(enum_name: &Ident, attrs: &Vec<Attribute>) -> OneofAttrs {
  let mut options: Option<TokenStream2> = None;
  let mut name: Option<String> = None;
  let mut reserved_numbers = ReservedNumbers::default();
  let mut required = false;

  for attr in attrs {
    if !attr.path().is_ident("proto") {
      continue;
    }

    let args = attr.parse_args::<PunctuatedParser<Meta>>().unwrap();

    for arg in args.inner {
      match arg {
        Meta::Path(path) => {
          if path.is_ident("required") {
            required = true;
          }
        }
        Meta::List(list) => {
          if list.path.is_ident("options") {
            let exprs = list.parse_args::<PunctuatedParser<Expr>>().unwrap().inner;

            options = Some(quote! { vec! [ #exprs ] });
          } else if list.path.is_ident("reserved_numbers") {
            let numbers = list.parse_args::<ReservedNumbers>().unwrap();

            reserved_numbers = numbers;
          }
        }
        Meta::NameValue(nameval) => {
          if nameval.path.is_ident("options") {
            let func_call = nameval.value;

            options = Some(quote! { #func_call });
          } else if nameval.path.is_ident("name") {
            name = Some(extract_string_lit(&nameval.value).unwrap());
          }
        }
      }
    }
  }

  OneofAttrs {
    options: attributes::ProtoOptions(options),
    name: name.unwrap_or_else(|| ccase!(snake, enum_name.to_string())),
    reserved_numbers,
    required,
  }
}
