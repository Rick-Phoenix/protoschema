use bon::Builder;

use crate::fields::{self, Field, FieldBuilder, FieldData};

/// The kind of proto3 extension
#[derive(Debug, Clone, Copy)]
pub enum ExtensionKind {
  MessageOptions,
  FieldOptions,
  ServiceOptions,
  MethodOptions,
  OneofOptions,
  FileOptions,
  EnumOptions,
  EnumValueOptions,
}

impl ExtensionKind {
  /// Returns the name of the message being extended
  pub fn get_target(&self) -> &str {
    match self {
      ExtensionKind::MessageOptions => "google.protobuf.MessageOptions",
      ExtensionKind::FieldOptions => "google.protobuf.FieldOptions",
      ExtensionKind::ServiceOptions => "google.protobuf.ServiceOptions",
      ExtensionKind::MethodOptions => "google.protobuf.MethodOptions",
      ExtensionKind::OneofOptions => "google.protobuf.OneofOptions",
      ExtensionKind::FileOptions => "google.protobuf.FileOptions",
      ExtensionKind::EnumOptions => "google.protobuf.EnumOptions",
      ExtensionKind::EnumValueOptions => "google.protobuf.EnumValueOptions",
    }
  }
}

/// A struct representing a protobuf extension
#[derive(Clone, Debug, Builder)]
pub struct Extension {
  pub kind: ExtensionKind,
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[(u32, Field)]>,
}

impl<S: extension_builder::State> ExtensionBuilder<S> {
  /// Sets the fields for this extension
  pub fn fields<I, F>(self, fields: I) -> ExtensionBuilder<extension_builder::SetFields<S>>
  where
    S::Fields: extension_builder::IsUnset,
    I: IntoIterator<Item = (u32, FieldBuilder<F>)>,
    F: fields::IsComplete,
  {
    self.fields_internal(
      fields
        .into_iter()
        .map(|(tag, field)| (tag, field.build()))
        .collect(),
    )
  }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ExtensionData {
  pub kind: ExtensionKind,
  pub fields: Box<[(u32, FieldData)]>,
}
