use std::sync::Arc;

use bon::Builder;

use crate::{
  field_type::{get_shortest_item_name, ImportedItemPath},
  fields::{self, Field, FieldBuilder, FieldData},
};

#[derive(Clone, Debug, Default, Builder)]
pub struct Extension {
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[Field]>,
  pub import_path: Arc<ImportedItemPath>,
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
  pub fields: Box<[FieldData]>,
  pub import_path: Arc<ImportedItemPath>,
}

impl ExtensionData {
  pub fn get_target(&self, current_file: &str, current_package: &str) -> Arc<str> {
    get_shortest_item_name(&self.import_path, current_file, current_package)
  }
}
