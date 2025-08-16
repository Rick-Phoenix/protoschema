#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use bon::{builder, Builder};

use crate::{
  field_data_builder::{SetFieldType, SetName, SetTag},
  message_data_builder::IsSet,
};
pub use crate::{
  field_type::FieldType,
  option::{OptionValue, ProtoOption},
};

#[macro_use]
mod macros;
mod field_type;
#[macro_use]
mod option;

type Arena = Rc<RefCell<SchemaInner>>;

#[derive(Default, Debug)]
struct SchemaInner {
  pub(crate) files: Vec<FileData>,
  pub(crate) messages: Vec<MessageData>,
}

#[derive(Clone, Default)]
pub struct Package {
  arena: Arena,
}

impl Package {
  pub fn new_file(
    &self,
    name: &str,
  ) -> FileDataBuilder<
    file_data_builder::SetArena<file_data_builder::SetId<file_data_builder::SetName>>,
  > {
    let mut arena = self.arena.borrow_mut();
    let file_id = arena.files.len();

    arena.files.push(FileData::default());
    FileData::builder()
      .name(name.into())
      .id(file_id)
      .arena(self.arena.clone())
  }
}

impl<S: file_data_builder::State> FileDataBuilder<S>
where
  S::Arena: IsSet,
  S::Id: IsSet,
  S::Name: IsSet,
{
  pub fn new_message(
    &self,
    name: &str,
  ) -> MessageDataBuilder<
    message_data_builder::SetArena<
      message_data_builder::SetPackageId<
        message_data_builder::SetId<message_data_builder::SetFile<message_data_builder::SetName>>,
      >,
    >,
  > {
    let mut arena = self.get_arena().borrow_mut();
    let msg_id = arena.messages.len();
    arena.messages.push(MessageData::default());
    let data = &mut arena.files[*self.get_id()];
    data.messages.push(msg_id);

    MessageData::builder()
      .name(name.into())
      .file(data.name.to_string())
      .id(msg_id)
      .package_id(1)
      .arena(self.get_arena().clone())
  }
}

#[derive(Default, Builder, Debug)]
pub struct FileData {
  #[builder(getter(name = get_id, vis = ""))]
  pub id: usize,
  #[builder(getter(name = get_arena, vis = ""))]
  arena: Arena,
  pub name: Box<str>,
  pub options: Vec<ProtoOption>,
  pub messages: Vec<usize>,
  pub imports: Vec<String>,
  pub enums: Vec<usize>,
  pub package_id: String,
}

#[derive(Builder)]
pub struct PackageData {
  #[builder(field)]
  pub files: Vec<FileData>,
  pub name: Box<str>,
}

#[derive(Debug)]
pub struct Range {
  pub start: u32,
  pub end: u32,
}

pub struct EnumData {
  pub name: Box<str>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Vec<ProtoOption>,
  pub variants: Vec<(u32, String)>,
  pub file_id: usize,
  pub parent_message_id: Option<usize>,
}

impl<S: message_data_builder::State> MessageDataBuilder<S>
where
  S::Name: IsSet,
  S::Id: IsSet,
  S::Arena: IsSet,
{
  pub fn get_field_type(&self) -> FieldType {
    FieldType::Message {
      name: self.get_name().to_string(),
      id: *self.get_id(),
    }
  }
  pub fn field(
    mut self,
    new_field: FieldDataBuilder<field_data_builder::SetOptions<SetTag<SetFieldType<SetName>>>>,
  ) -> Self {
    self
      .fields
      .push(new_field.message(self.get_name().to_string()).build());
    self
  }
}

#[derive(Builder, Default, Debug)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct MessageData {
  #[builder(field)]
  pub fields: Vec<FieldData>,
  #[builder(getter(name = get_arena, vis = ""))]
  arena: Arena,
  #[builder(getter(name = get_id, vis = ""))]
  pub id: usize,
  pub package_id: usize,
  #[builder(getter(name = get_name, vis = ""))]
  pub name: Box<str>,
  pub reserved_numbers: Option<Box<[u32]>>,
  pub reserved_ranges: Option<Box<[Range]>>,
  pub reserved_names: Option<Box<[Box<str>]>>,
  pub options: Option<Vec<ProtoOption>>,

  pub enums: Option<Vec<usize>>,
  pub file: String,
}

impl<S: message_data_builder::IsComplete> MessageDataBuilder<S> {
  pub fn build(self, package: &Package)
  where
    <S as message_data_builder::State>::Arena: IsSet,
    <S as message_data_builder::State>::Id: IsSet,
  {
    let mut arena = package.arena.borrow_mut();
    let id = &self.get_id().clone();
    arena.messages[*id] = self.build_internal();
  }
}

#[derive(Builder, Debug)]
pub struct FieldData {
  message: String,
  name: Box<str>,
  field_type: FieldType,
  tag: u32,
  options: Vec<ProtoOption>,
}

impl MessageData {
  pub fn get_message_type(&self) {
    for field in &self.fields {
      if let FieldType::Message { name: _, id } = field.field_type {
        let msg_data = &self.arena.borrow().messages[id];
        println!("{:#?}", msg_data);
      }
    }
  }
}
