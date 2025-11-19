#[macro_use]
mod macros;

pub use paste::paste;
mod items;
pub mod validators;
use std::{collections::BTreeSet, marker::PhantomData, ops::Range, sync::Arc};

use bon::Builder;
pub use items::*;

pub trait ValidatorBuilderFor<T>: Into<ProtoOption> {}

pub trait AsProtoType {
  fn proto_type() -> ProtoType;
}

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

#[derive(Clone)]
pub struct ProtoField {
  pub name: String,
  pub type_: ProtoType,
  pub options: Vec<ProtoOption>,
  pub validator: Option<ProtoOption>,
}

#[derive(Clone)]
pub enum ProtoType {
  Single(TypeInfo),
  Repeated(TypeInfo),
  Map { keys: TypeInfo, values: TypeInfo },
}

#[derive(Clone)]
pub struct TypeInfo {
  pub name: &'static str,
  pub path: Option<ProtoPath>,
}

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
pub struct Message {
  pub package: Arc<str>,
  pub file: Arc<str>,
  pub name: Arc<str>,
  pub fields: Vec<(u32, ProtoField)>,
  pub messages: Vec<Message>,
  pub oneofs: Vec<Oneof>,
  pub enums: Vec<ProtoEnum>,
  pub options: Vec<ProtoOption>,
  pub reserved_names: Vec<&'static str>,
  pub reserved_numbers: Vec<Range<u32>>,
  pub imports: BTreeSet<Arc<str>>,
}

#[derive(Default, Clone)]
pub struct Oneof {
  pub name: Arc<str>,
  pub fields: Vec<(u32, ProtoField)>,
  pub options: Vec<ProtoOption>,
}

impl Message {
  pub fn add_oneof(&mut self, oneof: Oneof) {
    self.oneofs.push(oneof);
  }

  pub fn nested_message(&mut self, mut message: Message) -> &mut Message {
    message.file = self.file.clone();
    message.package = self.package.clone();

    let new_idx = self.messages.len();

    self.messages.push(message);

    &mut self.messages[new_idx]
  }

  pub fn nested_enum(&mut self, mut proto_enum: ProtoEnum) -> &mut ProtoEnum {
    proto_enum.file = self.file.clone();
    proto_enum.package = self.package.clone();

    let new_idx = self.enums.len();

    self.enums.push(proto_enum);

    &mut self.enums[new_idx]
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

#[derive(Default, Clone)]
pub struct ProtoEnum {
  pub name: Arc<str>,
  pub package: Arc<str>,
  pub file: Arc<str>,
  pub variants: Vec<(i32, EnumVariant)>,
  pub reserved_numbers: Vec<Range<u32>>,
  pub reserved_names: Vec<&'static str>,
  pub options: Vec<ProtoOption>,
}
