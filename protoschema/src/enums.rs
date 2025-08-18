use std::{collections::BTreeMap, marker::PhantomData};

use crate::{schema::Arena, sealed, Empty, IsSet, IsUnset, ProtoOption, Range, Set, Unset};

#[derive(Clone, Debug)]
pub struct EnumBuilder<S: EnumState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

#[derive(Clone, Debug)]
pub struct EnumData {
  pub name: String,
  pub variants: BTreeMap<i32, String>,
  pub package: String,
  pub file: String,
  pub parent_message: Option<usize>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Vec<Range>,
  pub reserved_names: Vec<String>,
  pub options: Vec<ProtoOption>,
}

impl<S: EnumState> EnumBuilder<S> {
  pub fn get_name(&self) -> String {
    let arena = self.arena.borrow();

    arena.enums[self.id].name.clone()
  }

  pub fn reserved_numbers(self, numbers: &[u32]) -> EnumBuilder<SetReservedNumbers>
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
  pub fn reserved_names(self, names: &[&str]) -> EnumBuilder<SetReservedNames>
  where
    S::ReservedNames: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.reserved_names = names.iter().map(|n| n.to_string()).collect::<Vec<String>>()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
  pub fn reserved_ranges(self, ranges: &[Range]) -> EnumBuilder<SetReservedRanges>
  where
    S::ReservedRanges: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.reserved_ranges = ranges.to_vec()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
  pub fn variants<F>(self, variants: BTreeMap<i32, String>) -> EnumBuilder<SetVariants<S>>
  where
    S::Variants: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let enum_ = &mut arena.enums[self.id];

      enum_.variants = variants;
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
