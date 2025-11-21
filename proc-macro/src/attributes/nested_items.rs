use crate::*;

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
