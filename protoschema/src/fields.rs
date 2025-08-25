use std::{fmt::Display, sync::Arc};

use bon::Builder;
pub(crate) use field_builder::*;

use crate::{FieldType, ProtoOption};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum FieldKind {
  #[default]
  Normal,
  Repeated,
  Optional,
}

impl Display for FieldKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Normal => {
        write!(f, "")
      }
      Self::Repeated => {
        write!(f, "repeated ")
      }
      Self::Optional => {
        write!(f, "optional ")
      }
    }
  }
}

/// A struct representing a protobuf field
#[derive(Clone, Debug, Builder)]
#[builder(derive(Clone))]
pub struct Field {
  #[builder(field)]
  pub options: Vec<ProtoOption>,
  #[builder(field)]
  pub imports: Vec<Arc<str>>,
  #[builder(field)]
  pub kind: FieldKind,
  #[builder(setters(vis = "", name = field_type_internal))]
  pub field_type: FieldType,
  pub name: Arc<str>,
  pub tag: u32,
}

/// A struct representing the processed data for a protobuf field, after it's been added to a message or extension
#[derive(Clone, Debug)]
pub struct FieldData {
  pub options: Box<[ProtoOption]>,
  pub kind: FieldKind,
  pub field_type: FieldType,
  pub name: Arc<str>,
  pub tag: u32,
}

impl<S: field_builder::State> FieldBuilder<S> {
  /// Marks this field as `repeated`
  pub fn repeated(mut self) -> FieldBuilder<S> {
    self.kind = FieldKind::Repeated;
    self
  }

  /// Marks this field as `optional`
  pub fn optional(mut self) -> FieldBuilder<S> {
    self.kind = FieldKind::Optional;
    self
  }

  /// Sets the [`FieldType`] for this field
  pub fn field_type(self, field_type: FieldType) -> FieldBuilder<SetFieldType<S>>
  where
    S::FieldType: field_builder::IsUnset,
  {
    self.field_type_internal(field_type)
  }

  /// Adds an option to this field
  pub fn add_option(mut self, option: ProtoOption) -> Self {
    self.options.push(option);
    self
  }

  /// Adds multiple options to this field
  pub fn add_options<I>(mut self, options: I) -> Self
  where
    I: IntoIterator<Item = ProtoOption>,
  {
    self.options.extend(options);
    self
  }

  /// Adds an import to this field.
  /// When this field is cloned and reused in other messages, the receiving file will automatically add this import to its list.
  /// For the most common cases, this crate will automatically add the necessary imports, so make sure to use this only if you notice that an import is missing.
  pub fn add_import<T: AsRef<str>>(mut self, import: T) -> Self {
    self.imports.push(import.as_ref().into());
    self
  }
}
