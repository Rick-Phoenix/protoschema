use std::{collections::HashSet, marker::PhantomData, sync::Arc};

use crate::{
  enums::{EnumBuilder, EnumData},
  extensions::{Extension, ExtensionData},
  field_type::ImportedItemPath,
  fields::FieldData,
  message::{MessageBuilder, MessageData},
  package::Arena,
  rendering::FileTemplate,
  services::{ServiceBuilder, ServiceData},
  ProtoOption,
};

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
  pub fn get_data(&self) -> FileTemplate {
    let arena = self.arena.borrow();
    arena.files[self.id].build_template(&arena)
  }

  pub fn get_name(&self) -> Arc<str> {
    self.arena.borrow().files[self.id].name.clone()
  }

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

  pub fn add_options<I>(&self, options: I)
  where
    I: IntoIterator<Item = ProtoOption>,
  {
    let file = &mut self.arena.borrow_mut().files[self.id];
    file.options.extend(options)
  }

  pub fn add_extension(&self, extension: Extension) {
    let file = &mut self.arena.borrow_mut().files[self.id];

    file.conditionally_add_import(&extension.import_path.file);

    let built_fields: Vec<FieldData> = extension
      .fields
      .into_iter()
      .map(|f| {
        f.imports.into_iter().for_each(|i| {
          file.conditionally_add_import(&i);
        });

        FieldData {
          name: f.name,
          field_type: f.field_type,
          kind: f.kind,
          options: f.options.into_boxed_slice(),
          tag: f.tag,
        }
      })
      .collect();

    let ext_data = ExtensionData {
      import_path: extension.import_path,
      fields: built_fields.into_boxed_slice(),
    };

    file.extensions.push(ext_data)
  }

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
