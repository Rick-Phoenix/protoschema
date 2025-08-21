#[crabtime::function]
#[macro_export]
fn get_map_func_path(pattern!($key_type:ident, $value_type:ident): _) {
  let ty = expand!($crate::parse_field_type!($value_type));
  match ty {
    FieldType::Enum(_) => {
      crabtime::output!($crate::validators::map::build_map_{{key_type}}_keys_enum_values_validator)
    }
    FieldType::Message(_) => {
      crabtime::output!($crate::validators::map::build_map_{{key_type}}_keys_message_values_validator)
    }
  }
}

#[macro_export]
macro_rules! parse_field_type {
  (enum($ty:ident)) => {
    $crate::FieldType::from($ty.get_type())
  };

  ($ty:ident) => {
    $crate::FieldType::from($ty.get_type())
  };

  ($ty:expr) => {
    $crate::FieldType::from($ty)
  };
}

#[macro_export]
macro_rules! msg_field {
  ($msg:expr, $field_name:literal $(, [$($option_name:expr),*])? $(,)? ) => {
    $crate::fields::Field::builder()
      .name($field_name.into())
      .field_type($crate::FieldType::Message($msg.get_full_name().into()))
      .add_import(&$msg.get_file())
  };
}

#[macro_export]
macro_rules! get_func_path {
  (repeated $proto_type:ident) => {
    paste! { $crate::validators::repeated::[< build_repeated_ $proto_type _validator_option >] }
  };

  ($proto_type:ident, $path:path) => {
    paste! { $crate::validators::$path::[< build_ $proto_type _validator_option >] }
  };

  ($proto_type:ident) => {
    paste! { $crate::validators::$proto_type::[< build_ $proto_type _validator_option >] }
  };
}

macro_rules! proto_field {
  ($proto_type:ident, $path:path) => {
    proto_field_inner!(
      $proto_type,
      paste! { $crate::validators::$path::[< build_ $proto_type _validator_option >] },
      paste! { $crate::validators::repeated::[< build_repeated_ $proto_type _validator_option >] }
    );
  };

  ($proto_type:ident) => {
    proto_field_inner!(
      $proto_type,
      paste! { $crate::validators::$proto_type::[< build_ $proto_type _validator_option >] },
      paste! { $crate::validators::repeated::[< build_repeated_ $proto_type _validator_option >] }
    );
  };
}

macro_rules! proto_field_inner {
  ($proto_type:ident, $validator_func:expr, $repeated_validator:expr) => {
    #[macro_export]
    macro_rules! $proto_type {
      ($field_name:literal, $validator:expr) => {
        $crate::fields::Field::builder()
          .name($field_name.into())
          .field_type($crate::parse_field_type!(stringify!($proto_type)))
          .option($validator_func($validator))
          .add_import("buf/validate/validate.proto")
      };

      (repeated $field_name:literal, $validator:expr) => {
        $crate::fields::Field::builder()
          .name($field_name.into())
          .repeated()
          .field_type($crate::parse_field_type!(stringify!($proto_type)))
          .option($repeated_validator($validator))
          .add_import("buf/validate/validate.proto")
      };

      ($field_name:literal) => {
        $crate::fields::Field::builder()
          .name($field_name.into())
          .field_type($crate::parse_field_type!(stringify!($proto_type)))
      };
    }
  };
}

#[macro_export]
macro_rules! map {
  ($name:literal, <$key_type:ident, $value_type:ident>) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Map(
        $crate::MapKey:: [<  $key_type:camel >],
        Box::new($crate::parse_field_type!($value_type))
      ))
    }
  };

  ($name:literal, <$key_type:ident, $value_type:ident>, $validator:expr) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Map(
        $crate::MapKey:: [<  $key_type:camel >],
        Box::new($crate::parse_field_type!($value_type))
      ))
      .option($crate::validators::map:: [< build_map_ $key_type _keys_ $value_type _values_ validator >]($validator))
      .add_import("buf/validate/validate.proto")
    }
  };
}

#[macro_export]
macro_rules! enum_map {
  ($name:literal, <$key_type:ident, $value_type:ident>) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Map(
        $crate::MapKey:: [<  $key_type:camel >],
        Box::new($crate::parse_field_type!($value_type))
      ))
    }
  };

  ($name:literal, <$key_type:ident, $value_type:ident>, $validator:expr) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Map(
        $crate::MapKey:: [<  $key_type:camel >],
        Box::new($crate::parse_field_type!($value_type))
      ))
      .option($crate::validators::map:: [< build_map_ $key_type _keys_ enum _values_ validator >]($validator))
      .add_import("buf/validate/validate.proto")
    }
  };
}

