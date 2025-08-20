use std::{
  collections::{BTreeMap, HashSet},
  ops::Range,
};

use askama::Template;

use crate::{
  enums::EnumData, fields::Field, message::MessageData, oneofs::OneofData, schema::PackageData,
  ProtoOption,
};

#[derive(Debug, Clone, Template, Default)]
#[template(path = "file.proto.j2")]
pub struct FileTemplate {
  pub name: Box<str>,
  pub imports: HashSet<Box<str>>,
  pub package: String,
  pub messages: Vec<MessageTemplate>,
  pub enums: Vec<EnumTemplate>,
}

#[derive(Clone, Debug, Default, Template)]
#[template(path = "message.proto.j2")]
pub struct MessageTemplate {
  pub name: String,
  pub package: String,
  pub parent_message_name: Option<String>,
  pub fields: Vec<Field>,
  pub messages: Vec<MessageTemplate>,
  pub oneofs: Vec<OneofData>,
  pub enums: Vec<EnumTemplate>,
}

#[derive(Clone, Debug, Default, Template)]
#[template(path = "enum.proto.j2")]
pub struct EnumTemplate {
  pub name: String,
  pub variants: BTreeMap<i32, String>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Vec<Range<i32>>,
  pub reserved_names: Vec<String>,
  pub options: Vec<ProtoOption>,
}

impl From<EnumData> for EnumTemplate {
  fn from(value: EnumData) -> Self {
    EnumTemplate {
      name: value.name,
      variants: value.variants,
      reserved_numbers: value.reserved_numbers,
      reserved_ranges: value.reserved_ranges,
      reserved_names: value.reserved_names,
      options: value.options,
    }
  }
}

impl MessageData {
  pub fn build_template(&self, package: &PackageData) -> MessageTemplate {
    let built_messages: Vec<MessageTemplate> = self
      .messages
      .iter()
      .map(|id| {
        let data = &package.messages[*id];

        data.build_template(package)
      })
      .collect();

    let parent_message_name = self
      .parent_message
      .map(|id| package.messages[id].get_full_name(package));

    let enums: Vec<EnumTemplate> = self
      .enums
      .iter()
      .map(|id| package.enums[*id].clone().into())
      .collect();

    MessageTemplate {
      name: self.name.clone(),
      package: self.package.clone(),
      fields: self.fields.clone(),
      parent_message_name,
      oneofs: self.oneofs.clone(),
      messages: built_messages,
      enums,
    }
  }
}
