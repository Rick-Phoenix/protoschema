#[macro_use]
mod macros;

pub use paste::paste;
mod items;
pub mod validators;
use std::{borrow::Cow, collections::BTreeSet, ops::Range, sync::Arc};

use bon::Builder;
pub use items::*;

pub trait ProtoMessage {
  fn name() -> &'static str;
}

pub trait ProtoEnumTrait {}

impl Message {
  pub fn full_name(&self) -> Cow<'_, str> {
    let name = self.name;

    if let Some(parent) = &self.parent_message {
      format!("{parent}.{name}").into()
    } else {
      Cow::Borrowed(name)
    }
  }
}

pub trait AsProtoType {
  fn proto_type() -> ProtoType;
}

pub trait ValidatorBuilderFor<T>: Into<ProtoOption> {}

pub trait ProtoValidator<T> {
  type Builder;

  fn builder() -> Self::Builder;

  #[track_caller]
  fn from_builder<B>(builder: B) -> ProtoOption
  where
    B: ValidatorBuilderFor<T>,
  {
    builder.into()
  }

  #[track_caller]
  fn build_rules<F, FinalBuilder>(config_fn: F) -> ProtoOption
  where
    F: FnOnce(Self::Builder) -> FinalBuilder,
    FinalBuilder: ValidatorBuilderFor<T>,
  {
    let initial_builder = Self::builder();

    let final_builder = config_fn(initial_builder);

    final_builder.into()
  }
}

pub struct ValidatorMap;

#[derive(Debug, Clone)]
pub struct ProtoField {
  pub name: String,
  pub type_: ProtoType,
  pub options: Vec<ProtoOption>,
  pub validator: Option<ProtoOption>,
}

#[derive(Debug, Clone)]
pub enum ProtoType {
  Single(TypeInfo),
  Repeated(TypeInfo),
  Map { keys: TypeInfo, values: TypeInfo },
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
  pub name: &'static str,
  pub path: Option<ProtoPath>,
}

#[derive(Debug, Default, Clone)]
pub struct EnumVariant {
  pub name: String,
  pub options: Vec<ProtoOption>,
}

impl ProtoFile {
  pub fn new(name: &str, package: &str) -> Self {
    Self {
      name: name.into(),
      package: package.into(),
      ..Default::default()
    }
  }

  pub fn add_messages<I: IntoIterator<Item = Message>>(&mut self, messages: I) {
    for message in messages.into_iter() {
      self.messages.push(message);
    }
  }

  pub fn add_enums<I: IntoIterator<Item = ProtoEnum>>(&mut self, enums: I) {
    for enum_ in enums.into_iter() {
      self.enums.push(enum_);
    }
  }
}

#[derive(Default)]
pub struct ProtoFile {
  pub name: Arc<str>,
  pub package: Arc<str>,
  pub imports: BTreeSet<Arc<str>>,
  pub messages: Vec<Message>,
  pub enums: Vec<ProtoEnum>,
  pub services: Vec<ServiceData>,
}

impl ProtoFile {
  pub fn add_message(&mut self, mut message: Message) -> &mut Message {
    message.file = self.name.clone();
    message.package = self.package.clone();

    let new_idx = self.messages.len();

    self.messages.push(message);

    &mut self.messages[new_idx]
  }

  pub fn path(&self) -> ProtoPath {
    ProtoPath {
      package: self.package.clone(),
      file: self.name.clone(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ProtoPath {
  pub package: Arc<str>,
  pub file: Arc<str>,
}

#[derive(Debug, Default, Clone)]
pub struct Message {
  pub name: &'static str,
  pub full_name: &'static str,
  pub package: Arc<str>,
  pub file: Arc<str>,
  pub fields: Vec<(u32, ProtoField)>,
  pub messages: Vec<Message>,
  pub oneofs: Vec<Oneof>,
  pub enums: Vec<ProtoEnum>,
  pub options: Vec<ProtoOption>,
  pub reserved_names: Vec<&'static str>,
  pub reserved_numbers: Vec<Range<u32>>,
  pub imports: BTreeSet<Arc<str>>,
  pub parent_message: Option<&'static str>,
}

#[derive(Debug, Default, Clone)]
pub struct Oneof {
  pub name: Arc<str>,
  pub fields: Vec<(u32, ProtoField)>,
  pub options: Vec<ProtoOption>,
}

impl Message {
  pub fn add_oneofs<I: IntoIterator<Item = Oneof>>(&mut self, oneofs: I) {
    self.oneofs = oneofs.into_iter().collect();
  }

  pub fn add_enums<I: IntoIterator<Item = ProtoEnum>>(&mut self, enums: I) {
    self.enums = enums.into_iter().collect();
  }
}

pub struct ServiceData {
  pub name: Box<str>,
  pub handlers: Box<[ServiceHandler]>,
  pub options: Box<[ProtoOption]>,
}

pub struct ImportedItemPath {
  pub file: String,
}

#[derive(Builder)]
pub struct ServiceHandler {
  pub name: Box<str>,
  pub options: Vec<ProtoOption>,
  pub request: Arc<ImportedItemPath>,
  pub response: Arc<ImportedItemPath>,
}

#[derive(Debug, Default, Clone)]
pub struct ProtoEnum {
  pub name: Arc<str>,
  pub full_name: &'static str,
  pub package: Arc<str>,
  pub file: Arc<str>,
  pub variants: Vec<(u32, EnumVariant)>,
  pub reserved_numbers: Vec<Range<u32>>,
  pub reserved_names: Vec<&'static str>,
  pub options: Vec<ProtoOption>,
}
