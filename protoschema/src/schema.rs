use std::{cell::RefCell, collections::HashSet, marker::PhantomData, rc::Rc};

use askama::Template;

use crate::{
  enums::EnumData,
  message::{MessageBuilder, MessageData},
};

pub(crate) type Arena = Rc<RefCell<PackageData>>;

#[derive(Default, Debug, Template)]
#[template(path = "template.proto.j2")]
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
  pub fn render(&self) -> String {
    self.arena.borrow().render().unwrap()
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
  id: usize,
  arena: Arena,
}

impl FileBuilder {
  pub fn new_message(&self, name: &str) -> MessageBuilder {
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let msg_id = arena.messages.len();

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
