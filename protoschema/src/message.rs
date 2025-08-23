use std::{marker::PhantomData, ops::Range, sync::Arc};

use crate::{
  enums::{EnumBuilder, EnumData},
  fields::{self, FieldBuilder, FieldData},
  oneofs::{Oneof, OneofData},
  rendering::MessageTemplate,
  schema::{Arena, PackageData},
  sealed, Empty, FieldType, IsSet, IsUnset, ProtoOption, Set, Unset,
};

#[derive(Clone, Debug)]
pub struct MessageBuilder<S: MessageState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) file_id: usize,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

impl PackageData {
  pub fn get_full_message_name(
    &self,
    base_name: &str,
    parent_message_id: Option<usize>,
  ) -> Arc<str> {
    let mut path = String::new();

    match parent_message_id {
      Some(id) => {
        let mut current_id = Some(id);
        path.push_str(base_name);

        while let Some(id) = current_id {
          let current_message = &self.messages[id];
          path.insert(0, '.');
          path.insert_str(0, &current_message.name);

          current_id = current_message.parent_message;
        }
      }
      None => path.push_str(base_name),
    }

    path.insert(0, '.');
    path.insert_str(0, &self.name);

    path.into()
  }
}

#[derive(Clone, Debug, Default)]
pub struct MessageData {
  pub name: Arc<str>,
  pub full_name: Arc<str>,
  pub package: Arc<str>,
  pub parent_message: Option<usize>,
  pub fields: Box<[FieldData]>,
  pub oneofs: Box<[OneofData]>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range<u32>]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Box<[ProtoOption]>,
  pub enums: Vec<usize>,
  pub messages: Vec<usize>,
  pub imports: Vec<Arc<str>>,
}

impl<S: MessageState> MessageBuilder<S> {
  // Getters
  pub fn get_type(&self) -> FieldType {
    let name = self.get_full_name();
    FieldType::Message(name)
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_data(self) -> MessageTemplate
  where
    S::Fields: IsSet,
  {
    let arena = self.arena.borrow();
    arena.messages[self.id].build_template(&arena)
  }

  pub fn get_name(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.messages[self.id].name.clone()
  }

  pub fn get_full_name(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    let msg = &arena.messages[self.id];
    msg.full_name.clone()
  }

  pub fn get_package(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.messages[self.id].package.clone()
  }

  pub fn get_file(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.files[self.file_id].name.clone()
  }

  // Setters
  pub fn new_message(&self, name: &str) -> MessageBuilder {
    let file_id = self.file_id;
    let package = self.get_package();
    let mut arena = self.arena.borrow_mut();

    let parent_message_id = self.id;
    let child_message_id = arena.messages.len();

    arena.messages[parent_message_id]
      .messages
      .push(child_message_id);

    let full_name = arena.get_full_message_name(name, Some(parent_message_id));

    let new_msg = MessageData {
      name: name.into(),
      package,
      full_name,
      parent_message: Some(parent_message_id),
      ..Default::default()
    };

    arena.messages.push(new_msg);

    MessageBuilder {
      id: child_message_id,
      arena: self.arena.clone(),
      file_id,
      _phantom: PhantomData,
    }
  }

  pub fn new_enum(&self, name: &str) -> EnumBuilder {
    let package = self.get_package();
    let parent_message_full_name = self.get_full_name();
    let mut arena = self.arena.borrow_mut();

    let file_id = self.file_id;
    let parent_message_id = self.id;
    let new_enum_id = arena.enums.len();

    arena.messages[parent_message_id].enums.push(new_enum_id);

    let new_enum = EnumData {
      name: name.into(),
      full_name: format!("{}.{}", parent_message_full_name, name).into(),
      package,
      file_id,
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

  pub fn fields<I, F>(self, fields: I) -> MessageBuilder<SetFields<S>>
  where
    S::Fields: IsUnset,
    I: IntoIterator<Item = FieldBuilder<F>>,
    F: fields::IsComplete,
  {
    {
      let mut arena = self.arena.borrow_mut();

      let final_fields: Vec<FieldData> = fields
        .into_iter()
        .map(|field| {
          let field = field.build();
          let file_id = self.file_id;

          for import in &field.imports {
            arena.files[file_id].imports.insert(import.clone());
          }

          FieldData {
            name: field.name.clone(),
            tag: field.tag,
            options: field.options.into_boxed_slice(),
            kind: field.kind,
            field_type: field.field_type,
          }
        })
        .collect();

      arena.messages[self.id].fields = final_fields.into_boxed_slice()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  pub fn oneofs<I>(self, oneofs: I) -> MessageBuilder<SetOneofs<S>>
  where
    S::Oneofs: IsUnset,
    I: IntoIterator<Item = Oneof>,
  {
    {
      let mut arena = self.arena.borrow_mut();

      let oneofs_data: Vec<OneofData> = oneofs
        .into_iter()
        .map(|of| {
          let built_fields: Vec<FieldData> = of
            .fields
            .iter()
            .map(|f| {
              f.imports.iter().for_each(|i| {
                arena.files[self.file_id].imports.insert(i.clone());
              });

              FieldData {
                name: f.name.clone(),
                tag: f.tag,
                options: f.options.clone().into_boxed_slice(),
                kind: f.kind,
                field_type: f.field_type.clone(),
              }
            })
            .collect();

          OneofData {
            name: of.name.clone(),
            options: of.options.clone(),
            fields: built_fields.into_boxed_slice(),
          }
        })
        .collect();

      arena.messages[self.id].oneofs = oneofs_data.into_boxed_slice();
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  pub fn options<I>(self, options: I) -> MessageBuilder<SetOptions<S>>
  where
    S::Options: IsUnset,
    I: IntoIterator<Item = ProtoOption>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.options = options.into_iter().collect()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  pub fn reserved_names<I, Str>(self, names: I) -> MessageBuilder<SetReservedNames<S>>
  where
    S::ReservedNames: IsUnset,
    I: IntoIterator<Item = Str>,
    Str: AsRef<str>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];
      let reserved_names: Vec<Box<str>> = names.into_iter().map(|n| n.as_ref().into()).collect();

      msg.reserved_names = reserved_names.into_boxed_slice();
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  pub fn reserved_numbers<I>(self, numbers: I) -> MessageBuilder<SetReservedNumbers<S>>
  where
    S::ReservedNumbers: IsUnset,
    I: IntoIterator<Item = u32>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.reserved_numbers = numbers.into_iter().collect()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  pub fn reserved_ranges<I>(self, ranges: I) -> MessageBuilder<SetReservedRanges<S>>
  where
    S::ReservedRanges: IsUnset,
    I: IntoIterator<Item = Range<u32>>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.reserved_ranges = ranges.into_iter().collect()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
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
  pub struct oneofs;
}

pub trait MessageState: Sized {
  type Fields;
  type ReservedNumbers;
  type ReservedRanges;
  type ReservedNames;
  type Options;
  type Oneofs;
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
  S::Oneofs: IsSet,
{
  const SEALED: sealed::Sealed = sealed::Sealed;
}

pub struct SetFields<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNumbers<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedRanges<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNames<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetOptions<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetOneofs<S: MessageState = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl MessageState for Empty {
  type Fields = Unset<members::fields>;
  type ReservedNumbers = Unset<members::reserved_numbers>;
  type ReservedRanges = Unset<members::reserved_ranges>;
  type ReservedNames = Unset<members::reserved_names>;
  type Options = Unset<members::options>;
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
  type Oneofs = Set<members::oneofs>;

  const SEALED: sealed::Sealed = sealed::Sealed;
}
