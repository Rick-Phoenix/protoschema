use std::marker::PhantomData;

use crate::{
  fields::{self, Field, FieldBuilder},
  Arena, ProtoOption, Range, Set, Unset,
};
mod sealed {
  pub struct Sealed;
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
{
  const SEALED: sealed::Sealed = sealed::Sealed;
}
pub struct SetFields<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNumbers<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedRanges<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNames<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetOptions<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetEnums<S: MessageState = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl MessageState for Empty {
  type Fields = Unset<members::fields>;
  type ReservedNumbers = Unset<members::reserved_numbers>;
  type ReservedRanges = Unset<members::reserved_ranges>;
  type ReservedNames = Unset<members::reserved_names>;
  type Options = Unset<members::options>;
  type Enums = Unset<members::enums>;
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
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[derive(Clone, Debug, Default)]
pub struct MessageData {
  pub file_id: usize,
  pub name: String,
  pub fields: Vec<Field>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Vec<Range>,
  pub reserved_names: Vec<String>,
  pub options: Vec<ProtoOption>,
  pub enums: Vec<usize>,
}

#[allow(non_camel_case_types)]
mod members {
  pub struct fields;
  pub struct reserved_numbers;
  pub struct reserved_ranges;
  pub struct reserved_names;
  pub struct options;
  pub struct enums;
}

pub trait MessageState: Sized {
  type Fields;
  type ReservedNumbers;
  type ReservedRanges;
  type ReservedNames;
  type Options;
  type Enums;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

pub trait IsSet {}
pub trait IsUnset {}

impl<T> IsSet for Set<T> {}
impl<T> IsUnset for Unset<T> {}

pub struct Empty;

#[derive(Clone, Debug)]
pub struct MessageBuilder<S: MessageState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

impl<S: MessageState> MessageBuilder<S>
where
  S::Fields: IsSet,
{
  pub fn get_data(self) -> MessageData {
    let arena = self.arena.borrow();
    arena.messages[self.id].clone()
  }
}

impl<S: MessageState> MessageBuilder<S> {
  pub fn get_name(&self) -> String {
    let arena = self.arena.borrow();

    arena.messages[self.id].name.clone()
  }
  pub fn reserved_numbers(self, numbers: &[u32]) -> MessageBuilder<SetReservedNumbers>
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
  pub fn reserved_names(self, names: &[&str]) -> MessageBuilder<SetReservedNames>
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
  pub fn reserved_ranges(self, ranges: &[Range]) -> MessageBuilder<SetReservedRanges>
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
  pub fn fields<F>(self, fields: F) -> MessageBuilder<SetFields<S>>
  where
    F: IntoIterator<Item = FieldBuilder<fields::SetTag<fields::SetFieldType<fields::SetName>>>>,
    S::Fields: IsUnset,
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
      _phantom: PhantomData,
    }
  }
}
