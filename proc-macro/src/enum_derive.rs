use crate::*;

pub(crate) fn process_enum_derive(input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as DeriveInput);

  let DeriveInput {
    attrs,
    ident: enum_name,
    data,
    ..
  } = tokens;

  let ContainerAttrs {
    reserved_names,
    reserved_numbers,
    options,
    proto_name,
    file,
    package,
  } = process_container_attr(&enum_name, &attrs).unwrap();

  let data = if let Data::Enum(enum_data) = data {
    enum_data
  } else {
    panic!()
  };

  let mut output_tokens = TokenStream2::new();

  let mut fields_data: Vec<TokenStream2> = Vec::new();

  let mut variants_tokens: Vec<TokenStream2> = Vec::new();

  for variant in data.variants {
    if !variant.fields.is_empty() {
      panic!("Must be a unit variant");
    }

    let variant_name = variant.ident;

    let EnumVariantAttrs { tag, options, name } =
      process_enum_variants_attrs(&variant_name, &reserved_numbers, &variant.attrs);

    variants_tokens.push(quote! {
      (
        1,
        EnumVariant { name: #name.to_string(), options: #options, }
      )
    });
  }

  output_tokens.extend(quote! {
    impl #enum_name {
      pub fn to_proto_enum(file: &mut ProtoFile) -> ProtoEnum {
        let path = file.path();

        ProtoEnum {
          name: #proto_name.into(),
          package: path.package,
          file: path.file,
          variants: vec! [ #(#variants_tokens,)* ],
          reserved_names: #reserved_names,
          reserved_numbers: #reserved_numbers,
          options: #options,
          ..Default::default()
        }
      }

      pub fn to_nested_enum(message: &mut Message) ->  &mut ProtoEnum {
        let mut new_enum = ProtoEnum {
          name: #proto_name.into(),
          package: message.package.clone(),
          file: message.file.clone(),
          variants: vec! [ #(#variants_tokens,)* ],
          reserved_names: #reserved_names,
          reserved_numbers: #reserved_numbers,
          options: #options,
          ..Default::default()
        };

        message.nested_enum(new_enum)
      }
    }
  });

  output_tokens.into()
}
