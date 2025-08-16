#![allow(dead_code)]

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::message::{Empty, MessageBuilder, MessageData};
pub use crate::{
  field_type::FieldType,
  option::{OptionValue, ProtoOption},
};

pub mod fields;
mod message;
#[macro_use]
mod macros;
mod field_type;
#[macro_use]
mod option;

#[derive(Clone, Debug)]
pub struct Range {
  pub start: u32,
  pub end: u32,
}

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
  pub fn new_file(&self, name: &str) -> FileBuilder {
    let mut arena = self.arena.borrow_mut();
    let file_id = arena.files.len();

    arena.files.push(FileData {
      name: name.into(),
      messages: vec![],
    });
    FileBuilder {
      id: file_id,
      arena: self.arena.clone(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Set<T>(PhantomData<fn() -> T>);
#[derive(Clone, Debug)]
pub struct Unset<T>(PhantomData<fn() -> T>);

pub struct FileBuilder {
  id: usize,
  arena: Arena,
}

impl FileBuilder {
  pub fn new_message(&self, name: &str) -> MessageBuilder<Empty> {
    let mut arena = self.arena.borrow_mut();
    let msg_id = arena.messages.len();

    arena.messages.push(MessageData {
      file_id: self.id,
      name: name.into(),
      ..Default::default()
    });

    MessageBuilder {
      id: msg_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Clone, Debug)]
pub struct FileData {
  pub name: Box<str>,
  pub messages: Vec<usize>,
}
