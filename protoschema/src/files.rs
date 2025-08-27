use std::{collections::HashSet, marker::PhantomData, sync::Arc};

use crate::{
  common::DESCRIPTOR_PROTO_FILE,
  enums::{EnumBuilder, EnumData},
  extensions::{Extension, ExtensionData},
  field_type::ImportedItemPath,
  fields::FieldData,
  messages::{MessageBuilder, MessageData},
  packages::Arena,
  rendering::FileTemplate,
  services::{ServiceBuilder, ServiceData},
  ProtoOption,
};

#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct FileData {
  pub name: Arc<str>,
  pub messages: Vec<usize>,
  pub enums: Vec<usize>,
  pub imports: HashSet<Arc<str>>,
  pub services: Vec<usize>,
  pub extensions: Vec<ExtensionData>,
  pub options: Vec<ProtoOption>,
}

/// The builder for a protobuf file. Its methods are used to collect and store the data for a given file
#[derive(Debug, Clone)]
pub struct FileBuilder {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
}

impl FileData {
  pub(crate) fn conditionally_add_import(&mut self, import: &Arc<str>) {
    if import.as_ref() != self.name.as_ref() {
      self.imports.insert(import.clone());
    }
  }
}

impl FileBuilder {
  /// Builds the full template for this file, and returns it.
  /// Mostly useful for debugging.
  pub fn get_data(&self) -> FileTemplate {
    let arena = self.arena.borrow();
    arena.files[self.id].build_template(&arena)
  }

  /// Returns this file's name
  pub fn get_name(&self) -> Arc<str> {
    self.arena.borrow().files[self.id].name.clone()
  }

  /// Creates a new message belonging to this file and returns its builder.
  /// If you want to define a message inside another message, use [`MessageBuilder::new_message`] instead.
  pub fn new_message<T: AsRef<str>>(&self, name: T) -> MessageBuilder {
    let file_name = self.get_name();
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let msg_id = arena.messages.len();

    arena.files[self.id].messages.push(msg_id);
    let full_name_with_package = format!("{}.{}", package_name, name.as_ref());

    arena.messages.push(MessageData {
      name: name.as_ref().into(),
      import_path: ImportedItemPath {
        full_name: name.as_ref().into(),
        full_name_with_package: full_name_with_package.into(),
        package: package_name,
        file: file_name,
      }
      .into(),
      ..Default::default()
    });

    MessageBuilder {
      id: msg_id,
      arena: self.arena.clone(),
      file_id: self.id,
      _phantom: PhantomData,
    }
  }

  /// Creates a new enum belonging to this file and returns its builder.
  /// If you want to define an enum inside another message, use [`MessageBuilder::new_enum`] instead.
  pub fn new_enum<T: AsRef<str>>(&self, name: T) -> EnumBuilder {
    let file_name = self.get_name();
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let enum_id = arena.enums.len();

    arena.files[self.id].enums.push(enum_id);

    let full_name_with_package = format!("{}.{}", package_name, name.as_ref());

    arena.enums.push(EnumData {
      name: name.as_ref().into(),
      import_path: ImportedItemPath {
        full_name: name.as_ref().into(),
        full_name_with_package: full_name_with_package.into(),
        file: file_name,
        package: package_name,
      }
      .into(),
      file_id: self.id,
      ..Default::default()
    });

    EnumBuilder {
      id: enum_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }

  /// Creates a new service belonging to this file and returns its builder.
  /// Use the [`services`](crate::services!) macro to define multiple services with a shorter syntax.
  pub fn new_service<T: AsRef<str>>(&self, name: T) -> ServiceBuilder {
    let mut arena = self.arena.borrow_mut();
    let service_id = arena.services.len();

    arena.files[self.id].services.push(service_id);

    arena.services.push(ServiceData {
      name: name.as_ref().into(),
      ..Default::default()
    });

    ServiceBuilder {
      id: service_id,
      arena: self.arena.clone(),
      file_id: self.id,
      _phantom: PhantomData,
    }
  }

  /// Adds the given options to this file's options. It does not consume the original builder and does not return a new one.
  pub fn add_options<I>(&self, options: I)
  where
    I: IntoIterator<Item = ProtoOption>,
  {
    let file = &mut self.arena.borrow_mut().files[self.id];
    file.options.extend(options)
  }

  /// Adds the given extension to this file.
  /// Used by the [`extension`](crate::extension) macro.
  /// It does not consume the original builder and does not return a new one.
  pub fn add_extension(&self, extension: Extension) {
    let file = &mut self.arena.borrow_mut().files[self.id];

    file.imports.insert(DESCRIPTOR_PROTO_FILE.clone());

    let mut built_fields: Vec<(u32, FieldData)> = extension
      .fields
      .into_iter()
      .map(|(tag, field)| {
        field.imports.into_iter().for_each(|i| {
          file.conditionally_add_import(&i);
        });

        (
          tag,
          FieldData {
            name: field.name,
            field_type: field.field_type,
            kind: field.kind,
            options: field.options.into_boxed_slice(),
          },
        )
      })
      .collect();

    built_fields.sort_by_key(|t| t.0);

    let ext_data = ExtensionData {
      kind: extension.kind,
      fields: built_fields.into_boxed_slice(),
    };

    file.extensions.push(ext_data)
  }

  /// Adds the given imports to this file.
  /// For the most common cases, this crate will automatically add the necessary imports, so make sure to use this only if you notice that an import is missing.
  /// It does not consume the original builder and does not return a new one.
  pub fn add_imports<I, S>(&self, imports: I)
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    let file = &mut self.arena.borrow_mut().files[self.id];
    let file_name = file.name.as_ref();

    for import in imports.into_iter() {
      if import.as_ref() != file_name {
        file.imports.insert(import.as_ref().into());
      }
    }
  }
}
