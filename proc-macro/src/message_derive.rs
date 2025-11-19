use crate::*;

pub(crate) fn process_message_derive(input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as DeriveInput);

  let DeriveInput {
    attrs,
    ident: struct_name,
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
  } = process_container_attr(&struct_name, &attrs).unwrap();

  let data = if let Data::Struct(struct_data) = data {
    struct_data
  } else {
    panic!()
  };

  let fields = if let Fields::Named(fields) = data.fields {
    fields.named
  } else {
    panic!()
  };

  let mut output_tokens = TokenStream2::new();

  let mut fields_data: Vec<TokenStream2> = Vec::new();

  for field in fields {
    let field_name = field.ident.as_ref().expect("Expected named field");

    let FieldAttrs {
      tag,
      validator,
      options,
      name,
      type_,
    } = process_field_attrs(field_name, &reserved_numbers, &field.attrs);

    let mut is_repeated = false;
    let mut is_optional = false;

    let field_type = match field.ty {
      Type::Path(type_path) => type_path.path,

      _ => panic!("Must be a path type"),
    };

    let proto_type = if let Some(type_data) = type_ {
      type_data
    } else {
      field_type
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

    fields_data.push(quote! {
      (#tag, ProtoField {
        name: #name.to_string(),
        options: #options,
        type_: <#proto_type as ProtoType>::type_name(),
        validator: #validator_tokens,
      })
    });
  }

  output_tokens.extend(quote! {
    impl ProtoMessage for #struct_name {}

    impl ProtoType for #struct_name {
      fn type_name() -> Arc<str> {
        #proto_name.into()
      }

      fn import_path() -> Option<ProtoPath> {
        Some(ProtoPath {
          file: #file.into(),
          package: #package.into()
        })
      }
    }

    impl #struct_name {
      pub fn to_message(file: &mut ProtoFile) -> &mut Message {
        let path = file.path();

        let mut new_msg = Message {
          name: #proto_name.into(),
          package: path.package,
          file: path.file,
          fields: vec![ #(#fields_data,)* ],
          reserved_names: #reserved_names,
          reserved_numbers: #reserved_numbers,
          options: #options,
          ..Default::default()
        };

        file.add_message(new_msg)
      }

    }
  });

  output_tokens.into()
}
