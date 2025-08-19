#[macro_export]
macro_rules! parse_field_type {
  ($ty:ident) => {
    paste! {
      FieldType::[< $ty:camel >]
    }
  };
}

#[macro_export]
macro_rules! msg_field {
  ($msg:ident, $field_name:ident $(, [$($option_name:expr),*])? $(,)? ) => {
    Field::builder()
      .name(stringify!($field_name).into())
      .field_type(FieldType::Message($msg.get_full_name().into()))
      .import(&$msg.get_file())
  };
}

#[macro_export]
macro_rules! field {
  ($field_type:expr, $field_name:ident) => {
    Field::builder()
      .name(stringify!($field_name).into())
      .field_type($field_type)
  };

  ($field_type:expr, $field_name:ident, $validator:expr) => {
    Field::builder()
      .name(stringify!($field_name).into())
      .field_type($field_type)
      .option(vec![$validator])
  };
}

#[macro_export]
macro_rules! string {
  ($field_name:literal, $validator:expr) => {
    Field::builder()
      .name($field_name.into())
      .field_type(parse_field_type!(string))
      .option(build_string_validator_option($validator))
  };

  ($field_name:literal) => {
    Field::builder()
      .name($field_name.into())
      .field_type(parse_field_type!(string))
  };
}

#[macro_export]
macro_rules! message {
  ($file:ident, $name:literal, $($tokens:tt)*) => {
    {
      let msg = $file.new_message($name);
      message_body!(msg, $($tokens)*)
    }
  };
}

#[macro_export]
macro_rules! message_body {
  ($msg_builder:ident, options = $options:expr, $($tokens:tt)*) => {
    $crate::_internal_message_body! {
      @builder($msg_builder)
      @fields()
      @oneofs()
      @enums()
      @input($($tokens)*)
    }.options($options)
  };

  ($msg_builder:ident, $($tokens:tt)*) => {
    $crate::_internal_message_body! {
      @builder($msg_builder)
      @fields()
      @oneofs()
      @enums()
      @input($($tokens)*)
    }
  };
}

#[macro_export]
macro_rules! _internal_message_body {
  // No tokens remaining, process items
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @input()
  ) => {
    {
      { $($enums)* };
      let fields_map = btreemap! { $($fields)* };
      let oneofs_map: Vec<OneofData> = vec! [ $($oneofs)* ];
      $builder.fields(fields_map).oneofs(oneofs_map)
    }
  };

  // Enum with trailing comma
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @input(enum $name:literal { $($tokens:tt)* }, $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs($($oneofs)*)
      @enums($crate::proto_enum!($builder.new_enum($name), $($tokens)*); $($enums)*)
      @input($($rest)*)
    }
  };

  // Enum without trailing comma
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @input(enum $name:literal { $($tokens:tt)* } $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs($($oneofs)*)
      @enums($crate::proto_enum!($builder.new_enum($name), $($tokens)*); $($enums)*)
      @input()
    }
  };

  // Oneof with trailing comma
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @input(oneof $name:literal { $($oneof_body:tt)* }, $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs(
        $($oneofs)*
        $crate::oneof!($builder, $name, $($oneof_body)*),
      )
      @enums($($enums)*)
      @input($($rest)*)
    }
  };

  // Process oneof without trailing comma
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @input(oneof $name:literal { $($oneof_body:tt)* })
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs(
        $($oneofs)*
        $crate::oneof!($builder, $name, $($oneof_body)*)
      )
      @enums($($enums)*)
      @input()
    }
  };


  // Process normal field with trailing comma
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @input($tag:literal => $field:expr, $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* $tag => $field,)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @input($($rest)*)
    }
  };

  // Process normal field
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
              @enums($($enums:tt)*)
    @input($tag:literal => $field:expr)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* $tag => $field)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @input()
    }
  };
}

#[macro_export]
macro_rules! proto_enum {
  (
    $enum:expr,
    options = $options_expr:expr,
    $($tag:literal => $variant:literal),* $(,)?
  ) => {
    {
      $enum
        .options($options_expr)
        .variants(
          btreemap! { $($tag => $variant.to_string()),* }
        )
    }
  };

  (
    $enum:expr,
    $($tag:literal => $variant:literal),* $(,)?
  ) => {
    {
      $enum
        .variants(
          btreemap! { $($tag => $variant.to_string()),* }
        )
    }
  };
}

#[macro_export]
macro_rules! oneof {
  (
    $msg:ident,
    $name:expr,
    options = $options_expr:expr,
    $($tag:literal => $field:expr),* $(,)?
  ) => {
    {
      OneofData::builder()
        .name($name.to_string())
        .parent_message_id($msg.get_id())
        .options($options_expr)
        .fields(
          vec! [ $($field.tag($tag).build()),* ]
        )
        .build()
    }
  };

  (
    $msg:ident,
    $name:expr,
    $($tag:literal => $field:expr),* $(,)?
  ) => {
    {
      OneofData::builder()
        .name($name.to_string())
        .parent_message_id($msg.get_id())
        .fields(
          vec! [ $($field.tag($tag).build()),* ]
        )
        .build()
    }
  };
}
