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
  #[builder(into)]
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
  pub fn options(self, options: &[ProtoOption]) -> OneofBuilder<oneof_builder::SetOptions<S>>
  where
    S::Options: IsUnset,
  {
    self.options_internal(options.into())
  }
}
