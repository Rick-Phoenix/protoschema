use crate::*;

pub(crate) struct EnumVariantAttrs {
  pub name: String,
  pub tag: u32,
  pub options: Options,
}

pub(crate) fn process_enum_variants_attrs(
  original_name: &Ident,
  reserved_numbers: &ReservedNumbers,
  attrs: &Vec<Attribute>,
) -> EnumVariantAttrs {
  let mut tag: Option<u32> = None;
  let mut options: Option<TokenStream2> = None;
  let mut name: Option<String> = None;

  let mut incr_counter: u32 = 1;

  for attr in attrs {
    if !attr.path().is_ident("proto") {
      continue;
    }

    let args = attr.parse_args::<PunctuatedParser<Meta>>().unwrap();

    // eprintln!("{:#?}", args.inner);

    for meta in args.inner {
      match meta {
        Meta::NameValue(nameval) => {
          if nameval.path.is_ident("tag") {
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

  EnumVariantAttrs {
    tag,
    options: attributes::Options(options),
    name: name.unwrap_or_else(|| ccase!(constant, original_name.to_string())),
  }
}
