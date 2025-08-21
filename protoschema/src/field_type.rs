use std::fmt::Display;

pub use proto_types::{Duration, Timestamp};

#[derive(Debug, Clone)]
pub struct ImportedItemPath {
  pub package: String,
  pub file: String,
  pub name: String,
}

#[derive(Debug, Clone)]
pub enum MapKey {
  Int32,
  Int64,
  Uint32,
  Uint64,
  Sint32,
  Sint64,
  Fixed32,
  Fixed64,
  Sfixed32,
  Sfixed64,
  Bool,
  String,
}

impl MapKey {
  pub fn name(&self) -> &str {
    match self {
      Self::Int64 => "int64",
      Self::Uint64 => "uint64",
      Self::Int32 => "int32",
      Self::Fixed64 => "fixed64",
      Self::Fixed32 => "fixed32",
      Self::Bool => "bool",
      Self::String => "string",
      Self::Uint32 => "uint32",
      Self::Sfixed32 => "sfixed32",
      Self::Sfixed64 => "sfixed64",
      Self::Sint32 => "sint32",
      Self::Sint64 => "sint64",
    }
  }
}

impl Display for MapKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
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
  Map(MapKey, Box<FieldType>),
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

  pub fn name(&self) -> Box<str> {
    match self {
      FieldType::Double => "double".into(),
      FieldType::Float => "float".into(),
      FieldType::Int64 => "int64".into(),
      FieldType::Uint64 => "uint64".into(),
      FieldType::Int32 => "int32".into(),
      FieldType::Fixed64 => "fixed64".into(),
      FieldType::Fixed32 => "fixed32".into(),
      FieldType::Bool => "bool".into(),
      FieldType::String => "string".into(),
      FieldType::Bytes => "bytes".into(),
      FieldType::Uint32 => "uint32".into(),
      FieldType::Enum(name) => name.clone(),
      FieldType::Sfixed32 => "sfixed32".into(),
      FieldType::Sfixed64 => "sfixed64".into(),
      FieldType::Sint32 => "sint32".into(),
      FieldType::Sint64 => "sint64".into(),
      FieldType::Message(name) => name.clone(),
      FieldType::Duration => "google.protobuf.Duration".into(),
      FieldType::Timestamp => "google.protobuf.Timestamp".into(),
      FieldType::Any => "google.protobuf.Any".into(),
      FieldType::FieldMask => "google.protobuf.FieldMask".into(),
      FieldType::Empty => "google.protobuf.Empty".into(),
      FieldType::Map(key, value) => format!("map<{}, {}>", key, value).into(),
    }
  }
}
