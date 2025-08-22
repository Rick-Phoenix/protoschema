use std::sync::Arc;

use crate::fields::{Field, FieldData};

#[derive(Clone, Debug, Default)]
pub struct Extension {
  pub target: Arc<str>,
  pub fields: Box<[Field]>,
  pub import_path: Arc<str>,
}

#[derive(Clone, Debug, Default)]
pub struct ExtensionData {
  pub target: Arc<str>,
  pub fields: Box<[FieldData]>,
  pub import_path: Arc<str>,
}
