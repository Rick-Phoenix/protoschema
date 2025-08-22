use std::{ops::Range, sync::Arc};

use askama::Template;

use crate::{
  enums::EnumData, extensions::ExtensionData, fields::FieldData, files::FileData,
  message::MessageData, oneofs::OneofData, schema::PackageData, services::ServiceData, ProtoOption,
};

#[derive(Debug, Clone, Template, Default)]
#[template(path = "file.proto.j2")]
pub struct FileTemplate {
  pub name: Arc<str>,
  pub imports: Vec<Arc<str>>,
  pub package: Arc<str>,
  pub messages: Vec<MessageTemplate>,
  pub enums: Vec<EnumTemplate>,
  pub services: Vec<ServiceData>,
  pub extensions: Vec<ExtensionData>,
}

#[derive(Clone, Debug, Default, Template)]
#[template(path = "message.proto.j2")]
pub struct MessageTemplate {
  pub name: Arc<str>,
  pub package: Arc<str>,
  pub parent_message_name: Option<Arc<str>>,
  pub fields: Box<[FieldData]>,
  pub messages: Vec<MessageTemplate>,
  pub oneofs: Box<[OneofData]>,
  pub enums: Vec<EnumTemplate>,
}

#[derive(Clone, Debug, Default, Template)]
#[template(path = "enum.proto.j2")]
pub struct EnumTemplate {
  pub name: Arc<str>,
  pub variants: Box<[(i32, Box<str>)]>,
  pub reserved_numbers: Box<[i32]>,
  pub reserved_ranges: Box<[Range<i32>]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Box<[ProtoOption]>,
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

impl FileData {
  pub(crate) fn build_template(&self, package: &PackageData) -> FileTemplate {
    let file_messages: Vec<MessageTemplate> = self
      .messages
      .iter()
      .map(|id| package.messages[*id].build_template(package))
      .collect();

    let built_enums: Vec<EnumTemplate> = self
      .enums
      .iter()
      .map(|id| package.enums[*id].clone().into())
      .collect();

    let services: Vec<ServiceData> = self
      .services
      .iter()
      .map(|id| package.services[*id].clone())
      .collect();

    let imports: Vec<Arc<str>> = self.imports.iter().cloned().collect();

    FileTemplate {
      name: self.name.clone(),
      package: package.name.clone(),
      messages: file_messages,
      imports,
      extensions: self.extensions.clone(),
      enums: built_enums,
      services,
    }
  }
}

impl MessageData {
  pub(crate) fn build_template(&self, package: &PackageData) -> MessageTemplate {
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
      .map(|id| package.messages[id].full_name.clone());

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
