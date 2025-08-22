use std::{collections::HashSet, marker::PhantomData, sync::Arc};

use crate::{
  enums::{EnumBuilder, EnumData},
  extensions::{Extension, ExtensionData},
  fields::FieldData,
  message::{MessageBuilder, MessageData},
  rendering::FileTemplate,
  schema::Arena,
  services::{ServiceBuilder, ServiceData},
};

#[derive(Clone, Debug, Default)]
pub struct FileData {
  pub name: Arc<str>,
  pub messages: Vec<usize>,
  pub enums: Vec<usize>,
  pub imports: HashSet<Arc<str>>,
  pub services: Vec<usize>,
  pub extensions: Vec<ExtensionData>,
}

#[derive(Debug, Clone)]
pub struct FileBuilder {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
}

impl FileBuilder {
  pub fn get_data(&self) -> FileTemplate {
    let arena = self.arena.borrow();
    arena.files[self.id].build_template(&arena)
  }

  pub fn get_name(&self) -> String {
    self.arena.borrow().files[self.id].name.to_string()
  }

  pub fn new_message(&self, name: &str) -> MessageBuilder {
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let msg_id = arena.messages.len();

    arena.files[self.id].messages.push(msg_id);

    let full_name = arena.get_full_message_name(name, None);

    arena.messages.push(MessageData {
      name: name.into(),
      full_name,
      package: package_name,
      ..Default::default()
    });

    MessageBuilder {
      id: msg_id,
      arena: self.arena.clone(),
      file_id: self.id,
      _phantom: PhantomData,
    }
  }

  pub fn new_enum(&self, name: &str) -> EnumBuilder {
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let enum_id = arena.enums.len();

    arena.files[self.id].enums.push(enum_id);

    arena.enums.push(EnumData {
      file_id: self.id,
      full_name: name.into(),
      name: name.into(),
      package: package_name,
      ..Default::default()
    });

    EnumBuilder {
      id: enum_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }

  pub fn new_service(&self, name: &str) -> ServiceBuilder {
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let service_id = arena.services.len();

    arena.files[self.id].services.push(service_id);

    arena.services.push(ServiceData {
      name: name.into(),
      package: package_name,
      ..Default::default()
    });

    ServiceBuilder {
      id: service_id,
      arena: self.arena.clone(),
      file_id: self.id,
      _phantom: PhantomData,
    }
  }

  pub fn add_extension(self, extension: Extension) -> FileBuilder {
    {
      let mut arena = self.arena.borrow_mut();

      arena.files[self.id]
        .imports
        .insert(extension.import_path.clone());

      let built_fields: Vec<FieldData> = extension
        .fields
        .into_iter()
        .map(|f| {
          f.imports.into_iter().for_each(|i| {
            arena.files[self.id].imports.insert(i);
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
        target: extension.target,
        import_path: extension.import_path,
        fields: built_fields.into_boxed_slice(),
      };

      arena.files[self.id].extensions.push(ext_data)
    }

    FileBuilder {
      id: self.id,
      arena: self.arena,
    }
  }

  pub fn add_imports(self, imports: &[&str]) -> FileBuilder {
    {
      let file_imports = &mut self.arena.borrow_mut().files[self.id].imports;

      for &import in imports {
        file_imports.insert(import.into());
      }
    }

    FileBuilder {
      id: self.id,
      arena: self.arena,
    }
  }
}
