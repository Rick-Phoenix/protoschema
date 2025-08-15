#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use bon::{builder, Builder};
use paste::paste;

use crate::{
  field_data_builder::{SetName, SetTag, SetTy},
  message_data_builder::IsSet,
};
pub use crate::{
  field_type::FieldType,
  option::{OptionValue, ProtoOption},
};

mod field_type;
#[macro_use]
mod option;

type Arena = Rc<RefCell<SchemaInner>>;

#[derive(Default)]
struct SchemaInner {
  pub(crate) files: Vec<FileData>,
  pub(crate) messages: Vec<MessageData>,
}

#[derive(Clone, Default)]
pub struct Schema {
  arena: Arena,
}

impl Schema {
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
{
  fn add_message(
    &self,
    name: &str,
  ) -> MessageDataBuilder<message_data_builder::SetFile<message_data_builder::SetName>> {
    let mut arena = self.get_arena().borrow_mut();
    let msg_id = arena.messages.len();
    arena.messages.push(MessageData::default());
    let data = &mut arena.files[*self.get_id()];
    data.messages.push(msg_id);

    MessageData::builder()
      .name(name.into())
      .file(data.name.to_string())
  }
}

#[derive(Default, Builder)]
pub struct FileData {
  #[builder(getter(name = get_id, vis = ""))]
  pub id: usize,
  #[builder(getter(name = get_arena, vis = ""))]
  pub arena: Arena,
  pub name: Box<str>,
  pub options: Vec<ProtoOption>,
  pub messages: Vec<usize>,
  pub imports: Vec<String>,
  pub enums: Vec<usize>,
  pub package_id: String,
}

impl<S: package_data_builder::State> PackageDataBuilder<S>
where
  S::Name: IsSet,
{
  fn new_file(mut self) -> Self {}
}

#[derive(Builder)]
pub struct PackageData {
  #[builder(field)]
  pub files: Vec<FileData>,
  pub name: Box<str>,
}

impl PackageData {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      files: vec![],
    }
  }

  pub fn new_file(&self, name: &str) -> FileData {
    FileData {
      name: name.into(),
      options: vec![],
      messages: vec![],
      imports: vec![],
      enums: vec![],
      package: self.name.clone().into(),
    }
  }
}

impl FileData {
  pub fn new_message(
    &self,
    name: &str,
  ) -> MessageDataBuilder<message_data_builder::SetFile<message_data_builder::SetName>> {
    MessageData::builder()
      .name(name.into())
      .file(self.name.to_string())
  }
}

macro_rules! string {
  ($field_name:ident = $tag:literal $(, [$($option_name:expr),*])? $(,)?) => {
    field!(string $field_name = $tag $(, [$($option_name),*])?)
  };
}

macro_rules! field {
  ($field_type:ident $field_name:ident = $tag:literal $(, [$($option_name:expr),*])? $(,)?) => {
    FieldData::builder().name(stringify!($field_name).into()).ty(parse_field_type!($field_type)).tag($tag).options(vec![
      $($($option_name),*)?
    ])
  };
}

macro_rules! add_field {
  ($current_builder:expr, $field_def:expr) => {
    $current_builder.field($field_def)
  };

  ($current_builder:expr, $head_field:expr, $($tail_fields:expr),* $(,)?) =>  {
    add_field!(
      $current_builder.field($head_field),
      $($tail_fields),*
    )
  };
}

macro_rules! message_fields {
  ($message:ident, [$head_field:expr, $($tail_fields:expr),* ] $(,)?) => {
    add_field!($message.field($head_field), $($tail_fields),*)
  };
}

macro_rules! message {
  ($file:ident, $name:literal, [$head_field:expr, $($tail_fields:expr),* ] $(,)?) => {
    {
      let msg = $file.new_message($name);
      message_fields!(msg, [ $head_field, $($tail_fields),* ])
    }
  };
}

macro_rules! parse_field_type {
  ($ty:ident) => {
    paste! {
      FieldType::[< $ty:camel >]
    }
  };
}

fn example() {
  let schema = Schema::default();

  let file = schema.new_file("abc");

  let pac = PackageData::new("abc");

  let file = pac.new_file("abc");

  let opt = ProtoOption {
    name: "abc",
    value: OptionValue::Bool(true),
  };

  let msg = message!(
    file,
    "MyMsg",
    [
      string!(abc = 5),
      string!(abc = 5),
      string!(abc = 5),
      string!(abc = 5, [opt.clone(), opt])
    ]
  );

  let field = string!(abc = 5);

  let built = msg.build();
}

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
{
  fn field(
    mut self,
    new_field: FieldDataBuilder<field_data_builder::SetOptions<SetTag<SetTy<SetName>>>>,
  ) -> Self {
    self
      .fields
      .push(new_field.message(self.get_name().to_string()).build());
    self
  }
}

#[derive(Builder, Default)]
pub struct MessageData {
  #[builder(field)]
  pub fields: Vec<FieldData>,
  pub arena: Arena,
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

#[derive(Builder)]
pub struct FieldData {
  message: String,
  name: Box<str>,
  ty: FieldType,
  tag: u32,
  options: Vec<ProtoOption>,
}
