use crate::*;

pub(crate) fn process_oneof_derive(tokens: DeriveInput) -> Result<TokenStream2, Error> {
  let enum_name = tokens.ident;

  let mut output_tokens = TokenStream2::new();

  let OneofAttrs {
    options,
    name: proto_name,
    required,
  } = process_oneof_attrs(&enum_name, &tokens.attrs);

  let data = match tokens.data {
    Data::Enum(enum_data) => enum_data,
    _ => panic!(),
  };

  let mut variants_tokens: Vec<TokenStream2> = Vec::new();

  for variant in data.variants {
    let variant_name = variant.ident;

    let field_attrs = if let Some(attrs) = process_field_attrs(&variant_name, &variant.attrs)? {
      attrs
    } else {
      continue;
    };

    let FieldAttrs {
      tag,
      validator,
      options,
      name,
      type_,
    } = field_attrs;

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
          quote! { Some(<ValidatorMap as ProtoValidator<#proto_type>>::from_builder(#call)) }
        }
        ValidatorExpr::Closure(closure) => {
          quote! { Some(<ValidatorMap as ProtoValidator<#proto_type>>::build_rules(#closure)) }
        }
      }
    } else {
      quote! { None }
    };

    let tag_tokens = OptionTokens::new(tag.as_ref());

    variants_tokens.push(quote! {
      ProtoField {
        name: #name.to_string(),
        options: #options,
        type_: <#proto_type as AsProtoType>::proto_type(),
        validator: #validator_tokens,
        tag: tag_allocator.get_or_next(#tag_tokens),
      }
    });
  }

  let required_option_tokens = required.then(|| quote! { options.push(oneof_required()); });

  output_tokens.extend(quote! {
    impl ProtoOneof for #enum_name {
      fn fields(tag_allocator: &mut TagAllocator) -> Vec<ProtoField> {
        vec![ #(#variants_tokens,)* ]
      }
    }

    impl #enum_name {
      #[track_caller]
      pub fn to_oneof(tag_allocator: &mut TagAllocator) -> Oneof {
        let mut options: Vec<ProtoOption> = #options;

        #required_option_tokens

        Oneof {
          name: #proto_name.into(),
          fields: Self::fields(tag_allocator),
          options,
        }
      }
    }
  });

  Ok(output_tokens)
}
