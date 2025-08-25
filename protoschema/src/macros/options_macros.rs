/// A macro to easily define key-value pairs for a message [`OptionValue`](crate::options::OptionValue)
/// # Examples
/// ```
/// use protoschema::message_option;
///
/// let my_option_value = message_option!("name" => "cats", "are_cute" => true);
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
/// use protoschema::{proto_option, enum_option};
///
/// let my_option = proto_option("my_option", enum_option!("MY_ENUM_VALUE"));
/// ```
#[macro_export]
macro_rules! enum_option {
  ($val:expr) => {
    $crate::options::OptionValue::Enum($val.into())
  };
}
