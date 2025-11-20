use crate::*;

pub(crate) fn process_enum_derive(input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as DeriveInput);

  let DeriveInput {
    attrs,
    ident: enum_name,
    data,
    ..
  } = tokens;

  let MessageAttrs {
    reserved_names,
    reserved_numbers,
    options,
    name: proto_name,
    file,
    package,
    nested_messages,
    full_name,
    ..
  } = process_message_attrs(&enum_name, &attrs).unwrap();

  let data = if let Data::Enum(enum_data) = data {
    enum_data
  } else {
    panic!("The enum derive can only be used on enums");
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
        #tag,
        EnumVariant { name: #name.to_string(), options: #options, }
      )
    });
  }

  output_tokens.extend(quote! {
    impl ProtoEnumTrait for #enum_name {}

    impl ProtoValidator<#enum_name> for ValidatorMap {
      type Builder = EnumValidatorBuilder;

      fn builder() -> Self::Builder {
        EnumValidator::builder()
      }
    }

    impl AsProtoType for #enum_name {
      fn proto_type() -> ProtoType {
        ProtoType::Single(TypeInfo {
          name: #full_name,
          path: Some(ProtoPath {
            file: #file.into(),
            package: #package.into()
          })
        })
      }
    }

    impl #enum_name {
      pub fn to_enum() -> ProtoEnum {
        ProtoEnum {
          name: #proto_name.into(),
          full_name: #full_name,
          package: #package.into(),
          file: #file.into(),
          variants: vec! [ #(#variants_tokens,)* ],
          reserved_names: #reserved_names,
          reserved_numbers: #reserved_numbers,
          options: #options,
        }
      }
    }
  });

  output_tokens.into()
}