#[macro_export]
macro_rules! enum_field {
  (repeated $enum_ident:ident, $name:literal) => {
    $crate::fields::Field::builder()
      .name($name.into())
      .repeated()
      .field_type($crate::FieldType::Enum($enum_ident.get_full_name().into()))
      .add_import(&$enum_ident.get_file())
  };

  (repeated $enum_ident:ident, $name:literal, $validator:expr) => {
    $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Enum($enum_ident.get_full_name().into()))
      .add_import(&$enum_ident.get_file())
      .option($crate::validators::repeated::build_repeated_enum_validator_option($validator))
      .add_import("buf/validate/validate.proto")
  };

  ($enum_ident:ident, $name:literal) => {
    $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Enum($enum_ident.get_full_name().into()))
      .add_import(&$enum_ident.get_file())
  };

  ($enum_ident:ident, $name:literal, $validator:expr) => {
    $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Enum($enum_ident.get_full_name().into()))
      .add_import(&$enum_ident.get_file())
      .option($crate::validators::enums::build_enum_validator_option(
        $validator,
      ))
      .add_import("buf/validate/validate.proto")
  };
}

proto_field!(string);
proto_field!(any);
proto_field!(duration);
proto_field!(timestamp);
proto_field!(bytes);
proto_field!(bool);
proto_field!(int64, numeric);
proto_field!(int32, numeric);
proto_field!(sint64, numeric);
proto_field!(sint32, numeric);
proto_field!(sfixed64, numeric);
proto_field!(sfixed32, numeric);
proto_field!(uint64, numeric);
proto_field!(uint32, numeric);
proto_field!(fixed64, numeric);
proto_field!(fixed32, numeric);
proto_field!(double, numeric);
proto_field!(float, numeric);

#[macro_export]
macro_rules! message_body {
  ($msg_builder:expr, options = $options:expr, $($tokens:tt)*) => {
    $crate::_internal_message_body! {
      @builder($msg_builder)
      @fields()
      @oneofs()
      @enums()
      @reserved()
      @reserved_names()
      @input($($tokens)*)
    }.options($options.as_slice())
  };

  ($msg_builder:expr, $($tokens:tt)*) => {
    $crate::_internal_message_body! {
      @builder($msg_builder)
      @fields()
      @oneofs()
      @enums()
      @reserved()
      @reserved_names()
      @input($($tokens)*)
    }
  };
}

#[macro_export]
macro_rules! _internal_message_body {
  // No tokens remaining, process items
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($names:expr)?)
    @input($(,)?)
  ) => {
    {
      { $($enums)* };

      let fields_list = &[ $($fields)* ];
      let oneofs_list = &[ $($oneofs)* ];

      let mut new_msg = $builder
        .fields(fields_list)
        .oneofs(oneofs_list)
      $(
        .reserved_names($names.as_slice())
      )?;

      $crate::parse_reserved! {
        @builder(new_msg)
        @ranges()
        @numbers()
        @rest($($reserved)*)
      }
    }
  };

  // Reserved numbers
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved()
    @reserved_names($($reserved_names:tt)*)
    @input($(,)? reserved = [ $($items:tt)* ] $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($items)*)
      @reserved_names($($reserved_names)*)
      @input($($rest)*)
    }
  };

  // Reserved names
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names()
    // Expr cannot be followed by tt so there must be a comma right after
    @input($(,)? reserved_names = $reserved_names:expr, $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($reserved_names)
      @input($($rest)*)
    }
  };

  // Handle enum
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @input($(,)? enum $name:literal { $($tokens:tt)* } $($rest:tt)* )
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs($($oneofs)*)
      @enums($crate::proto_enum!($builder.new_enum($name), $($tokens)*); $($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @input($($rest)*)
    }
  };

  // Handle oneof
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @input($(,)? oneof $name:literal { $($oneof_body:tt)* } $($rest:tt)* )
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs(
        $($oneofs)*
        $crate::oneof!($builder, $name, $($oneof_body)*),
      )
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @input($($rest)*)
    }
  };

  // Process normal field with trailing comma
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @input($(,)? $tag:literal => $field:expr, $($rest:tt)* )
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* ($tag, $field),)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @input($($rest)*)
    }
  };

  // Process normal field
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @input($(,)? $tag:literal => $field:expr)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* ($tag, $field))
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @input()
    }
  };
}

