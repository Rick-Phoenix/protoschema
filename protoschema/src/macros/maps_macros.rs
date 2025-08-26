#[doc(hidden)]
#[macro_export]
macro_rules! map_impl {
  ($name:literal, $key_type:ident, $value_type:expr, $values_type_name:ident $(, $validator:expr)?) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($crate::FieldType::Map(
        $crate::MapKey:: [<  $key_type:camel >],
        Box::new($value_type)
      ))
      $(
        .add_option($crate::validators::map:: [< build_map_ $key_type _keys_ $values_type_name _values_ validator >]($validator))
        .add_import("buf/validate/validate.proto")
      )?
    }
  };
}

/// Evaluates to a protobuf map [`FieldBuilder`](crate::fields::FieldBuilder).
///
/// Use [`enum_map`](crate::enum_map) or [`msg_map`](crate::msg_map) if the values for the map are messages or enums.
/// The first argument is the name of the field, followed by `<$key_type, $value_type>`, which are two idents describing the field types.
/// The last, optional argument is a closure that will receive three arguments, the first being a [`MapValidator`](crate::validators::map::MapValidator) instance, and the other two being the field validator instances for the given key/value types.
/// # Examples
/// ```
/// use protoschema::map;
/// // Without validator
/// let my_field = map!("my_map", <int64, string>);
/// // With validator
/// let my_field2 = map!("my_map_with_validator",
///   <int64, string>,
///   |map_validator, keys_validator, values_validator|
///   map_validator.min_pairs(2).keys(keys_validator.gt(0)).values(values_validator.min_len(1))
/// );
/// ```
#[macro_export]
macro_rules! map {
  ($name:expr, <$key_type:ident, $value_type:ident> $(, $validator:expr)?) => {
    $crate::paste! {
      $crate::map_impl!(
        $name, $key_type, $crate::FieldType::[< $value_type:camel >], $value_type $(, $validator)?
      )
    }
  };
}

/// Evaluates to a protobuf map field, where the values are of the specified enum type.
///
/// The first argument is the name of the field, followed by `<$key_type, $enum_ident>`, where $key_type is a plain ident for the type of the keys, and the $enum_ident is an ident pointing to a [`EnumBuilder`](crate::enums::EnumBuilder) instance.
/// The last, optional argument is a closure that will receive three arguments, the first being a [`MapValidator`](crate::validators::map::MapValidator) instance, and the other two being the field validator instances for the given key type and the [`EnumValidator`](crate::validators::enums::EnumValidator) builder.
/// # Examples
/// ```
/// use protoschema::{enum_map, Package};
///
/// let pkg = Package::new("my_pkg");
/// let file = pkg.new_file("my_file");
/// let my_enum = file.new_enum("my_enum");
/// // Without validator
/// let my_field = enum_map!("my_map", <int64, my_enum>);
/// // With validator
/// let my_field2 = enum_map!("my_map_with_validator",
///   <int64, my_enum>,
///   |map_validator, keys_validator, values_validator|
///   map_validator.min_pairs(2).keys(keys_validator.gt(0)).values(values_validator.defined_only())
/// );
/// ```
#[macro_export]
macro_rules! enum_map {
  ($name:expr, <$key_type:ident, $enum_ident:ident> $(, $validator:expr)?) => {
    $crate::map_impl!(
      $name, $key_type, $enum_ident.get_type(), enum $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };
}

/// Evaluates to a protobuf map field, where the values are of the specified message type.
///
/// The first argument is the name of the field, followed by `<$key_type, $msg_ident>`, where $key_type is a plain ident for the type of the keys, and the $msg_ident is an ident pointing to a [`MessageBuilder`](crate::messages::MessageBuilder) instance.
/// The last, optional argument is a closure that will receive three arguments, the first being a [`MapValidator`](crate::validators::map::MapValidator) instance, and the other two being the field validator instances for the given key type and the [`MessageValidator`](crate::validators::message::MessageValidator) builder.
///
/// # Examples
/// ```
/// use protoschema::{Package, msg_map, cel_rule};
///
/// let pkg = Package::new("my_pkg");
/// let file = pkg.new_file("my_file");
/// let my_msg = file.new_message("my_msg");
/// // Without validator
/// let my_field = msg_map!("my_map", <int64, my_msg>);
/// // With validator
/// let my_field2 = msg_map!("my_map_with_validator",
///   <int64, my_msg>,
///   |map_validator, keys_validator, values_validator|
///   map_validator.min_pairs(2).keys(keys_validator.gt(0)).values(values_validator.cel(&[
///     cel_rule!(
///       id = "passwords_not_matching",
///       msg = "the two passwords do not match",
///       expr = "this.password == this.repeated_password"
///     )
///   ]))
/// );
/// ```
#[macro_export]
macro_rules! msg_map {
  ($name:expr, <$key_type:ident, $message_ident:ident> $(, $validator:expr)?) => {
    $crate::map_impl!(
      $name, $key_type, $message_ident.get_type(), message $(, $validator)?
    )
    .add_import(&$message_ident.get_file())
  };
}
