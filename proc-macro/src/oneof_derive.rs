use crate::*;

pub(crate) fn process_oneof_derive(input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as DeriveInput);
  let enum_name = tokens.ident;

  let mut output_tokens = TokenStream2::new();

  let OneofAttrs {
    options,
    name: proto_name,
    reserved_numbers,
    required,
  } = process_oneof_attrs(&enum_name, &tokens.attrs);

  let data = match tokens.data {
    Data::Enum(enum_data) => enum_data,
    _ => panic!(),
  };

  let mut variants_tokens: Vec<TokenStream2> = Vec::new();

  for variant in data.variants {
    let variant_name = variant.ident;

    let FieldAttrs {
      tag,
      validator,
      options,
      name,
      type_,
      ..
    } = process_field_attrs(&variant_name, &reserved_numbers, &variant.attrs);

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

    let proto_type = if let Some(path) = type_ {
      path
    } else {
      variant_type
    };

    let validator_tokens = if let Some(validator) = validator {
      match validator {
        ValidatorExpr::Call(call) => {
          quote! { Some(<prelude::ValidatorMap as prelude::ProtoValidator<#proto_type>>::from_builder(#call)) }
        }
        ValidatorExpr::Closure(closure) => {
          let validator_type = get_validator_call(&proto_type);

          quote! { Some(<prelude::ValidatorMap as prelude::ProtoValidator<#proto_type>>::build_rules(#closure)) }
        }
      }
    } else {
      quote! { None }
    };

    variants_tokens.push(quote! {
        (#tag, ProtoField {
          name: #name.to_string(),
          options: #options,
          type_: <#proto_type as AsProtoType>::proto_type(),
          validator: #validator_tokens,
        })
    });
  }

  let required_option_tokens = required.then(|| quote! { options.push(oneof_required()); });

  output_tokens.extend(quote! {
    impl #enum_name {
      pub fn to_oneof() -> Oneof {
        let mut options: Vec<ProtoOption> = #options;

        #required_option_tokens

        Oneof {
          name: #proto_name.into(),
          fields: vec! [ #(#variants_tokens,)* ],
          options,
        }
      }
    }
  });

  output_tokens.into()
}
