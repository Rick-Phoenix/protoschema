use std::{
  collections::{BTreeMap, HashSet},
  marker::PhantomData,
  ops::Range,
};

use crate::{
  enums::{EnumBuilder, EnumData},
  fields::{self, Field, FieldBuilder},
  oneofs::OneofData,
  schema::{Arena, PackageData},
  sealed, Empty, IsSet, IsUnset, ProtoOption, Set, Unset,
};

#[derive(Clone, Debug)]
pub struct MessageBuilder<S: MessageState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

impl MessageData {
  pub fn get_full_name(&self, arena: &PackageData) -> String {
    let mut path = String::new();

    match self.parent_message {
      Some(id) => {
        let mut current_id = Some(id);
        path.push_str(&self.name);

        while let Some(id) = current_id {
          let current_message = &arena.messages[id];
          path.insert(0, '.');
          path.insert_str(0, &current_message.name);

          current_id = current_message.parent_message;
        }
      }
      None => path.push_str(&self.name),
    }

    path.insert(0, '.');
    path.insert_str(0, &self.package);

    path
  }
}

#[derive(Clone, Debug, Default)]
pub struct MessageData {
  pub name: String,
  pub package: String,
  pub file_id: usize,
  pub parent_message: Option<usize>,
  pub fields: Vec<Field>,
  pub oneofs: Vec<OneofData>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Vec<Range<u32>>,
  pub reserved_names: Vec<String>,
  pub options: Vec<ProtoOption>,
  pub enums: Vec<usize>,
  pub messages: Vec<usize>,
  pub imports: HashSet<Box<str>>,
}

