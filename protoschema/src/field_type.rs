use std::{fmt::Display, sync::Arc};

pub use proto_types::{Duration, Timestamp};

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ImportedItemPath {
  pub name: Arc<str>,
  pub file: Arc<str>,
  pub package: Arc<str>,
}

impl ImportedItemPath {
  pub fn full_name(&self) -> String {
    format!("{}.{}", self.package, self.name)
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
  Enum(Arc<ImportedItemPath>),
  Sfixed32,
  Sfixed64,
  Sint32,
  Sint64,
  Message(Arc<ImportedItemPath>),
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

pub(crate) fn strip_common_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  let zipped_chars = s1.chars().zip(s2.chars());

  let prefix_len = zipped_chars.take_while(|(c1, c2)| c1 == c2).count();

  let byte_offset = s1.chars().take(prefix_len).map(|c| c.len_utf8()).sum();

  (&s1[byte_offset..]) as _
}

pub fn get_shortest_item_name(
  path: &Arc<ImportedItemPath>,
  current_file: &str,
  current_package: &str,
) -> Arc<str> {
  if path.file.as_ref() == current_file || path.package.as_ref() == current_package {
    path.name.clone()
  } else {
    format!("{}.{}", path.package, path.name).into()
  }
}

impl FieldType {
  pub fn render_name(&self, current_file: &str, current_package: &str) -> Arc<str> {
    match self {
      FieldType::Message(path) => get_shortest_item_name(path, current_file, current_package),
      FieldType::Enum(path) => get_shortest_item_name(path, current_file, current_package),
      FieldType::Map(key, val) => format!(
        "map<{}, {}>",
        key,
        val.render_name(current_file, current_package)
      )
      .into(),
      _ => self.name().into(),
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
      FieldType::Enum(path) => path.name.as_ref(),
      FieldType::Sfixed32 => "sfixed32",
      FieldType::Sfixed64 => "sfixed64",
      FieldType::Sint32 => "sint32",
      FieldType::Sint64 => "sint64",
      FieldType::Message(path) => path.name.as_ref(),
      FieldType::Duration => "google.protobuf.Duration",
      FieldType::Timestamp => "google.protobuf.Timestamp",
      FieldType::Any => "google.protobuf.Any",
      FieldType::FieldMask => "google.protobuf.FieldMask",
      FieldType::Empty => "google.protobuf.Empty",
      FieldType::Map(_, _) => "map",
    }
  }
}

impl From<&str> for FieldType {
  #[track_caller]
  fn from(value: &str) -> Self {
    match value {
      "double" => Self::Double,
      "float" => Self::Float,
      "int64" => Self::Int64,
      "int32" => Self::Int32,
      "uint64" => Self::Uint64,
      "uint32" => Self::Uint32,
      "fixed64" => Self::Fixed64,
      "fixed32" => Self::Fixed32,
      "bool" => Self::Bool,
      "string" => Self::String,
      "bytes" => Self::Bytes,
      "sfixed32" => Self::Sfixed32,
      "sfixed64" => Self::Sfixed64,
      "sint32" => Self::Sint32,
      "sint64" => Self::Sint64,
      "duration" => Self::Duration,
      "timestamp" => Self::Timestamp,
      "any" => Self::Any,
      "fieldmask" => Self::FieldMask,
      "empty" => Self::Empty,
      _ => panic!("Invalid protobuf type"),
    }
  }
}
