use std::collections::{BTreeMap, HashSet};

use askama::Template;

use crate::{fields::Field, message::MessageData, schema::PackageData};

#[derive(Debug, Clone, Template, Default)]
#[template(path = "file.proto.j2")]
pub struct FileTemplate {
  pub name: Box<str>,
  pub imports: HashSet<Box<str>>,
  pub package: String,
  pub messages: Vec<MessageTemplate>,
}

#[derive(Clone, Debug, Default, Template)]
#[template(path = "message.proto.j2")]
pub struct MessageTemplate {
  pub name: String,
  pub package: String,
  pub parent_message_name: Option<String>,
  pub fields: BTreeMap<u32, Field>,
  pub messages: Vec<MessageTemplate>,
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

    MessageTemplate {
      name: self.name.clone(),
      package: self.package.clone(),
      fields: self.fields.clone(),
      parent_message_name,
      messages: built_messages,
    }
  }
}
