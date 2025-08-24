use std::sync::Arc;

use bon::Builder;
pub(crate) use oneof_builder::*;

use crate::{
  fields::{self, Field, FieldBuilder, FieldData},
  ProtoOption,
};

// A struct representing a protobuf Oneof
#[derive(Clone, Debug, Builder)]
pub struct Oneof {
  pub name: Arc<str>,
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[Field]>,
  #[builder(default)]
  #[builder(setters(vis = "", name = options_internal))]
  pub options: Box<[ProtoOption]>,
}

// The struct where the Oneof data is stored
#[derive(Clone, Debug)]
pub struct OneofData {
  pub name: Arc<str>,
  pub fields: Box<[FieldData]>,
  pub options: Box<[ProtoOption]>,
}

impl<S: oneof_builder::State> OneofBuilder<S> {
  // Sets the fields for this oneof
  pub fn fields<I, F>(self, fields: I) -> OneofBuilder<oneof_builder::SetFields<S>>
  where
    S::Fields: IsUnset,
    I: IntoIterator<Item = FieldBuilder<F>>,
    F: fields::IsComplete,
  {
    self.fields_internal(fields.into_iter().map(|f| f.build()).collect())
  }

  // Sets the options for this oneof
  pub fn options<I>(self, options: I) -> OneofBuilder<oneof_builder::SetOptions<S>>
  where
    S::Options: IsUnset,
    I: IntoIterator<Item = ProtoOption>,
  {
    self.options_internal(options.into_iter().collect())
  }
}
