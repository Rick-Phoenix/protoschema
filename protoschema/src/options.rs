use std::sync::Arc;

use askama::Template;

pub use crate::field_type::{Duration, Timestamp};

#[derive(Clone, Debug)]
pub struct ProtoOption {
  pub name: &'static str,
  pub value: Arc<OptionValue>,
}

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
  Identifier(Box<str>),
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
  pub fn is_short(&self) -> bool {
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
