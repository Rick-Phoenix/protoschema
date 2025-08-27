use std::sync::Arc;

use bon::Builder;
pub(crate) use oneof_builder::*;

use crate::{
  fields::{self, Field, FieldBuilder, FieldData},
  ProtoOption,
};

/// A struct representing a protobuf Oneof
#[derive(Clone, Debug, Builder)]
pub struct Oneof {
  pub name: Arc<str>,
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[(u32, Field)]>,
  #[builder(default)]
  #[builder(setters(vis = "", name = options_internal))]
  pub options: Box<[ProtoOption]>,
  #[builder(default, setters(vis = "", name = imports_internal))]
  pub imports: Box<[Arc<str>]>,
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct OneofData {
  pub name: Arc<str>,
  pub fields: Box<[(u32, FieldData)]>,
  pub options: Box<[ProtoOption]>,
}

impl<S: oneof_builder::State> OneofBuilder<S> {
  /// Adds a list of imports to this oneof.
  /// These will be added to the receiving file.
  pub fn imports<I, Str>(self, imports: I) -> OneofBuilder<oneof_builder::SetImports<S>>
  where
    I: IntoIterator<Item = Str>,
    Str: Into<Arc<str>>,
    S::Imports: oneof_builder::IsUnset,
  {
    self.imports_internal(imports.into_iter().map(|i| i.into()).collect())
  }

  /// Sets the fields for this oneof
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

  /// Sets the options for this oneof
  pub fn options<I>(self, options: I) -> OneofBuilder<oneof_builder::SetOptions<S>>
  where
    S::Options: IsUnset,
    I: IntoIterator<Item = ProtoOption>,
  {
    self.options_internal(options.into_iter().collect())
  }
}
