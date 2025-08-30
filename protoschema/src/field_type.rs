use std::{fmt::Display, sync::Arc};

#[doc(inline)]
pub use proto_types::{Duration, Timestamp};

/// Protobuf map key types
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

/// The import path information for a protobuf importable item, like an enum or a message
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ImportedItemPath {
  pub full_name: Arc<str>,
  pub full_name_with_package: Arc<str>,
  pub file: Arc<str>,
  pub package: Arc<str>,
}

/// The various types of protobuf fields, including some well known types such as [`any`](::proto_types::Any) or [`duration`](::proto_types::Duration)
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
  Struct,
  Money,
  Interval,
  Color,
  Date,
  DateTime,
  TimeZone,
  DayOfWeek,
  Decimal,
  Expr,
  Fraction,
  LatLng,
  LocalizedText,
  Month,
  PhoneNumber,
  PostalAddress,
  Quaternion,
  TimeOfDay,
  Status,
  Code,
  HttpRequest,
  HttpResponse,
  HttpHeader,
  ErrorInfo,
  RetryInfo,
  DebugInfo,
  QuotaFailure,
  QuotaFailureViolation,
  PreconditionFailure,
  PreconditionFailureViolation,
  BadRequest,
  FieldViolation,
  RequestInfo,
  ResourceInfo,
  Help,
  LocalizedMessage,
  Link,
}

impl Display for FieldType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}

pub(crate) fn get_shortest_item_name(
  path: &Arc<ImportedItemPath>,
  current_file: &str,
  current_package: &str,
) -> Arc<str> {
  if path.file.as_ref() == current_file || path.package.as_ref() == current_package {
    path.full_name.clone()
  } else {
    path.full_name_with_package.clone()
  }
}

impl FieldType {
  pub(crate) fn render_name(&self, current_file: &str, current_package: &str) -> Arc<str> {
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
      FieldType::Enum(path) => path.full_name.as_ref(),
      FieldType::Sfixed32 => "sfixed32",
      FieldType::Sfixed64 => "sfixed64",
      FieldType::Sint32 => "sint32",
      FieldType::Sint64 => "sint64",
      FieldType::Message(path) => path.full_name.as_ref(),
      FieldType::Duration => "google.protobuf.Duration",
      FieldType::Timestamp => "google.protobuf.Timestamp",
      FieldType::Any => "google.protobuf.Any",
      FieldType::FieldMask => "google.protobuf.FieldMask",
      FieldType::Empty => "google.protobuf.Empty",
      FieldType::Struct => "google.protobuf.Struct",
      FieldType::Map(_, _) => "map",
      FieldType::Money => "google.type.Money",
      FieldType::Interval => "google.type.Interval",
      FieldType::Color => "google.type.Color",
      FieldType::Date => "google.type.Date",
      FieldType::DateTime => "google.type.DateTime",
      FieldType::TimeZone => "google.type.TimeZone",
      FieldType::DayOfWeek => "google.type.DayOfWeek",
      FieldType::Decimal => "google.type.Decimal",
      FieldType::Expr => "google.type.Expr",
      FieldType::Fraction => "google.type.Fraction",
      FieldType::LatLng => "google.type.LatLng",
      FieldType::LocalizedText => "google.type.LocalizedText",
      FieldType::Month => "google.type.Month",
      FieldType::PhoneNumber => "google.type.PhoneNumber",
      FieldType::PostalAddress => "google.type.PostalAddress",
      FieldType::Quaternion => "google.type.Quaternion",
      FieldType::TimeOfDay => "google.type.TimeOfDay",
      FieldType::Status => "google.rpc.Status",
      FieldType::Code => "google.rpc.Code",
      FieldType::HttpRequest => "google.rpc.HttpRequest",
      FieldType::HttpResponse => "google.rpc.HttpResponse",
      FieldType::HttpHeader => "google.rpc.HttpHeader",
      FieldType::ErrorInfo => "google.rpc.ErrorInfo",
      FieldType::RetryInfo => "google.rpc.RetryInfo",
      FieldType::DebugInfo => "google.rpc.DebugInfo",
      FieldType::QuotaFailure => "google.rpc.QuotaFailure",
      FieldType::QuotaFailureViolation => "google.rpc.QuotaFailure.Violation",
      FieldType::PreconditionFailure => "google.rpc.PreconditionFailure",
      FieldType::PreconditionFailureViolation => "google.rpc.PreconditionFailure.Violation",
      FieldType::BadRequest => "google.rpc.BadRequest",
      FieldType::FieldViolation => "google.rpc.BadRequest.FieldViolation",
      FieldType::RequestInfo => "google.rpc.RequestInfo",
      FieldType::ResourceInfo => "google.rpc.ResourceInfo",
      FieldType::Help => "google.rpc.Help",
      FieldType::Link => "google.rpc.Help.Link",
      FieldType::LocalizedMessage => "google.rpc.LocalizedMessage",
    }
  }
}
