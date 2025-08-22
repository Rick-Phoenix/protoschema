use std::sync::Arc;

use bon::Builder;
pub(crate) use oneof_builder::*;

use crate::{
  fields::{Field, FieldData},
  ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct Oneof {
  pub name: Arc<str>,
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[Field]>,
  #[builder(default)]
  #[builder(setters(vis = "", name = options_internal))]
  pub options: Box<[ProtoOption]>,
}

#[derive(Clone, Debug)]
pub struct OneofData {
  pub name: Arc<str>,
  pub fields: Box<[FieldData]>,
  pub options: Box<[ProtoOption]>,
}

impl<S: oneof_builder::State> OneofBuilder<S> {
  pub fn fields<I>(self, fields: I) -> OneofBuilder<oneof_builder::SetFields<S>>
  where
    S::Fields: IsUnset,
    I: IntoIterator<Item = Field>,
  {
    self.fields_internal(fields.into_iter().collect())
  }

  pub fn options<I>(self, options: I) -> OneofBuilder<oneof_builder::SetOptions<S>>
  where
    S::Options: IsUnset,
    I: IntoIterator<Item = ProtoOption>,
  {
    self.options_internal(options.into_iter().collect())
  }
}
