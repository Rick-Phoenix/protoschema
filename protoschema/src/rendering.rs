use std::{collections::HashSet, ops::Range, sync::Arc};

use askama::Template;
use convert_case::{Case, Casing};

use crate::{
  enums::{EnumData, EnumVariant},
  extensions::ExtensionData,
  fields::FieldData,
  files::FileData,
  messages::MessageData,
  oneofs::OneofData,
  packages::PackageData,
  services::ServiceData,
  ProtoOption,
};

/// The struct containing all of the processed data for a protobuf file template
#[derive(Debug, Clone, Template, Default)]
#[template(path = "file.proto.j2")]
pub struct FileTemplate {
  pub name: Arc<str>,
  pub imports: HashSet<Arc<str>>,
  pub package: Arc<str>,
  pub messages: Vec<MessageTemplate>,
  pub enums: Vec<EnumTemplate>,
  pub services: Vec<ServiceData>,
  pub extensions: Vec<ExtensionData>,
  pub options: Box<[ProtoOption]>,
}

/// The struct containing all of the processed data for a protobuf message template
#[derive(Clone, Debug, Default, Template)]
#[template(path = "message.proto.j2")]
pub struct MessageTemplate {
  pub name: Arc<str>,
  pub package: Arc<str>,
  pub file: Arc<str>,
  pub fields: Box<[(u32, FieldData)]>,
  pub messages: Vec<MessageTemplate>,
  pub oneofs: Box<[OneofData]>,
  pub enums: Vec<EnumTemplate>,
  pub options: Box<[ProtoOption]>,
  pub reserved_names: Box<[Box<str>]>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range<u32>]>,
}

/// The struct containing all of the processed data for a protobuf enum template
#[derive(Clone, Debug, Default, Template)]
#[template(path = "enum.proto.j2")]
pub struct EnumTemplate {
  pub name: Arc<str>,
  pub variants: Box<[(i32, EnumVariant)]>,
  pub reserved_numbers: Box<[i32]>,
  pub reserved_ranges: Box<[Range<i32>]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Box<[ProtoOption]>,
}

impl From<EnumData> for EnumTemplate {
  fn from(mut value: EnumData) -> Self {
    value.variants.sort_by_key(|t| t.0);

    let full_variants: Vec<(i32, EnumVariant)> = value
      .variants
      .into_iter()
      .map(|(tag, mut variant)| {
        variant.name = format!("{}_{}", value.name.to_case(Case::UpperSnake), variant.name).into();
        (tag, variant)
      })
      .collect();

    EnumTemplate {
      name: value.name.clone(),
      variants: full_variants.into_boxed_slice(),
      reserved_numbers: value.reserved_numbers,
      reserved_ranges: value.reserved_ranges,
      reserved_names: value.reserved_names,
      options: value.options,
    }
  }
}

impl FileData {
  pub(crate) fn build_template(&self, package: &PackageData) -> FileTemplate {
    let mut imports = self.imports.clone();

    let file_messages: Vec<MessageTemplate> = self
      .messages
      .iter()
      .map(|id| {
        let msg = &package.messages[*id];
        msg.imports.iter().for_each(|i| {
          if i.as_ref() != self.name.as_ref() {
            imports.insert(i.clone());
          };
        });
        msg.build_template(package)
      })
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

    FileTemplate {
      name: self.name.clone(),
      package: package.name.clone(),
      messages: file_messages,
      options: self.options.clone().into_boxed_slice(),
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

    let enums: Vec<EnumTemplate> = self
      .enums
      .iter()
      .map(|id| package.enums[*id].clone().into())
      .collect();

    MessageTemplate {
      name: self.name.clone(),
      package: self.import_path.package.clone(),
      file: self.import_path.file.clone(),
      fields: self.fields.clone(),
      oneofs: self.oneofs.clone().into_boxed_slice(),
      options: self.options.clone().into_boxed_slice(),
      messages: built_messages,
      enums,
      reserved_names: self.reserved_names.clone(),
      reserved_numbers: self.reserved_numbers.clone(),
      reserved_ranges: self.reserved_ranges.clone(),
    }
  }
}
