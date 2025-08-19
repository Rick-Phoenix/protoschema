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
      @reserved()
      @reserved_names()
      @input($($tokens)*)
    }.options($options)
  };

  ($msg_builder:ident, $($tokens:tt)*) => {
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
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($names:expr)?)
    @input($(,)?)
  ) => {
    {
      { $($enums)* };

      let fields_map = btreemap! { $($fields)* };
      let oneofs_map: Vec<OneofData> = vec! [ $($oneofs)* ];

      let mut new_msg = $builder
        .fields(fields_map)
        .oneofs(oneofs_map)
      $(
        .reserved_names($names)
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
    @builder($builder:ident)
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
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names()
    @input($(,)? reserved_names = [ $($name:literal),* ] $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names( &[ $($name),* ] )
      @input($($rest)*)
    }
  };

  // Handle enum
  (
    @builder($builder:ident)
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
    @builder($builder:ident)
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
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @input($(,)? $tag:literal => $field:expr, $($rest:tt)* )
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* $tag => $field,)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @input($($rest)*)
    }
  };

  // Process normal field
  (
    @builder($builder:ident)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @input($(,)? $tag:literal => $field:expr)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* $tag => $field)
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
    @rest(reserved_names = [ $($names:literal),* $(,)? ], $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder( $enum ),
      @options( $($options)* ),
      @reserved( $($reserved)* ),
      @reserved_names(
        &[ $($names),* ]
      ),
      @rest( $($rest)* )
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
        .options($options)
      )?

      $(
        .reserved_names($reserved_names)
      )?

      .variants(
        btreemap! { $($tag => $variant.to_string()),* }
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
    @builder($builder:ident)
    @ranges()
    @numbers()
    @rest($(,)?)
  ) => {
     $builder
  };

  (
    @builder($builder:ident)
    @ranges($($start:literal..$end:literal),* $(,)?)
    @numbers()
    @rest($(,)?)
  ) => {
     $builder
      .reserved_ranges(&[$(::std::ops::Range { start: $start, end: $end }),*])
  };

  (
    @builder($builder:ident)
    @ranges()
    @numbers($($number:literal),* $(,)?)
    @rest($(,)?)
  ) => {
     $builder
      .reserved_numbers(&[$($number),*])
  };

  (
    @builder($builder:ident)
    @ranges($($start:literal..$end:literal),* $(,)?)
    @numbers($($number:literal),* $(,)?)
    @rest($(,)?)
  ) => {
     $builder
      .reserved_ranges(&[$(::std::ops::Range { start: $start, end: $end }),*])
      .reserved_numbers(&[$($number),*])
  };

  (
    @builder($builder:ident)
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
    @builder($builder:ident)
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
