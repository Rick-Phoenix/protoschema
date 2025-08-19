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
  ($field_name:ident, $validator:expr) => {
    Field::builder()
      .name(stringify!($field_name).into())
      .field_type(parse_field_type!(string))
      .option(build_string_validator_option($validator))
  };
  ($field_name:ident) => {
    Field::builder()
      .name(stringify!($field_name).into())
      .field_type(parse_field_type!(string))
  };
}

#[macro_export]
macro_rules! message_body {
  ($msg_builder:expr, $($tokens:tt)*) => {
    $crate::_internal_message_body! {
        @builder($msg_builder)
        @fields()
        @oneofs()
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
    @input()
  ) => {
    {
      let builder = $builder;
      let fields_map = btreemap! { $($fields)* };
      let oneofs_map = btreemap! { $($oneofs)* };
      builder.fields(fields_map).oneofs(oneofs_map)
    }
  };

  // Process oneof with a trailing comma
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @input(oneof $name:ident { $($oneof_body:tt)* }, $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
        @builder($builder)
        @fields($($fields)*)
        @oneofs($($oneofs)* stringify!($name).to_string() => btreemap!{$($oneof_body)*},)
        @input($($rest)*)
    }
  };

  // Process oneof without trailing comma
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @input(oneof $name:ident { $($oneof_body:tt)* })
  ) => {
    $crate::_internal_message_body! {
        @builder($builder)
        @fields($($fields)*)
        @oneofs($($oneofs)* stringify!($name).to_string() => btreemap!{$($oneof_body)*})
        @input()
    }
  };

  // Process normal field with trailing comma
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @input($tag:literal => $field:expr, $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
        @builder($builder)
        @fields($($fields)* $tag => $field,)
        @oneofs($($oneofs)*)
        @input($($rest)*)
    }
  };

  // Process normal field
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @oneofs($($oneofs:tt)*)
    @input($tag:literal => $field:expr)
  ) => {
    $crate::_internal_message_body! {
        @builder($builder)
        @fields($($fields)* $tag => $field)
        @oneofs($($oneofs)*)
        @input()
    }
  };
}
