use std::sync::Arc;

use askama::Template;

pub use crate::field_type::{Duration, Timestamp};

// A struct representing a protobuf option
#[derive(Clone, Debug)]
pub struct ProtoOption {
  pub name: &'static str,
  pub value: Arc<OptionValue>,
}

// A helper to build a ProtoOption
pub fn proto_option<T: Into<OptionValue>>(name: &'static str, value: T) -> ProtoOption {
  ProtoOption {
    name,
    value: Arc::new(value.into()),
  }
}

// An enum representing values for protobuf options
#[derive(Clone, Debug, Template)]
#[template(path = "option_value.proto.j2")]
pub enum OptionValue {
  Bool(bool),
  Int(i64),
  Uint(u64),
  Float(f64),
  String(Box<str>),
  List(Box<[OptionValue]>),
  Message(Box<[(Box<str>, OptionValue)]>),
  Enum(Box<str>),
  Duration(Duration),
  Timestamp(Timestamp),
}

// A helper to build an OptionValue::List
pub fn list_value<T: Into<Box<[OptionValue]>>>(i: T) -> OptionValue {
  OptionValue::List(i.into())
}

// A helper to build an OptionValue::Message
pub fn message_value<T: Into<Box<[(Box<str>, OptionValue)]>>>(i: T) -> OptionValue {
  OptionValue::Message(i.into())
}

// A helper to build an OptionValue::Int
pub fn int_value<T: Into<i64>>(n: T) -> OptionValue {
  OptionValue::Int(n.into())
}

// A helper to build an OptionValue::Uint
pub fn uint_value<T: Into<u64>>(n: T) -> OptionValue {
  OptionValue::Uint(n.into())
}

// A helper to build an OptionValue::Float
pub fn float_value<T: Into<f64>>(n: T) -> OptionValue {
  OptionValue::Float(n.into())
}

// A helper to build an OptionValue::String
pub fn string_value<T: AsRef<str>>(str: T) -> OptionValue {
  OptionValue::String(str.as_ref().into())
}

// A helper to build an OptionValue::Enum
pub fn enum_value<T: AsRef<str>>(name: T) -> OptionValue {
  OptionValue::String(name.as_ref().into())
}

// A helper to build an OptionValue::Duration
pub fn duration_value(d: Duration) -> OptionValue {
  OptionValue::Duration(d)
}

// A helper to build an OptionValue::Timestamp
pub fn timestamp_value(d: Timestamp) -> OptionValue {
  OptionValue::Timestamp(d)
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

option_value_conversion!(Box<[(Box<str>, OptionValue)]>, Message);
option_value_conversion!(Box<[OptionValue]>, List);
option_value_conversion!(bool, Bool);
option_value_conversion!(Duration, Duration);
option_value_conversion!(Timestamp, Timestamp);
option_value_conversion!(i64, Int);
option_value_conversion!(i32, Int, as i64);
option_value_conversion!(u64, Uint);
option_value_conversion!(u32, Uint, as u64);
option_value_conversion!(f64, Float);
option_value_conversion!(f32, Float, as f64);

impl OptionValue {
  pub(crate) fn is_short(&self) -> bool {
    match self {
      Self::List(list) => list.len() <= 5 && list.iter().all(OptionValue::is_short),
      Self::String(str) => str.chars().count() <= 5,
      Self::Duration(_) | Self::Timestamp(_) | Self::Message(_) => false,
      _ => true,
    }
  }
}

#[macro_export]
macro_rules! proto_str_list {
  ($($val:expr),* $(,)?) => {
    $crate::OptionValue::List(vec![
      $(
        OptionValue::String($val.to_string())
      ),*
    ])
  };
}
