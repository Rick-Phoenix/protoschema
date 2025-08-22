#[macro_export]
macro_rules! parse_field_type {
  ($ty:ident) => {
    $crate::FieldType::from($ty.get_type())
  };

  ($ty:expr) => {
    $crate::FieldType::from($ty)
  };
}

#[macro_export]
macro_rules! handler {
  ($handler:ident($request:expr => $response:expr) $($options:expr)?) => {
    $crate::services::ServiceHandler::new(stringify!($handler).into())
      .request(&$request)
      .response(&$response)
      $(.options(&$options))?
      .build()
  };
}

#[macro_export]
macro_rules! service {
  ($file:ident, $name:ident { options = $service_options:expr; $($handler_name:ident($request:ident => $response:ident) $([ $($handler_options:tt)+ ])?);+ $(;)? } $(;)?) => {
    $file
      .new_service(stringify!($name).into())
      .handlers(&[
        $($crate::handler!($handler_name($request => $response) $([ $($handler_options)+ ])?)),*
      ])
      .options(&$service_options)
  };
}

#[macro_export]
macro_rules! services {
  ($file:ident, $($service_name:ident { $($service:tt)* });+ $(;)?) => {
    {
      $(
        $crate::service!($file, $service_name { $($service)* })
      );*
    }
  };
}

#[macro_export]
macro_rules! repeated_field {
  ($name:literal, $field_type:expr, $proto_type:ident $(, $validator:expr)?) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .repeated()
      .field_type($field_type)
      $(
        .option($crate::validators::repeated::[< build_repeated_ $proto_type _validator_option >]($validator))
        .add_import("buf/validate/validate.proto")
      )?
    }
  };
}

#[macro_export]
macro_rules! field {
  ($($optional:ident)? $name:literal, $field_type:expr, $proto_type:ident, $module_name:ident $(, $validator:expr)? ) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($field_type)
      $(.$optional())?
      $(
        .option($crate::validators::$module_name::[< build_ $proto_type _validator_option >]($validator))
        .add_import("buf/validate/validate.proto")
      )?
    }
  };
}

macro_rules! field_impl {
  ($proto_type:ident, $module_name:ident $(, $import_path:literal)?) => {
    #[macro_export]
    macro_rules! $proto_type {
      (repeated $name:literal, $validator:expr) => {
        $crate::repeated_field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $validator
        )
        $(
          .add_import($import_path)
        )?
      };

      (repeated $name:literal) => {
        $crate::repeated_field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type
        )
        $(
          .add_import($import_path)
        )?
      };

      (optional $name:literal, $validator:expr) => {
        $crate::field!(
          optional
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name,
          $validator
        )
        $(
          .add_import($import_path)
        )?
      };

      (optional $name:literal) => {
        $crate::field!(
          optional
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name
        )
        $(
          .add_import($import_path)
        )?
      };

      ($name:literal, $validator:expr) => {
        $crate::field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name,
          $validator
        )
        $(
          .add_import($import_path)
        )?
      };

      ($name:literal) => {
        $crate::field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name
        )
        $(
          .add_import($import_path)
        )?
      };
    }
  };
}

field_impl!(string, string);
field_impl!(any, any, "google/protobuf/any.proto");
field_impl!(duration, duration, "google/protobuf/duration.proto");
field_impl!(timestamp, timestamp, "google/protobuf/timestamp.proto");
field_impl!(bytes, bytes);
field_impl!(bool, booleans);
field_impl!(int64, numeric);
field_impl!(int32, numeric);
field_impl!(sint64, numeric);
field_impl!(sint32, numeric);
field_impl!(sfixed64, numeric);
field_impl!(sfixed32, numeric);
field_impl!(uint64, numeric);
field_impl!(uint32, numeric);
field_impl!(fixed64, numeric);
field_impl!(fixed32, numeric);
field_impl!(double, numeric);
field_impl!(float, numeric);

#[macro_export]
macro_rules! enum_field {
  (repeated $enum_ident:expr, $name:literal $(, $validator:expr)?) => {
    $crate::repeated_field!(
      $name,
      $crate::FieldType::Enum($enum_ident.get_full_name().into()),
      enum
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };

  (optional $enum_ident:expr, $name:literal $(, $validator:expr)? ) => {
    $crate::field!(
      optional
      $name,
      $crate::FieldType::Enum($enum_ident.get_full_name().into()),
      enum
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };

  ($enum_ident:expr, $name:literal $(, $validator:expr)?) => {
    $crate::field!(
      $name,
      $crate::FieldType::Enum($enum_ident.get_full_name().into()),
      enum,
      enums
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };
}

#[macro_export]
macro_rules! msg_field {
  (repeated $msg_ident:expr, $name:literal $(, $validator:expr)?) => {
    $crate::repeated_field!(
      $name,
      $crate::FieldType::Message($msg_ident.get_full_name().into()),
      message
      $(, $validator)?
    )
    .add_import(&$msg_ident.get_file())
  };

  ($msg_ident:expr, $name:literal $(, $validator:expr)?) => {
    $crate::field!(
      $name,
      $crate::FieldType::Message($msg_ident.get_full_name().into()),
      message,
      message
      $(, $validator)?
    )
    .add_import(&$msg_ident.get_file())
  };
}

#[macro_export]
macro_rules! map_impl {
  ($name:literal, <$key_type:ident, $value_type:ident>, $values_type_name:ident $(, $validator:expr)?) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Map(
        $crate::MapKey:: [<  $key_type:camel >],
        Box::new($crate::parse_field_type!($value_type))
      ))
      $(
        .option($crate::validators::map:: [< build_map_ $key_type _keys_ $values_type_name _values_ validator >]($validator))
        .add_import("buf/validate/validate.proto")
      )?
    }
  };
}

#[macro_export]
macro_rules! map {
  ($name:literal, <$key_type:ident, $value_type:ident> $(, $validator:expr)?) => {
    $crate::map_impl!(
      $name, <$key_type, $value_type>, $value_type $(, $validator)?
    )
  };
}

#[macro_export]
macro_rules! enum_map {
  ($name:literal, <$key_type:ident, $enum_ident:ident> $(, $validator:expr)?) => {
    $crate::map_impl!(
      $name, <$key_type, $enum_ident>, enum $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };
}

#[macro_export]
macro_rules! msg_map {
  ($name:literal, <$key_type:ident, $message_ident:ident> $(, $validator:expr)?) => {
    $crate::map_impl!(
      $name, <$key_type, $message_ident>, message $(, $validator)?
    )
    .add_import(&$message_ident.get_file())
  };
}

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
