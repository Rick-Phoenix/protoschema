use crate::*;

pub struct MessageAttrs {
  pub reserved_names: ReservedNames,
  pub reserved_numbers: ReservedNumbers,
  pub options: ProtoOptions,
  pub name: String,
  pub full_name: String,
  pub file: String,
  pub package: String,
  pub nested_messages: Option<NestedMessages>,
  pub nested_enums: Option<NestedEnums>,
  pub oneofs: Option<Oneofs>,
}

pub struct NestedEnums {
  pub paths: PunctuatedParser<Path>,
}

impl ToTokens for NestedEnums {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    for path in &self.paths.inner {
      tokens.extend(quote! { #path::to_enum(), });
    }
  }
}

pub struct NestedMessages {
  pub paths: PunctuatedParser<Path>,
}

impl ToTokens for NestedMessages {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    for path in &self.paths.inner {
      tokens.extend(quote! { #path::to_message(), });
    }
  }
}

pub struct Oneofs {
  pub paths: PunctuatedParser<Path>,
}

impl Oneofs {
  pub fn contains(&self, path: &Path) -> bool {
    for oneof_path in &self.paths.inner {
      if oneof_path == path {
        return true;
      }
    }

    false
  }
}

impl ToTokens for Oneofs {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    for path in &self.paths.inner {
      tokens.extend(quote! { #path::to_oneof(), });
    }
  }
}

pub fn process_message_attrs(
  rust_name: &Ident,
  attrs: &Vec<Attribute>,
) -> Result<MessageAttrs, Error> {
  let mut reserved_names = ReservedNames::default();
  let mut reserved_numbers = ReservedNumbers::default();
  let mut options: Option<TokenStream2> = None;
  let mut proto_name: Option<String> = None;
  let mut full_name: Option<String> = None;
  let mut file: Option<String> = None;
  let mut package: Option<String> = None;
  let mut nested_messages: Option<NestedMessages> = None;
  let mut nested_enums: Option<NestedEnums> = None;
  let mut oneofs: Option<Oneofs> = None;

  for attr in attrs {
    if !attr.path().is_ident("proto") {
      continue;
    }

    let args = attr.parse_args::<PunctuatedParser<Meta>>().unwrap();

    for arg in args.inner {
      match arg {
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
            let paths = list.parse_args::<PunctuatedParser<Path>>().unwrap();

            nested_messages = Some(NestedMessages { paths });
          } else if list.path.is_ident("nested_enums") {
            let paths = list.parse_args::<PunctuatedParser<Path>>().unwrap();

            nested_enums = Some(NestedEnums { paths });
          } else if list.path.is_ident("oneofs") {
            let paths = list.parse_args::<PunctuatedParser<Path>>().unwrap();

            oneofs = Some(Oneofs { paths });
          }
        }
        Meta::NameValue(nameval) => {
          if nameval.path.is_ident("options") {
            let func_call = nameval.value;

            options = Some(quote! { #func_call });
          } else if nameval.path.is_ident("name") {
            proto_name = Some(extract_string_lit(&nameval.value).unwrap());
          } else if nameval.path.is_ident("full_name") {
            full_name = Some(extract_string_lit(&nameval.value).unwrap());
          } else if nameval.path.is_ident("reserved_names") {
            reserved_names = ReservedNames::Expr(nameval.value);
          } else if nameval.path.is_ident("file") {
            file = Some(extract_string_lit(&nameval.value)?);
          } else if nameval.path.is_ident("package") {
            package = Some(extract_string_lit(&nameval.value)?);
          }
        }
        Meta::Path(_) => {}
      }
    }
  }

  let file = file.ok_or(error!(Span::call_site(), "File attribute is missing"))?;
  let package = package.ok_or(error!(Span::call_site(), "Package attribute is missing"))?;

  let name = proto_name.unwrap_or_else(|| ccase!(pascal, rust_name.to_string()));

  Ok(MessageAttrs {
    reserved_names,
    reserved_numbers,
    options: attributes::ProtoOptions(options),
    full_name: full_name.unwrap_or_else(|| name.clone()),
    name,
    file,
    package,
    nested_messages,
    nested_enums,
    oneofs,
  })
}
