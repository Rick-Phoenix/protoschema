use std::{cell::RefCell, collections::HashSet, marker::PhantomData, rc::Rc};

use crate::{
  enums::EnumData,
  message::{MessageBuilder, MessageData},
};

pub(crate) type Arena = Rc<RefCell<SchemaInner>>;

#[derive(Default, Debug)]
pub(crate) struct SchemaInner {
  pub(crate) files: Vec<FileData>,
  pub(crate) messages: Vec<MessageData>,
  pub(crate) enums: Vec<EnumData>,
}

#[derive(Clone)]
pub struct Package {
  name: String,
  arena: Arena,
}

impl Package {
  pub fn new(name: &str) -> Self {
    Package {
      name: name.to_string(),
      arena: Rc::new(RefCell::new(Default::default())),
    }
  }
}

impl Package {
  pub fn new_file(&self, name: &str) -> FileBuilder {
    let mut arena = self.arena.borrow_mut();
    let file_id = arena.files.len();

    arena.files.push(FileData {
      name: name.into(),
      package: self.name.clone(),
      ..Default::default()
    });
    FileBuilder {
      id: file_id,
      arena: self.arena.clone(),
      name: name.to_string(),
    }
  }
}

pub struct FileBuilder {
  id: usize,
  name: String,
  arena: Arena,
}

impl FileBuilder {
  pub fn new_message(&self, name: &str) -> MessageBuilder {
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.files[self.id].package.clone();
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
  pub package: String,
  pub messages: Vec<usize>,
  pub enums: Vec<usize>,
  pub imports: HashSet<Box<str>>,
}
