use std::sync::Arc;

use proto_types::{Duration, Timestamp};

#[derive(Clone, Debug)]
pub struct ProtoOption {
  pub name: Arc<str>,
  pub value: OptionValue,
}

/// An enum representing values for protobuf options.
/// For building [`OptionValue`]s for options with a message type, try using the [`message_option`](crate::message_option) macro or the [`message_value`] helper. For lists, use the [`list_value`] helper. For options that have enum values, you can use the [`enum_option`](crate::enum_option) macro or the [`enum_values_list`] helper.
#[derive(Clone, Debug)]
pub enum OptionValue {
  Bool(bool),
  Int(i64),
  Uint(u64),
  Float(f64),
  String(Arc<str>),
  List(Arc<[OptionValue]>),
  Message(Arc<[(Arc<str>, OptionValue)]>),
  Enum(Arc<str>),
  Duration(Duration),
  Timestamp(Timestamp),
}

macro_rules! option_value_conversion {
  ($origin_type:ty, $dest_type:ident $(, as $as_type:ty)?) => {
    impl From<$origin_type> for OptionValue {
      fn from(value: $origin_type) -> OptionValue {
        OptionValue::$dest_type(value $(as $as_type)?)
      }
    }
  };
}

impl From<&str> for OptionValue {
  fn from(value: &str) -> Self {
    OptionValue::String(value.into())
  }
}

option_value_conversion!(Arc<[(Arc<str>, OptionValue)]>, Message);
option_value_conversion!(Arc<[OptionValue]>, List);
option_value_conversion!(bool, Bool);
option_value_conversion!(Duration, Duration);
option_value_conversion!(Timestamp, Timestamp);
option_value_conversion!(i64, Int);
option_value_conversion!(i32, Int, as i64);
option_value_conversion!(u64, Uint);
option_value_conversion!(u32, Uint, as u64);
option_value_conversion!(f64, Float);
option_value_conversion!(f32, Float, as f64);
