use bon::Builder;
pub(crate) use field_builder::*;

use crate::{FieldType, ProtoOption};

#[derive(Clone, Debug, Builder)]
#[builder(derive(Clone))]
pub struct Field {
  #[builder(field)]
  pub options: Vec<ProtoOption>,
  #[builder(field)]
  pub imports: Vec<Box<str>>,
  #[builder(setters(vis = "", name = field_type_internal))]
  pub field_type: FieldType,
  pub name: Box<str>,
  pub tag: u32,
}

impl<S: field_builder::State> FieldBuilder<S> {
  pub fn field_type(self, field_type: FieldType) -> FieldBuilder<SetFieldType<S>>
  where
    S::FieldType: field_builder::IsUnset,
  {
    self.field_type_internal(field_type)
  }

  pub fn option(mut self, option: ProtoOption) -> Self {
    self.options.push(option);
    self
  }

  pub fn options(mut self, options: &[ProtoOption]) -> Self {
    self.options = options.to_vec();
    self
  }

  pub fn add_import(mut self, import: &str) -> Self {
    self.imports.push(import.into());
    self
  }
}