#[macro_export]
macro_rules! proto_enum {
  ($enum:expr, $($tokens:tt)*) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options(),
      @reserved(),
      @reserved_names(),
      @rest($($tokens)*)
    }
  };
}

#[macro_export]
macro_rules! proto_enum_impl {
  (
    @builder($enum:expr),
    @options(),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:tt)*),
    @rest(options = $options_expr:expr, $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($options_expr),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:tt)*),
    @reserved(),
    @reserved_names($($reserved_names:tt)*),
    @rest(reserved = [ $($items:tt)* ], $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($items)*),
      @reserved_names($($reserved_names)*),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:tt)*),
    @reserved($($reserved:tt)*),
    @reserved_names(),
    @rest(reserved_names = $reserved_names:expr, $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names($reserved_names),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:expr)?),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:expr)?),
    @rest($($tag:literal => $variant:literal),* $(,)?)
  ) => {
    {
      let mut temp_enum = $enum

      $(
        .options($options.as_slice())
      )?

      $(
        .reserved_names($reserved_names.as_slice())
      )?

      .variants(
        &[ $(($tag, $variant.into())),* ]
      );

      $crate::parse_reserved!{
        @builder(temp_enum)
        @ranges()
        @numbers()
        @rest($($reserved)*)
      }
    }
  };
}

#[macro_export]
macro_rules! parse_reserved {
  (
    @builder($builder:expr)
    @ranges()
    @numbers()
    @rest($(,)?)
  ) => {
     $builder
  };

  (
    @builder($builder:expr)
    @ranges($($start:literal..$end:literal),* $(,)?)
    @numbers()
    @rest($(,)?)
  ) => {
     $builder
      .reserved_ranges(&[$(::std::ops::Range { start: $start, end: $end }),*])
  };

  (
    @builder($builder:expr)
    @ranges()
    @numbers($($number:literal),* $(,)?)
    @rest($(,)?)
  ) => {
     $builder
      .reserved_numbers(&[$($number),*])
  };

  (
    @builder($builder:expr)
    @ranges($($start:literal..$end:literal),* $(,)?)
    @numbers($($number:literal),* $(,)?)
    @rest($(,)?)
  ) => {
     $builder
      .reserved_ranges(&[$(::std::ops::Range { start: $start, end: $end }),*])
      .reserved_numbers(&[$($number),*])
  };

  (
    @builder($builder:expr)
    @ranges($($ranges:tt)*)
    @numbers($($numbers:tt)*)
    @rest($(,)? $start:literal..$end:literal $($rest:tt)* )
  ) => {
    $crate::parse_reserved!{
      @builder($builder)
      @ranges($($ranges)* $start..$end,)
      @numbers($($numbers)*)
      @rest($($rest)*)
    }
  };

  (
    @builder($builder:expr)
    @ranges($($ranges:tt)*)
    @numbers($($numbers:tt)*)
    @rest($(,)? $number:literal $($rest:tt)* )
  ) => {
    $crate::parse_reserved!{
      @builder($builder)
      @ranges($($ranges)*)
      @numbers($($numbers)* $number,)
      @rest($($rest)*)
    }
  };
}

#[macro_export]
macro_rules! oneof {
  (
    $msg:expr,
    $name:expr,
    options = $options_expr:expr,
    $($tag:literal => $field:expr),* $(,)?
  ) => {
    {
      $crate::oneofs::OneofData::builder()
        .name($name.to_string())
        .parent_message_id($msg.get_id())
        .options($options_expr.as_slice())
        .fields(
          vec! [ $($field.tag($tag).build()),* ]
        )
        .build()
    }
  };

  (
    $msg:expr,
    $name:expr,
    $($tag:literal => $field:expr),* $(,)?
  ) => {
    {
      $crate::oneofs::OneofData::builder()
        .name($name.to_string())
        .parent_message_id($msg.get_id())
        .fields(
          vec! [ $($field.tag($tag).build()),* ]
        )
        .build()
    }
  };
}
