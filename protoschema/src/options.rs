use std::sync::Arc;

use askama::Template;

pub use crate::field_type::{Duration, Timestamp};

/// A struct representing a protobuf option
#[derive(Clone, Debug)]
pub struct ProtoOption {
  pub name: &'static str,
  pub value: Arc<OptionValue>,
}

/// A helper to build a [`ProtoOption`]
pub fn proto_option<T: Into<OptionValue>>(name: &'static str, value: T) -> ProtoOption {
  ProtoOption {
    name,
    value: Arc::new(value.into()),
  }
}

/// An enum representing values for protobuf options
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

/// A helper to build a list of protobuf option values
pub fn list_value<L, I>(l: L) -> OptionValue
where
  L: IntoIterator<Item = I>,
  I: Into<OptionValue>,
{
  OptionValue::List(l.into_iter().map(|i| i.into()).collect())
}

/// A helper to build a list of enum values
pub fn enum_values_list<L, I>(l: L) -> OptionValue
where
  L: IntoIterator<Item = I>,
  I: AsRef<str>,
{
  OptionValue::List(
    l.into_iter()
      .map(|s| OptionValue::Enum(s.as_ref().into()))
      .collect(),
  )
}

/// A helper to build an [`OptionValue`]::Message.
/// Used by the [`message_option`](crate::message_option) macro to easily compose message option values.
pub fn message_value<T, N, V>(v: T) -> OptionValue
where
  T: IntoIterator<Item = (N, V)>,
  N: AsRef<str>,
  V: Into<OptionValue>,
{
  OptionValue::Message(
    v.into_iter()
      .map(|(name, val)| (name.as_ref().into(), val.into()))
      .collect(),
  )
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
