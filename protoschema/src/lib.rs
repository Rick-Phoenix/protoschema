#![allow(dead_code)]

use bon::{builder, Builder};

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

pub struct FileData {
  pub name: Box<str>,
  pub options: Vec<ProtoOption>,
  pub messages: Vec<MessageData>,
  pub imports: Vec<String>,
  pub enums: Vec<EnumData>,
  pub package: String,
}

pub struct PackageData {
  pub name: Box<str>,
  pub files: Vec<FileData>,
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
  ($field_name:ident = $tag:literal $(, $($option_name:expr),*)? $(,)?) => {
    field!(string $field_name = $tag $(, $($option_name),*)?)
  };
}

macro_rules! field {
  ($field_type:ident $field_name:ident = $tag:literal $(, $($option_name:expr),*)? $(,)?) => {
    FieldData::builder().name(stringify!($field_name).into()).ty(stringify!($field_type).into()).tag($tag).options(vec![
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

fn example() {
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
      string!(abc = 5, opt.clone(), opt)
    ]
  );

  let built = msg.build();
}

impl MessageData {
  pub fn field(&self) -> FieldDataBuilder<field_data_builder::SetMessage> {
    FieldData::builder().message("abc".to_string())
  }
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

#[derive(Builder)]
pub struct MessageData {
  #[builder(field)]
  pub fields: Vec<FieldData>,
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
  pub message: String,
  pub name: Box<str>,
  pub ty: String,
  pub tag: u32,
  pub options: Vec<ProtoOption>,
}
