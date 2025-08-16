use std::fmt::Display;

#[derive(Debug)]
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
  Enum { name: String, id: usize },
  Sfixed32,
  Sfixed64,
  Sint32,
  Sint64,
  Message { name: String, id: usize },
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

impl FieldType {
  /// Returns the short, lowercase name for the field type.
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
      FieldType::Enum { name, .. } => name,
      FieldType::Sfixed32 => "sfixed32",
      FieldType::Sfixed64 => "sfixed64",
      FieldType::Sint32 => "sint32",
      FieldType::Sint64 => "sint64",
      FieldType::Message { name, .. } => name,
      FieldType::Duration => "google.protobuf.Duration",
      FieldType::Timestamp => "google.protobuf.Timestamp",
      FieldType::Any => "google.protobuf.Any",
      FieldType::FieldMask => "google.protobuf.FieldMask",
      FieldType::Empty => "google.protobuf.Empty",
    }
  }
}
