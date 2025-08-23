use std::{fmt::Display, sync::Arc};

pub use crate::field_type::{Duration, Timestamp};

#[derive(Clone, Debug)]
pub struct ProtoOption {
  pub name: &'static str,
  pub value: Arc<OptionValue>,
}

#[derive(Clone, Debug)]
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

impl Display for OptionValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      OptionValue::Bool(v) => write!(f, "{}", v),
      OptionValue::Int(v) => write!(f, "{}", v),
      OptionValue::Uint(v) => write!(f, "{}", v),
      OptionValue::Float(v) => write!(f, "{}", v),
      OptionValue::String(v) => write!(f, "\"{}\"", v),
      OptionValue::List(values) => {
        write!(f, "[ ")?;
        for (idx, item) in values.iter().enumerate() {
          write!(f, "{}", item)?;
          if idx != values.len() - 1 {
            write!(f, ", ")?;
          }
        }
        write!(f, " ]")?;
        Ok(())
      }
      OptionValue::Message(key_value_pairs) => {
        write!(f, "{{ ")?;
        for (idx, (key, val)) in key_value_pairs.iter().enumerate() {
          write!(f, "{}: {}", key, val)?;
          if idx != key_value_pairs.len() - 1 {
            write!(f, ", ")?;
          }
        }
        write!(f, " }}")?;
        Ok(())
      }
      OptionValue::Identifier(v) => {
        write!(f, "{}", v)
      }
      OptionValue::Duration(Duration { seconds, nanos }) => {
        write!(f, "{{ seconds: {}, nanos: {} }}", seconds, nanos)
      }
      OptionValue::Timestamp(Timestamp { seconds, nanos }) => {
        write!(f, "{{ seconds: {}, nanos: {} }}", seconds, nanos)
      }
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
