use crate::*;

#[derive(Default)]
pub(crate) struct ReservedNumbers(pub Vec<Range<u32>>);

impl ReservedNumbers {
  pub fn contains(&self, number: &u32) -> bool {
    let result = self.0.binary_search_by(|range| {
      if range.contains(number) {
        Ordering::Equal
      } else if *number < range.start {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    });

    result.is_ok()
  }
}

impl ToTokens for ReservedNumbers {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let mut agg_tokens = TokenStream2::new();

    for range in &self.0 {
      let start = range.start;
      let end = range.end;

      agg_tokens.extend(quote! {
        #start..#end,
      });
    }

    tokens.extend(quote! {
      vec![ #agg_tokens ]
    });
  }
}

impl Parse for ReservedNumbers {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut ranges: Vec<Range<u32>> = Vec::new();

    let items = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

    for item in items {
      if let Expr::Range(range_expr) = &item {
        let start = if let Some(start_expr) = &range_expr.start {
          extract_u32(start_expr)?
        } else {
          0
        };

        let end = if let Some(end_expr) = &range_expr.end {
          extract_u32(end_expr)?
        } else {
          return Err(spanned_error!(
            range_expr,
            "Infinite range is not supported"
          ));
        };

        let final_end = if let RangeLimits::HalfOpen(_) = &range_expr.limits {
          end
        } else {
          end + 1
        };

        ranges.push(start..final_end);
      } else if let Expr::Lit(lit) = &item && let Lit::Int(lit_int) = &lit.lit {
        let num = lit_int.base10_parse::<u32>()?;

        ranges.push(num..num + 1);
      } else {
        return Err(spanned_error!(
          item,
          "Expected a range (e.g. `1..5`, `10..=15`)"
        ));
      }
    }

    ranges.sort_by_key(|range| range.start);

    Ok(Self(ranges))
  }
}
