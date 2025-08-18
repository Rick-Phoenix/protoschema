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
macro_rules! add_field {
  ($current_builder:expr, $field_def:expr) => {
    $current_builder.field($field_def)
  };

  ($current_builder:expr, $head_field:expr, $($tail_fields:expr),* $(,)?) =>  {
    add_field!(
      $current_builder.field($head_field),
      $($tail_fields),*
    )
  };
}

#[macro_export]
macro_rules! message_fields {
  ($message:ident, [$head_field:expr, $($tail_fields:expr),* ] $(,)?) => {
    add_field!($message.field($head_field), $($tail_fields),*)
  };
}

macro_rules! message {
  ($file:ident, $name:literal, [$head_field:expr, $($tail_fields:expr),* ] $(,)?) => {
    {
      let msg = $file.new_message($name);
      message_fields!(msg, [ $head_field, $($tail_fields),* ])
    }
  };
}
