use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ImportedItemPath {
  pub package: String,
  pub file: String,
  pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Timestamp {
  pub seconds: i64,
  pub nanos: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
  pub seconds: i64,
  pub nanos: i32,
}

#[derive(Debug, Clone)]
pub enum FieldType {
  Double,
  Float,
  Int64,
  Uint64,
  Int32,
  Fixed64,
  Fixed32,
  Bool,
  String,
  Bytes,
  Uint32,
  Enum(Box<str>),
  Sfixed32,
  Sfixed64,
  Sint32,
  Sint64,
  Message(Box<str>),
  Duration,
  Timestamp,
  Any,
  FieldMask,
  Empty,
}

impl Display for FieldType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}

fn strip_common_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  let zipped_chars = s1.chars().zip(s2.chars());

  let prefix_len = zipped_chars.take_while(|(c1, c2)| c1 == c2).count();

  let byte_offset = s1.chars().take(prefix_len).map(|c| c.len_utf8()).sum();

  (&s1[byte_offset..]) as _
}

impl FieldType {
  pub fn render_name<T: AsRef<str> + Display>(&self, prefix: T) -> String {
    match self {
      FieldType::Message(name) => {
        strip_common_prefix(name, &format!("{}.", prefix.as_ref())).to_string()
      }
      FieldType::Enum(name) => {
        strip_common_prefix(name, &format!("{}.", prefix.as_ref())).to_string()
      }
      _ => self.name().to_string(),
    }
  }

  pub fn name(&self) -> &str {
    match self {
      FieldType::Double => "double",
      FieldType::Float => "float",
      FieldType::Int64 => "int64",
      FieldType::Uint64 => "uint64",
      FieldType::Int32 => "int32",
      FieldType::Fixed64 => "fixed64",
      FieldType::Fixed32 => "fixed32",
      FieldType::Bool => "bool",
      FieldType::String => "string",
      FieldType::Bytes => "bytes",
      FieldType::Uint32 => "uint32",
      FieldType::Enum(name) => name,
      FieldType::Sfixed32 => "sfixed32",
      FieldType::Sfixed64 => "sfixed64",
      FieldType::Sint32 => "sint32",
      FieldType::Sint64 => "sint64",
      FieldType::Message(name) => name,
      FieldType::Duration => "google.protobuf.Duration",
      FieldType::Timestamp => "google.protobuf.Timestamp",
      FieldType::Any => "google.protobuf.Any",
      FieldType::FieldMask => "google.protobuf.FieldMask",
      FieldType::Empty => "google.protobuf.Empty",
    }
  }
}
