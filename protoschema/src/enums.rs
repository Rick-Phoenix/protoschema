use std::{marker::PhantomData, ops::Range, sync::Arc};

use bon::Builder;

use crate::{
  field_type::ImportedItemPath, packages::Arena, rendering::EnumTemplate, sealed, Empty, FieldType,
  IsSet, IsUnset, ProtoOption, Set, Unset,
};

/// The builder for a protobuf enum. Its methods are used to collect and store the information for that enum, which are later used to build a template for it.
#[derive(Clone, Debug)]
pub struct EnumBuilder<S: EnumState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

/// A struct representing an enum value
#[derive(Clone, Debug, Default, Builder)]
pub struct EnumVariant {
  #[builder(into)]
  pub name: Box<str>,
  #[builder(into, default)]
  pub options: Box<[ProtoOption]>,
}

/// The stored information for a given enum.
#[derive(Clone, Debug, Default)]
pub struct EnumData {
  pub name: Arc<str>,
  pub variants: Box<[(i32, EnumVariant)]>,
  pub import_path: Arc<ImportedItemPath>,
  pub file_id: usize,
  pub reserved_numbers: Box<[i32]>,
  pub reserved_ranges: Box<[Range<i32>]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Box<[ProtoOption]>,
}

impl<S: EnumState> EnumBuilder<S> {
  /// Builds the full template for this enum and returns it.
  /// Mostly useful for debugging.
  pub fn get_data(&self) -> EnumTemplate {
    let arena = self.arena.borrow();
    arena.enums[self.id].clone().into()
  }

  #[doc(hidden)]
  pub fn get_type(&self) -> FieldType {
    FieldType::Enum(self.get_import_path())
  }

  /// Returns the import path for this enum
  pub fn get_import_path(&self) -> Arc<ImportedItemPath> {
    self.arena.borrow().enums[self.id].import_path.clone()
  }

  /// Returns the name of the file containing this enum
  pub fn get_file(&self) -> Arc<str> {
    let arena = self.arena.borrow();
    let file_id = arena.enums[self.id].file_id;
    arena.files[file_id].name.clone()
  }

  /// Returns the full name of this enum
  pub fn get_full_name(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.enums[self.id].import_path.full_name.clone()
  }

  /// Returns the name of the package containing this enum
  pub fn get_package(&self) -> Arc<str> {
    self.arena.borrow().name.clone()
  }

  /// Returns the full name of this enum, with the package prefix included
  pub fn get_full_name_with_package(&self) -> Arc<str> {
    self.arena.borrow().enums[self.id]
      .import_path
      .full_name_with_package
      .clone()
  }

  /// Sets the variants for this enum.
  pub fn variants<I>(self, variants: I) -> EnumBuilder<SetVariants<S>>
  where
    S::Variants: IsUnset,
    I: IntoIterator<Item = (i32, EnumVariant)>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let enum_ = &mut arena.enums[self.id];

      enum_.variants = variants.into_iter().collect();
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  /// Sets the options for this enum.
  pub fn options<I>(self, options: I) -> EnumBuilder<SetOptions<S>>
  where
    S::Options: IsUnset,
    I: IntoIterator<Item = ProtoOption>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.options = options.into_iter().collect()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  /// Sets the reserved names for this enum.
  pub fn reserved_names<I, Str>(self, names: I) -> EnumBuilder<SetReservedNames<S>>
  where
    S::ReservedNames: IsUnset,
    I: IntoIterator<Item = Str>,
    Str: AsRef<str>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];
      let reserved_names: Vec<Box<str>> = names.into_iter().map(|n| n.as_ref().into()).collect();

      msg.reserved_names = reserved_names.into_boxed_slice()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  /// Sets the reserved numbers for this enum
  pub fn reserved_numbers<I>(self, numbers: I) -> EnumBuilder<SetReservedNumbers<S>>
  where
    S::ReservedNumbers: IsUnset,
    I: IntoIterator<Item = i32>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.reserved_numbers = numbers.into_iter().collect()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  /// Sets the reserved ranges for this enum
  /// Just like in protobuf, ranges are considered to be inclusive.
  pub fn reserved_ranges<I>(self, ranges: I) -> EnumBuilder<SetReservedRanges<S>>
  where
    S::ReservedRanges: IsUnset,
    I: IntoIterator<Item = Range<i32>>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.enums[self.id];

      msg.reserved_ranges = ranges.into_iter().collect()
    }

    EnumBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
}

#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
pub struct SetVariants<S: EnumState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
pub struct SetReservedNumbers<S: EnumState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
pub struct SetReservedRanges<S: EnumState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
pub struct SetReservedNames<S: EnumState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
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
