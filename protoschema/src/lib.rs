#![allow(dead_code)]

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use bon::Builder;

pub use crate::{
  field_type::FieldType,
  option::{OptionValue, ProtoOption},
};

#[macro_use]
mod macros;
mod field_type;
#[macro_use]
mod option;

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
pub struct Set;
#[derive(Clone, Debug)]
pub struct Unset;

pub struct FileBuilder {
  id: usize,
  arena: Arena,
}

impl FileBuilder {
  pub fn new_message(&self, name: &str) -> MessageBuilder<Unset> {
    let mut arena = self.arena.borrow_mut();
    let msg_id = arena.messages.len();

    arena.messages.push(MessageData {
      file_id: self.id,
      name: name.into(),
      fields: vec![],
    });

    MessageBuilder {
      id: msg_id,
      arena: self.arena.clone(),
      _fields_state: PhantomData,
    }
  }
}

#[derive(Clone, Debug)]
pub struct FileData {
  pub name: Box<str>,
  pub messages: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct MessageData {
  pub file_id: usize,
  pub name: String,
  pub fields: Vec<Field>,
}

pub trait MessageState {
  type Fields;
}

pub trait IsComplete: MessageState {}

#[derive(Clone, Debug)]
pub struct MessageBuilder<FieldsState> {
  id: usize,
  arena: Arena,
  _fields_state: PhantomData<FieldsState>,
}

#[derive(Clone, Debug, Builder)]
pub struct Field {
  pub parent_message_id: usize,
  pub name: Box<str>,
  pub tag: u32,
  pub field_type: FieldType,
  #[builder(default)]
  pub options: Vec<ProtoOption>,
}

impl MessageBuilder<Set> {
  pub fn build(self) -> MessageData {
    let arena = self.arena.borrow();
    arena.messages[self.id].clone()
  }
}

impl MessageBuilder<Unset> {
  pub fn name(&self) -> String {
    let arena = self.arena.borrow();

    arena.messages[self.id].name.clone()
  }
  pub fn fields<F>(self, fields: F) -> MessageBuilder<Set>
  where
    F: IntoIterator<
      Item = FieldBuilder<
        field_builder::SetOptions<
          field_builder::SetTag<field_builder::SetFieldType<field_builder::SetName>>,
        >,
      >,
    >,
  {
    let final_fields: Vec<Field> = fields
      .into_iter()
      .map(|f| f.parent_message_id(self.id).build())
      .collect();

    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.fields = final_fields;
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _fields_state: PhantomData,
    }
  }
}
