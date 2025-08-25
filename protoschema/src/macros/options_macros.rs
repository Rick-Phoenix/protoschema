/// A macro to easily define key-value pairs for a message [`OptionValue`](crate::options::OptionValue)
/// # Examples
/// ```
/// message_option!("name" => "cats", "are_cute" => true),
/// ```
#[macro_export]
macro_rules! message_option {
  ($($name:expr => $value:expr),+ $(,)?) => {
    $crate::options::message_value(
      [ $(($name, $crate::options::OptionValue::from($value))),+ ]
    )
  };
}

/// A macro to define an enum [`OptionValue`](crate::options::OptionValue)
/// Since both strings and enum variants are defined by using strings, only the latter have a From impl, whereas for enums, this macro must be used
///
/// # Examples
/// ```
/// let my_option = ProtoOption { name: "my_option", value: enum_option!("MY_ENUM_VALUE") };
/// ```
#[macro_export]
macro_rules! enum_option {
  ($val:expr) => {
    $crate::options::OptionValue::Enum($val.into())
  };
}
