macro_rules! parse_field_type {
  ($ty:ident) => {
    paste! {
      FieldType::[< $ty:camel >]
    }
  };
}

macro_rules! msg_field {
  ($msg:ident, $field_name:ident = $tag:literal $(, [$($option_name:expr),*])? $(,)? ) => {
    field!($msg.get_field_type(), $field_name = $tag $(, [$($option_name),*])? )
  };
}

macro_rules! field {
  ($field_type:expr, $field_name:ident = $tag:literal $(, [$($option_name:expr),*])? $(,)?) => {
    FieldData::builder().name(stringify!($field_name).into()).field_type($field_type).tag($tag).options(vec![
      $($($option_name),*)?
    ])
  };
}

macro_rules! string {
  ($field_name:ident = $tag:literal $(, [$($option_name:expr),*])? $(,)?) => {
    field!(parse_field_type!(string), $field_name = $tag $(, [$($option_name),*])? )
  };
}

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
