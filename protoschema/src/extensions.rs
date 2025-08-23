use std::sync::Arc;

use bon::Builder;

use crate::fields::{self, Field, FieldBuilder, FieldData};

#[derive(Clone, Debug, Default, Builder)]
pub struct Extension {
  pub target: Arc<str>,
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[Field]>,
  pub import_path: Arc<str>,
}

impl<S: extension_builder::State> ExtensionBuilder<S> {
  pub fn fields<I, F>(self, fields: I) -> ExtensionBuilder<extension_builder::SetFields<S>>
  where
    S::Fields: extension_builder::IsUnset,
    I: IntoIterator<Item = FieldBuilder<F>>,
    F: fields::IsComplete,
  {
    self.fields_internal(fields.into_iter().map(|f| f.build()).collect())
  }
}

#[derive(Clone, Debug, Default)]
pub struct ExtensionData {
  pub target: Arc<str>,
  pub fields: Box<[FieldData]>,
  pub import_path: Arc<str>,
}
