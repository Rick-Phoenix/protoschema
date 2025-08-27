use std::sync::Arc;

use bon::Builder;
pub(crate) use oneof_builder::*;

use crate::{
  common::VALIDATE_PROTO_FILE,
  fields::{self, Field, FieldBuilder, FieldData},
  OptionValue, ProtoOption,
};

/// A struct representing a protobuf Oneof
#[derive(Clone, Debug, Builder)]
pub struct Oneof {
  #[builder(field)]
  pub imports: Vec<Arc<str>>,
  #[builder(field)]
  pub options: Vec<ProtoOption>,
  pub name: Arc<str>,
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[(u32, Field)]>,
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct OneofData {
  pub name: Arc<str>,
  pub fields: Box<[(u32, FieldData)]>,
  pub options: Box<[ProtoOption]>,
}

impl<S: oneof_builder::State> OneofBuilder<S> {
  /// Makes the oneof required, using the related protovalidate option, meaning that at least one field in this oneof will have to be set in order for it to be considered as valid.
  pub fn required(self) -> OneofBuilder<S> {
    self
      .add_imports([VALIDATE_PROTO_FILE.clone()])
      .add_options([ProtoOption {
        name: "(buf.validate.oneof).required",
        value: Arc::new(OptionValue::Bool(true)),
      }])
  }

  /// Adds a list of imports to this oneof.
  /// These will be added to the receiving file.
  pub fn add_imports<I, Str>(mut self, imports: I) -> OneofBuilder<S>
  where
    I: IntoIterator<Item = Str>,
    Str: Into<Arc<str>>,
  {
    self.imports.extend(imports.into_iter().map(|i| i.into()));
    self
  }

  /// Sets the fields for this oneof.
  /// Used by the [`oneof`](crate::oneof) macro.
  pub fn fields<I, F>(self, fields: I) -> OneofBuilder<oneof_builder::SetFields<S>>
  where
    S::Fields: IsUnset,
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

  /// Adds the given options to this oneof.
  pub fn add_options<I>(mut self, options: I) -> OneofBuilder<S>
  where
    I: IntoIterator<Item = ProtoOption>,
  {
    self.options.extend(options.into_iter());
    self
  }
}
