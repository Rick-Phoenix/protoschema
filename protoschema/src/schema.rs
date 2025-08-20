use std::{cell::RefCell, collections::HashSet, marker::PhantomData, rc::Rc};

use crate::{
  enums::{EnumBuilder, EnumData},
  message::{MessageBuilder, MessageData},
  rendering::{EnumTemplate, FileTemplate, MessageTemplate},
};

pub(crate) type Arena = Rc<RefCell<PackageData>>;

#[derive(Default, Debug)]
pub(crate) struct PackageData {
  pub(crate) name: String,
  pub(crate) files: Vec<FileData>,
  pub(crate) messages: Vec<MessageData>,
  pub(crate) enums: Vec<EnumData>,
}

#[derive(Clone)]
pub struct Package {
  arena: Arena,
}

impl Package {
  pub fn build_templates(&self) -> Vec<FileTemplate> {
    let mut templates: Vec<FileTemplate> = Vec::new();
    let arena = self.arena.borrow();
    let package = arena.name.clone();

    for file in &arena.files {
      let file_messages: Vec<MessageTemplate> = file
        .messages
        .iter()
        .map(|id| {
          let msg = &arena.messages[*id];

          msg.build_template(&self.arena.borrow())
        })
        .collect();

      let built_enums: Vec<EnumTemplate> = file
        .enums
        .iter()
        .map(|id| arena.enums[*id].clone().into())
        .collect();

      templates.push(FileTemplate {
        name: file.name.clone(),
        package: package.clone(),
        messages: file_messages,
        imports: file.imports.clone(),
        enums: built_enums,
      });
    }

    templates
  }

  pub fn new(name: &str) -> Self {
    Package {
      arena: Rc::new(RefCell::new(PackageData {
        name: name.to_string(),
        ..Default::default()
      })),
    }
  }
}

impl Package {
  pub fn new_file(&self, name: &str) -> FileBuilder {
    let mut arena = self.arena.borrow_mut();
    let file_id = arena.files.len();

    arena.files.push(FileData {
      name: name.into(),
      ..Default::default()
    });
    FileBuilder {
      id: file_id,
      arena: self.arena.clone(),
    }
  }
}

pub struct FileBuilder {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
}

impl FileBuilder {
  pub fn get_data(&self) -> FileData {
    self.arena.borrow().files[self.id].clone()
  }

  pub fn get_name(&self) -> String {
    self.arena.borrow().files[self.id].name.to_string()
  }

  pub fn new_enum(&self, name: &str) -> EnumBuilder {
    let file = self.get_name();
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let enum_id = arena.messages.len();

    arena.files[self.id].enums.push(enum_id);

    arena.enums.push(EnumData {
      file,
      name: name.into(),
      package: package_name,
      ..Default::default()
    });

    EnumBuilder {
      id: enum_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }

  pub fn new_message(&self, name: &str) -> MessageBuilder {
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let msg_id = arena.messages.len();

    arena.files[self.id].messages.push(msg_id);

    arena.messages.push(MessageData {
      file_id: self.id,
      name: name.into(),
      package: package_name,
      ..Default::default()
    });

    MessageBuilder {
      id: msg_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Clone, Debug, Default)]
pub struct FileData {
  pub name: Box<str>,
  pub messages: Vec<usize>,
  pub enums: Vec<usize>,
  pub imports: HashSet<Box<str>>,
}
