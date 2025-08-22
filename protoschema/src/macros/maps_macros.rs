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
        .add_option($crate::validators::map:: [< build_map_ $key_type _keys_ $values_type_name _values_ validator >]($validator))
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
