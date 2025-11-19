use crate::*;

pub(crate) fn process_oneof_derive(input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as DeriveInput);
  let enum_name = tokens.ident;

  let mut output_tokens = TokenStream2::new();

  let OneofAttrs {
    options,
    name: proto_name,
    reserved_numbers,
  } = process_oneof_attrs(&enum_name, &tokens.attrs);

  let data = match tokens.data {
    Data::Enum(enum_data) => enum_data,
    _ => panic!(),
  };

  let mut variants_tokens: Vec<TokenStream2> = Vec::new();

  for variant in data.variants {
    let variant_type = if let Fields::Unnamed(variant_fields) = variant.fields {
      if variant_fields.unnamed.len() != 1 {
        panic!("Oneof variants must contain a single value");
      }

      match variant_fields.unnamed.first().unwrap().ty.clone() {
        Type::Path(type_path) => type_path.path,

        _ => panic!("Must be a path type"),
      }
    } else {
      panic!("Enum can only have one unnamed field")
    };

    let variant_name = variant.ident;

    let FieldAttrs {
      tag,
      validator,
      options,
      name,
      type_,
    } = process_field_attrs(&variant_name, &reserved_numbers, &variant.attrs);

    // let proto_type = if let Some(path) = type_ {
    //   path
    // } else if let Some(literal) = extract_known_type(&variant_type) {
    //   ProtoType::Literal(literal.to_string())
    // } else {
    //   panic!("No type defined")
    // };

    variants_tokens.push(quote! {
        (#tag, ProtoField {
          name: stringify!(#variant_name).to_string(),
          options: #options,
          type_: "to implement...".to_string(),
          validator: None,
        })
    });
  }

  output_tokens.extend(quote! {
    impl #enum_name {
      pub fn to_oneof(message: &mut Message) -> Oneof {
        Oneof {
          name: #proto_name.into(),
          fields: vec! [ #(#variants_tokens,)* ],
          options: #options,
          ..Default::default()
        }
      }
    }
  });

  output_tokens.into()
}
