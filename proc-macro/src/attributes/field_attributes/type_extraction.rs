use crate::*;

const PRIMITIVES: [(&str, &str); 7] = [
  ("String", "string"),
  ("u8", "bytes"),
  ("u32", "uint32"),
  ("u64", "uint64"),
  ("i32", "int32"),
  ("i64", "int64"),
  ("bool", "bool"),
];

pub fn get_validator_call(type_path: &Path) -> TokenStream2 {
  let last_segment = type_path.segments.last().unwrap();

  last_segment.to_token_stream()

  // if let Some(rust_type) = PRIMITIVES.iter().find_map(|(rust_type, proto_type)| {
  //   if last_segment.ident == rust_type {
  //     Some(&**proto_type)
  //   } else {
  //     None
  //   }
  // }) {
  //   todo!();
  // } else {
  //   last_segment.to_token_stream()
  // }
}

pub fn extract_known_type(path: &Path) -> Option<&'static str> {
  let last_segment = path.segments.last().unwrap();

  PRIMITIVES.iter().find_map(|(rust_type, proto_type)| {
    if last_segment.ident == rust_type {
      Some(&**proto_type)
    } else {
      None
    }
  })
}

pub enum ProtoType {
  Literal(String),
  Path(Path),
}

impl ToTokens for ProtoType {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    tokens.extend(match self {
      Self::Path(path) => quote! { <#path as ProtoType>::proto_type() },
      Self::Literal(literal) => quote! { #literal.to_string() },
    });
  }
}
