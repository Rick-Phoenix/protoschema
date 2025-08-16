use std::{collections::BTreeMap, fmt::Display};

use askama::Template;

#[derive(Template, Clone, Debug)]
#[template(path = "opt.proto.j2")]
pub struct ProtoOption {
  pub name: &'static str,
  pub value: OptionValue,
}

impl Display for OptionValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      OptionValue::Bool(v) => write!(f, "{}", v),
      OptionValue::Int(v) => write!(f, "{}", v),
      OptionValue::Float(v) => write!(f, "{}", v),
      OptionValue::String(v) => write!(f, "\"{}\"", v),
      OptionValue::List(values) => {
        write!(f, "[  ")?;
        for (idx, item) in values.iter().enumerate() {
          write!(f, "{}", item)?;
          if idx != values.len() - 1 {
            write!(f, ", ")?;
          }
        }
        write!(f, " ]")?;
        Ok(())
      }
      OptionValue::Message(btree_map) => {
        write!(f, "{{ ")?;
        for (idx, (key, val)) in btree_map.iter().enumerate() {
          write!(f, "{}: {}", key, val)?;
          if idx != btree_map.len() - 1 {
            write!(f, ", ")?;
          }
        }
        write!(f, " }}")?;
        Ok(())
      }
      OptionValue::Identifier(v) => {
        write!(f, "{}", v)
      }
      OptionValue::Duration { seconds, nanos } => {
        write!(f, "{{ seconds: {}, nanos: {} }}", seconds, nanos)
      }
      OptionValue::Timestamp { seconds, nanos } => {
        write!(f, "{{ seconds: {}, nanos: {} }}", seconds, nanos)
      }
    }
  }
}

#[derive(Clone, Debug)]
pub enum OptionValue {
  Bool(bool),
  Int(i64),
  Float(f64),
  String(String),
  List(Vec<OptionValue>),
  Message(BTreeMap<String, OptionValue>),
  Identifier(String),
  Duration { seconds: i64, nanos: i32 },
  Timestamp { seconds: i64, nanos: i32 },
}

#[macro_export]
macro_rules! proto_str_list {
  ($($val:expr),* $(,)?) => {
    ::protoschema::OptionValue::List(vec![
      $(
        OptionValue::String($val.to_string())
      ),*
    ])
  };
}
