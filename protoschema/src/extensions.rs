use std::sync::Arc;

use bon::Builder;

use crate::{
  field_type::{get_shortest_item_name, ImportedItemPath},
  fields::{self, Field, FieldBuilder, FieldData},
};

/// A struct representing a protobuf extension
#[derive(Clone, Debug, Default, Builder)]
pub struct Extension {
  #[builder(setters(vis = "", name = fields_internal))]
  pub fields: Box<[Field]>,
  pub import_path: Arc<ImportedItemPath>,
}

impl<S: extension_builder::State> ExtensionBuilder<S> {
  /// Sets the fields for this extension
  pub fn fields<I, F>(self, fields: I) -> ExtensionBuilder<extension_builder::SetFields<S>>
  where
    S::Fields: extension_builder::IsUnset,
    I: IntoIterator<Item = FieldBuilder<F>>,
    F: fields::IsComplete,
  {
    self.fields_internal(fields.into_iter().map(|f| f.build()).collect())
  }
}

/// The processed data for a protobuf extension
#[derive(Clone, Debug, Default)]
pub struct ExtensionData {
  pub fields: Box<[FieldData]>,
  pub import_path: Arc<ImportedItemPath>,
}

impl ExtensionData {
  /// Returns the shortest name for the target of the extension (the fully qualified name if the message is defined outside of the given package, and the short name in the opposite case)
  pub fn get_target(&self, current_file: &str, current_package: &str) -> Arc<str> {
    get_shortest_item_name(&self.import_path, current_file, current_package)
  }
}
