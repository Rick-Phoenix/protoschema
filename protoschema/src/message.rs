use std::{marker::PhantomData, ops::Range, sync::Arc};

use maplit::btreemap;

use crate::{
  enums::{EnumBuilder, EnumData},
  field_type::ImportedItemPath,
  fields::{self, FieldBuilder, FieldData},
  oneofs::{Oneof, OneofData},
  package::Arena,
  rendering::MessageTemplate,
  sealed,
  validators::cel::CelRule,
  Empty, FieldType, IsSet, IsUnset, OptionValue, ProtoOption, Set, Unset,
};

#[derive(Clone, Debug)]
pub struct MessageBuilder<S: MessageState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) file_id: usize,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

#[derive(Clone, Debug, Default)]
pub struct MessageData {
  pub import_path: Arc<ImportedItemPath>,
  pub fields: Box<[FieldData]>,
  pub oneofs: Box<[OneofData]>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range<u32>]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Vec<ProtoOption>,
  pub enums: Vec<usize>,
  pub messages: Vec<usize>,
  pub imports: Vec<Arc<str>>,
}

impl<S: MessageState> MessageBuilder<S> {
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
        name: "(buf.validate.cel).message",
        value: OptionValue::Message(btreemap! {
          "cel".into() => OptionValue::List(rules.into_boxed_slice())
        })
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

  // Getters
  pub fn get_type(&self) -> FieldType {
    FieldType::Message(self.get_import_path())
  }

  pub fn get_import_path(&self) -> Arc<ImportedItemPath> {
    self.arena.borrow().messages[self.id].import_path.clone()
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

    arena.messages[self.id].import_path.name.clone()
  }

  pub fn get_full_name(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    let msg = &arena.messages[self.id];
    msg.import_path.full_name.clone()
  }

  pub fn get_package(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.messages[self.id].import_path.package.clone()
  }

  pub fn get_file(&self) -> Arc<str> {
    let arena = self.arena.borrow();

    arena.files[self.file_id].name.clone()
  }

  // Setters
  pub fn new_message(&self, name: &str) -> MessageBuilder {
    let file_id = self.file_id;
    let package = self.get_package();
    let parent_message_name = self.get_name();

    let mut arena = self.arena.borrow_mut();
    let file_name = arena.files[file_id].name.clone();

    let parent_message_id = self.id;
    let child_message_id = arena.messages.len();

    arena.messages[parent_message_id]
      .messages
      .push(child_message_id);

    let full_message_name = format!("{}.{}", parent_message_name, name);
    let full_name_with_package = format!("{}.{}", package, full_message_name);

    let new_msg = MessageData {
      import_path: ImportedItemPath {
        name: full_message_name.into(),
        full_name: full_name_with_package.into(),
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

  pub fn new_enum(&self, name: &str) -> EnumBuilder {
    let package = self.get_package();
    let parent_message_name = self.get_name();
    let file_name = self.get_file();
    let mut arena = self.arena.borrow_mut();

    let file_id = self.file_id;
    let parent_message_id = self.id;
    let new_enum_id = arena.enums.len();

    arena.messages[parent_message_id].enums.push(new_enum_id);

    let full_enum_name = format!("{}.{}", parent_message_name, name);
    let full_name_with_package = format!("{}.{}", package, full_enum_name);

    let new_enum = EnumData {
      import_path: ImportedItemPath {
        name: full_enum_name.into(),
        full_name: full_name_with_package.into(),
        file: file_name,
        package,
      }
      .into(),
      file_id,
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
            arena.files[file_id].conditionally_add_import(import);
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
                arena.files[self.file_id].conditionally_add_import(i);
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
  pub struct oneofs;
}

pub trait MessageState: Sized {
  type Fields;
  type ReservedNumbers;
  type ReservedRanges;
  type ReservedNames;
  type Oneofs;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

pub struct SetFields<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNumbers<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedRanges<S: MessageState = Empty>(PhantomData<fn() -> S>);
pub struct SetReservedNames<S: MessageState = Empty>(PhantomData<fn() -> S>);
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
