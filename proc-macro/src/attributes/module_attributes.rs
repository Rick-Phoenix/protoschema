use syn::MetaNameValue;

use crate::*;

pub(crate) struct ModuleAttrs {
  pub file: String,
  pub package: String,
}

impl Parse for ModuleAttrs {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut file: Option<String> = None;
    let mut package: Option<String> = None;

    let args = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

    for arg in args {
      if arg.path.is_ident("file") {
        file = Some(extract_string_lit(&arg.value)?);
      } else if arg.path.is_ident("package") {
        package = Some(extract_string_lit(&arg.value)?);
      }
    }

    let file = file.ok_or(error!(Span::call_site(), "File attribute is missing"))?;
    let package = package.ok_or(error!(Span::call_site(), "Package attribute is missing"))?;

    Ok(ModuleAttrs { file, package })
  }
}

pub fn has_proto_derive(attrs: &[Attribute]) -> Result<bool, Error> {
  for attr in attrs {
    if attr.path().is_ident("derive") {
      let list = attr.meta.require_list()?;

      let derives = list.parse_args::<PunctuatedParser<Path>>()?;

      for path in derives.inner {
        if path.is_ident("Message") {
          return Ok(true);
        }
      }
    }
  }

  Ok(false)
}