impl<S: MessageState> MessageBuilder<S> {
  pub fn new_enum(&self, name: &str) -> EnumBuilder {
    let file = self.get_file();
    let package = self.get_package();
    let mut arena = self.arena.borrow_mut();

    let parent_message_id = self.id;
    let new_enum_id = arena.enums.len();

    arena.messages[parent_message_id].enums.push(new_enum_id);

    let new_enum = EnumData {
      name: name.into(),
      package,
      file: file.clone(),
      parent_message: Some(parent_message_id),
      ..Default::default()
    };

    arena.enums.push(new_enum);

    EnumBuilder {
      id: new_enum_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }

  pub fn new_message(&self, name: &str) -> MessageBuilder {
    let file_id = self.get_file_id();
    let package = self.get_package();
    let mut arena = self.arena.borrow_mut();

    let parent_message_id = self.id;
    let child_message_id = arena.messages.len();

    arena.messages[parent_message_id]
      .messages
      .push(child_message_id);

    let new_msg = MessageData {
      name: name.into(),
      package,
      file_id,
      parent_message: Some(parent_message_id),
      ..Default::default()
    };

    arena.messages.push(new_msg);

    MessageBuilder {
      id: child_message_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_file_id(&self) -> usize {
    self.arena.borrow().messages[self.id].file_id
  }

  pub fn get_data(self) -> MessageData
  where
    S::Fields: IsSet,
  {
    let arena = self.arena.borrow();
    arena.messages[self.id].clone()
  }

  pub fn get_name(&self) -> String {
    let arena = self.arena.borrow();

    arena.messages[self.id].name.clone()
  }

  pub fn get_full_name(&self) -> String {
    let arena = self.arena.borrow();

    let msg = &arena.messages[self.id];
    msg.get_full_name(&arena)
  }

  pub fn get_package(&self) -> String {
    let arena = self.arena.borrow();

    arena.messages[self.id].package.clone()
  }

  pub fn get_file(&self) -> String {
    let arena = self.arena.borrow();

    let file_id = arena.messages[self.id].file_id;
    arena.files[file_id].name.to_string()
  }

  pub fn reserved_numbers(self, numbers: &[u32]) -> MessageBuilder<SetReservedNumbers<S>>
  where
    S::ReservedNumbers: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.reserved_numbers = numbers.into()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn options(self, options: Vec<ProtoOption>) -> MessageBuilder<SetOptions<S>>
  where
    S::Options: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.options = options
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn reserved_names(self, names: &[&str]) -> MessageBuilder<SetReservedNames<S>>
  where
    S::ReservedNames: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.reserved_names = names.iter().map(|n| n.to_string()).collect::<Vec<String>>()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn reserved_ranges(self, ranges: &[Range<u32>]) -> MessageBuilder<SetReservedRanges<S>>
  where
    S::ReservedRanges: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.reserved_ranges = ranges.to_vec()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn oneofs(self, oneofs: Vec<OneofData>) -> MessageBuilder<SetOneofs<S>>
  where
    S::Oneofs: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();

      arena.messages[self.id].oneofs = oneofs;
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn oneof(self, oneof: OneofData) -> MessageBuilder<S> {
    {
      let mut arena = self.arena.borrow_mut();

      arena.messages[self.id].oneofs.push(oneof);
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn fields(
    self,
    fields: BTreeMap<u32, FieldBuilder<fields::SetFieldType<fields::SetName>>>,
  ) -> MessageBuilder<SetFields<S>>
  where
    S::Fields: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();

      let final_fields: Vec<Field> = fields
        .into_iter()
        .map(|(tag, field)| {
          let field = field.tag(tag).build();
          let file_id = arena.messages[self.id].file_id;
          arena.files[file_id].imports.extend(field.imports.clone());

          field
        })
        .collect();

      arena.messages[self.id].fields = final_fields
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
}

#[allow(non_camel_case_types)]
mod members {
  pub struct fields;
  pub struct reserved_numbers;
  pub struct reserved_ranges;
  pub struct reserved_names;
  pub struct options;
  pub struct enums;
  pub struct oneofs;
  pub struct messages;
}

pub trait MessageState: Sized {
  type Fields;
  type ReservedNumbers;
  type ReservedRanges;
  type ReservedNames;
  type Options;
  type Enums;
  type Oneofs;
  type Messages;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

pub trait IsComplete: MessageState {
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> IsComplete for S
where
  S::Fields: IsSet,
  S::ReservedNumbers: IsSet,
  S::ReservedRanges: IsSet,
  S::ReservedNames: IsSet,
  S::Options: IsSet,
  S::Enums: IsSet,
  S::Messages: IsSet,
  S::Oneofs: IsSet,
{
  const SEALED: sealed::Sealed = sealed::Sealed;
}

pub struct SetFields<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNumbers<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedRanges<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNames<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetOptions<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetEnums<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetMessages<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetOneofs<S: MessageState = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl MessageState for Empty {
  type Fields = Unset<members::fields>;
  type ReservedNumbers = Unset<members::reserved_numbers>;
  type ReservedRanges = Unset<members::reserved_ranges>;
  type ReservedNames = Unset<members::reserved_names>;
  type Options = Unset<members::options>;
  type Enums = Unset<members::enums>;
  type Messages = Unset<members::messages>;
  type Oneofs = Unset<members::oneofs>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetFields<S> {
  type Fields = Set<members::fields>;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  type Enums = S::Enums;
  type Messages = S::Messages;
  type Oneofs = S::Oneofs;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetReservedNumbers<S> {
  type Fields = S::Fields;
  type ReservedNumbers = Set<members::reserved_numbers>;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  type Enums = S::Enums;
  type Messages = S::Messages;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}
#[doc(hidden)]
impl<S: MessageState> MessageState for SetReservedRanges<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = Set<members::reserved_ranges>;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  type Enums = S::Enums;
  type Messages = S::Messages;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}
#[doc(hidden)]
impl<S: MessageState> MessageState for SetReservedNames<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = Set<members::reserved_names>;
  type Options = S::Options;
  type Enums = S::Enums;
  type Messages = S::Messages;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}
#[doc(hidden)]
impl<S: MessageState> MessageState for SetOptions<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = Set<members::options>;
  type Enums = S::Enums;
  type Messages = S::Messages;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetEnums<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  type Enums = Set<members::enums>;
  type Messages = S::Messages;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetMessages<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  type Enums = S::Enums;
  type Messages = Set<members::messages>;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetOneofs<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  type Enums = S::Enums;
  type Messages = S::Messages;
  type Oneofs = Set<members::oneofs>;

  const SEALED: sealed::Sealed = sealed::Sealed;
}
