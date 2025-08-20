use std::{marker::PhantomData, ops::Range};

use crate::{
  from_str_slice, schema::Arena, sealed, Empty, IsSet, IsUnset, ProtoOption, Set, Unset,
};

#[derive(Clone, Debug)]
pub struct EnumBuilder<S: EnumState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

#[derive(Clone, Debug, Default)]
pub struct EnumData {
  pub name: Box<str>,
  pub variants: Box<[(i32, Box<str>)]>,
  pub file_id: usize,
  pub package: Box<str>,
  pub parent_message: Option<usize>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range<i32>]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Box<[ProtoOption]>,
}

impl<S: EnumState> EnumBuilder<S> {
  pub fn options(self, options: &[ProtoOption]) -> EnumBuilder<SetOptions<S>>
  where
    S::Options: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.options = options.into()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn get_name(&self) -> Box<str> {
    let arena = self.arena.borrow();

    arena.enums[self.id].name.clone()
  }

  pub fn get_package(&self) -> Box<str> {
    self.arena.borrow().name.clone()
  }

  pub fn get_full_name(&self) -> String {
    let arena = self.arena.borrow();

    let enum_ = &arena.enums[self.id];

    match enum_.parent_message {
      Some(id) => format!("{}.{}", arena.messages[id].full_name, self.get_name()),
      None => format!("{}.{}", self.get_package(), self.get_name()),
    }
  }

  pub fn reserved_numbers(self, numbers: &[u32]) -> EnumBuilder<SetReservedNumbers<S>>
  where
    S::ReservedNumbers: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.reserved_numbers = numbers.into()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
  pub fn reserved_names(self, names: &[&str]) -> EnumBuilder<SetReservedNames<S>>
  where
    S::ReservedNames: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.reserved_names = from_str_slice(names)
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
  pub fn reserved_ranges(self, ranges: &[Range<i32>]) -> EnumBuilder<SetReservedRanges<S>>
  where
    S::ReservedRanges: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.reserved_ranges = ranges.into()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
  pub fn variants(self, variants: &[(i32, Box<str>)]) -> EnumBuilder<SetVariants<S>>
  where
    S::Variants: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let enum_ = &mut arena.enums[self.id];

      enum_.variants = variants.into();
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
}

pub trait EnumState: Sized {
  type Variants;
  type ReservedNumbers;
  type ReservedRanges;
  type ReservedNames;
  type Options;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

#[allow(non_camel_case_types)]
mod members {
  pub struct variants;
  pub struct reserved_numbers;
  pub struct reserved_ranges;
  pub struct reserved_names;
  pub struct options;
}

pub trait IsComplete: EnumState {
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}
#[doc(hidden)]
impl<S: EnumState> IsComplete for S
where
  S::Variants: IsSet,
  S::ReservedNumbers: IsSet,
  S::ReservedRanges: IsSet,
  S::ReservedNames: IsSet,
  S::Options: IsSet,
{
  const SEALED: sealed::Sealed = sealed::Sealed;
}

pub struct SetVariants<S: EnumState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNumbers<S: EnumState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedRanges<S: EnumState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNames<S: EnumState = Empty>(PhantomData<fn() -> S>);
pub struct SetOptions<S: EnumState = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl EnumState for Empty {
  type Variants = Unset<members::variants>;
  type ReservedNumbers = Unset<members::reserved_numbers>;
  type ReservedRanges = Unset<members::reserved_ranges>;
  type ReservedNames = Unset<members::reserved_names>;
  type Options = Unset<members::options>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: EnumState> EnumState for SetVariants<S> {
  type Variants = Set<members::variants>;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: EnumState> EnumState for SetReservedNumbers<S> {
  type Variants = S::Variants;
  type ReservedNumbers = Set<members::reserved_numbers>;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: EnumState> EnumState for SetReservedRanges<S> {
  type Variants = S::Variants;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = Set<members::reserved_ranges>;
  type ReservedNames = S::ReservedNames;
  type Options = S::Options;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: EnumState> EnumState for SetReservedNames<S> {
  type Variants = S::Variants;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = Set<members::reserved_names>;
  type Options = S::Options;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: EnumState> EnumState for SetOptions<S> {
  type Variants = S::Variants;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Options = Set<members::options>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
