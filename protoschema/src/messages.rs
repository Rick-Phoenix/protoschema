use std::{marker::PhantomData, ops::Range, sync::Arc};

use crate::{
  enums::{EnumBuilder, EnumData},
  field_type::ImportedItemPath,
  fields::{self, FieldBuilder, FieldData},
  oneofs::{Oneof, OneofData},
  packages::Arena,
  rendering::MessageTemplate,
  sealed,
  validators::cel::CelRule,
  Empty, FieldType, IsSet, IsUnset, OptionValue, ProtoOption, Set, Unset,
};

/// The builder for a protobuf Message. Its methods are used to collect and store the data for a given message.
#[derive(Clone, Debug)]
pub struct MessageBuilder<S: MessageState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) file_id: usize,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct MessageData {
  pub name: Arc<str>,
  pub import_path: Arc<ImportedItemPath>,
  pub fields: Box<[(u32, FieldData)]>,
  pub oneofs: Vec<OneofData>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range<u32>]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Vec<ProtoOption>,
  pub enums: Vec<usize>,
  pub messages: Vec<usize>,
  pub imports: Vec<Arc<str>>,
}

impl<S: MessageState> MessageBuilder<S> {
  /// Sets the Cel rules for this message to be used with protovalidate.
  /// Cel rules can be easily defined with the [`cel_rule`](crate::cel_rule) macro, or directly within the [`message`](crate::message!) macro.
  pub fn cel_rules<I>(self, rules: I) -> MessageBuilder<S>
  where
    I: IntoIterator<Item = CelRule>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];
      msg.imports.push("buf/validate/validate.proto".into());

      let rules: Vec<OptionValue> = rules.into_iter().map(|r| r.into()).collect();
      let option = ProtoOption {
        name: "(buf.validate.message)",
        value: OptionValue::Message(
          vec![("cel".into(), OptionValue::List(rules.into_boxed_slice()))].into(),
        )
        .into(),
      };

      msg.options.push(option);
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  #[doc(hidden)]
  pub fn get_type(&self) -> FieldType {
    FieldType::Message(self.get_import_path())
  }

  /// Returns the import path for this message
  pub fn get_import_path(&self) -> Arc<ImportedItemPath> {
    self.arena.borrow().messages[self.id].import_path.clone()
  }

  #[doc(hidden)]
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// Builds the full template for this message and returns it.
  /// Mostly useful for debugging.
  pub fn get_data(self) -> MessageTemplate
  where
    S::Fields: IsSet,
  {
    let arena = self.arena.borrow();
    arena.messages[self.id].build_template(&arena)
  }

  /// Returns the full name for this message
  pub fn get_full_name(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.messages[self.id].import_path.full_name.clone()
  }

  /// Returns the full name for this message with the package prefix included
  pub fn get_full_name_with_package(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    let msg = &arena.messages[self.id];
    msg.import_path.full_name_with_package.clone()
  }

  /// Returns the name of this message's package
  pub fn get_package(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.messages[self.id].import_path.package.clone()
  }

  /// Returns the name of this message's file
  pub fn get_file(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.files[self.file_id].name.clone()
  }

  /// Creates a new message belonging to this message, and returns its builder
  pub fn new_message<T: AsRef<str>>(&self, name: T) -> MessageBuilder {
    let file_id = self.file_id;
    let package = self.get_package();
    let parent_message_name = self.get_full_name();

    let mut arena = self.arena.borrow_mut();
    let file_name = arena.files[file_id].name.clone();

    let parent_message_id = self.id;
    let child_message_id = arena.messages.len();

    arena.messages[parent_message_id]
      .messages
      .push(child_message_id);

    let full_message_name = format!("{}.{}", parent_message_name, name.as_ref());
    let full_name_with_package = format!("{}.{}", package, full_message_name);

    let new_msg = MessageData {
      name: name.as_ref().into(),
      import_path: ImportedItemPath {
        full_name: full_message_name.into(),
        full_name_with_package: full_name_with_package.into(),
        file: file_name.clone(),
        package,
      }
      .into(),
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

  /// Creates a new enum belonging to this message, and returns its builder
  pub fn new_enum<T: AsRef<str>>(&self, name: T) -> EnumBuilder {
    let package = self.get_package();
    let parent_message_name = self.get_full_name();
    let file_name = self.get_file();
    let mut arena = self.arena.borrow_mut();

    let file_id = self.file_id;
    let parent_message_id = self.id;
    let new_enum_id = arena.enums.len();

    arena.messages[parent_message_id].enums.push(new_enum_id);

    let full_enum_name = format!("{}.{}", parent_message_name, name.as_ref());
    let full_name_with_package = format!("{}.{}", package, full_enum_name);

    let new_enum = EnumData {
      name: name.as_ref().into(),
      import_path: ImportedItemPath {
        full_name: full_enum_name.into(),
        full_name_with_package: full_name_with_package.into(),
        file: file_name,
        package,
      }
      .into(),
      ..Default::default()
    };

    arena.enums.push(new_enum);

    EnumBuilder {
      id: new_enum_id,
      arena: self.arena.clone(),
      file_id,
      _phantom: PhantomData,
    }
  }

  /// Adds a list of imports to the file containing this message.  
  pub fn add_imports<I, Str>(&self, imports: I)
  where
    I: IntoIterator<Item = Str>,
    Str: Into<Arc<str>>,
  {
    let file = &mut self.arena.borrow_mut().files[self.file_id];

    for import in imports {
      file.conditionally_add_import(&import.into());
    }
  }

  /// Sets the fields for this message
  pub fn fields<I, F>(self, fields: I) -> MessageBuilder<SetFields<S>>
  where
    S::Fields: IsUnset,
    I: IntoIterator<Item = (u32, FieldBuilder<F>)>,
    F: fields::IsComplete,
  {
    {
      let mut arena = self.arena.borrow_mut();

      let mut final_fields: Vec<(u32, FieldData)> = fields
        .into_iter()
        .map(|(tag, field)| {
          let field = field.build();
          let file_id = self.file_id;

          for import in field.imports {
            arena.files[file_id].conditionally_add_import(&import);
          }

          (
            tag,
            FieldData {
              name: field.name,
              options: field.options.into_boxed_slice(),
              kind: field.kind,
              field_type: field.field_type,
            },
          )
        })
        .collect();

      final_fields.sort_by_key(|t| t.0);

      arena.messages[self.id].fields = final_fields.into_boxed_slice()
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  /// Adds the given oneofs to this message
  pub fn add_oneofs<I>(self, oneofs: I) -> MessageBuilder<SetOneofs<S>>
  where
    S::Oneofs: IsUnset,
    I: IntoIterator<Item = Oneof>,
  {
    {
      let mut arena = self.arena.borrow_mut();

      let oneofs_data: Vec<OneofData> = oneofs
        .into_iter()
        .map(|of| {
          for import in of.imports {
            arena.files[self.file_id].conditionally_add_import(&import);
          }

          let mut built_fields: Vec<(u32, FieldData)> = of
            .fields
            .into_iter()
            .map(|(tag, field)| {
              field.imports.into_iter().for_each(|i| {
                arena.files[self.file_id].conditionally_add_import(&i);
              });

              (
                tag,
                FieldData {
                  name: field.name.clone(),
                  options: field.options.clone().into_boxed_slice(),
                  kind: field.kind,
                  field_type: field.field_type.clone(),
                },
              )
            })
            .collect();

          built_fields.sort_by_key(|t| t.0);

          OneofData {
            name: of.name,
            options: of.options,
            fields: built_fields.into_boxed_slice(),
          }
        })
        .collect();

      arena.messages[self.id].oneofs.extend(oneofs_data);
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  /// Adds the given options to the message's list of options
  pub fn add_options<I>(self, options: I) -> MessageBuilder<S>
  where
    I: IntoIterator<Item = ProtoOption>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let msg = &mut arena.messages[self.id];

      msg.options.extend(options)
    }

    MessageBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  /// Sets the reserved names for this message
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

  /// Sets the reserved numbers for this message
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

  /// Sets the reserved ranges for this message.
  /// As in protobuf, the ranges are considered to be inclusive.
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
  pub struct oneofs;
}

#[doc(hidden)]
pub trait MessageState: Sized {
  type Fields;
  type ReservedNumbers;
  type ReservedRanges;
  type ReservedNames;
  type Oneofs;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

#[doc(hidden)]
pub struct SetFields<S: MessageState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
pub struct SetReservedNumbers<S: MessageState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
pub struct SetReservedRanges<S: MessageState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
pub struct SetReservedNames<S: MessageState = Empty>(PhantomData<fn() -> S>);
#[doc(hidden)]
pub struct SetOneofs<S: MessageState = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl MessageState for Empty {
  type Fields = Unset<members::fields>;
  type ReservedNumbers = Unset<members::reserved_numbers>;
  type ReservedRanges = Unset<members::reserved_ranges>;
  type ReservedNames = Unset<members::reserved_names>;
  type Oneofs = Unset<members::oneofs>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetFields<S> {
  type Fields = Set<members::fields>;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Oneofs = S::Oneofs;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetReservedNumbers<S> {
  type Fields = S::Fields;
  type ReservedNumbers = Set<members::reserved_numbers>;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetReservedRanges<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = Set<members::reserved_ranges>;
  type ReservedNames = S::ReservedNames;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetReservedNames<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = Set<members::reserved_names>;
  type Oneofs = S::Oneofs;

  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: MessageState> MessageState for SetOneofs<S> {
  type Fields = S::Fields;
  type ReservedNumbers = S::ReservedNumbers;
  type ReservedRanges = S::ReservedRanges;
  type ReservedNames = S::ReservedNames;
  type Oneofs = Set<members::oneofs>;

  const SEALED: sealed::Sealed = sealed::Sealed;
}
