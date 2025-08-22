use std::{collections::HashSet, marker::PhantomData};

use crate::{
  enums::{EnumBuilder, EnumData},
  extensions::Extension,
  message::{MessageBuilder, MessageData},
  rendering::FileTemplate,
  schema::Arena,
  sealed,
  services::{ServiceBuilder, ServiceData},
  Empty, IsUnset, Set, Unset,
};

#[derive(Clone, Debug, Default)]
pub struct FileData {
  pub name: Box<str>,
  pub messages: Vec<usize>,
  pub enums: Vec<usize>,
  pub imports: HashSet<Box<str>>,
  pub services: Vec<usize>,
  pub extensions: Vec<Extension>,
}

#[derive(Debug, Clone)]
pub struct FileBuilder<S: FileState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<S>,
}

impl<S: FileState> FileBuilder<S> {
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
      file_id: self.id,
      name: name.into(),
      full_name,
      package: package_name,
      ..Default::default()
    });

    MessageBuilder {
      id: msg_id,
      arena: self.arena.clone(),
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

  pub fn add_extension(self, extension: Extension) -> FileBuilder<S> {
    {
      let mut arena = self.arena.borrow_mut();

      extension.imports.iter().for_each(|i| {
        arena.files[self.id].imports.insert(i.clone());
      });

      arena.files[self.id].extensions.push(extension)
    }

    FileBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }

  pub fn imports(self, imports: &[&str]) -> FileBuilder<SetImports<S>>
  where
    S::Imports: IsUnset,
  {
    {
      let file_imports = &mut self.arena.borrow_mut().files[self.id].imports;

      for &import in imports {
        file_imports.insert(import.into());
      }
    }

    FileBuilder {
      id: self.id,
      arena: self.arena,
      _phantom: PhantomData,
    }
  }
}

#[allow(non_camel_case_types)]
mod members {
  pub struct imports;
}

pub trait FileState: Sized {
  type Imports;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

impl FileState for Empty {
  type Imports = Unset<members::imports>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

pub struct SetImports<S: FileState = Empty>(PhantomData<fn() -> S>);

impl<S: FileState> FileState for SetImports<S> {
  type Imports = Set<members::imports>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
