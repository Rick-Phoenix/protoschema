use crate::*;

pub struct PunctuatedParser<T: Parse> {
  pub inner: Punctuated<T, Token![,]>,
}

impl<T: Parse> Parse for PunctuatedParser<T> {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let inner = Punctuated::parse_terminated(input)?;

    Ok(Self { inner })
  }
}

pub fn extract_u32(expr: &Expr) -> Result<u32, Error> {
  if let Expr::Lit(expr_lit) = expr && let Lit::Int(value) = &expr_lit.lit {
    Ok(value.base10_parse()?)
  } else {
    Err(spanned_error!(expr, "Expected an integer literal"))
  }
}

pub(crate) struct ProtoOptions(pub Option<TokenStream2>);

impl ToTokens for ProtoOptions {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    tokens.extend(if let Some(opts) = &self.0 {
      quote! { #opts }
    } else {
      quote! { vec![] }
    });
  }
}

pub struct StringList {
  pub list: Vec<String>,
}

impl Parse for StringList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let items = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;

    let list: Vec<String> = items.into_iter().map(|lit_str| lit_str.value()).collect();

    Ok(Self { list })
  }
}

pub fn extract_string_lit(expr: &Expr) -> Result<String, Error> {
  if let Expr::Lit(expr_lit) = expr && let Lit::Str(value) = &expr_lit.lit {
    Ok(value.value())
  } else {
    Err(spanned_error!(expr, "Expected a string literal"))
  }
}
