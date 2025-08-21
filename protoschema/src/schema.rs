use std::{cell::RefCell, collections::HashSet, marker::PhantomData, rc::Rc};

use crate::{
  enums::{EnumBuilder, EnumData},
  message::{MessageBuilder, MessageData},
  rendering::{EnumTemplate, FileTemplate, MessageTemplate},
  sealed,
  services::{ServiceBuilder, ServiceData},
  Empty, IsUnset, Set, Unset,
};

pub(crate) type Arena = Rc<RefCell<PackageData>>;

#[derive(Default, Debug)]
pub(crate) struct PackageData {
  pub(crate) name: Box<str>,
  pub(crate) files: Vec<FileData>,
  pub(crate) messages: Vec<MessageData>,
  pub(crate) enums: Vec<EnumData>,
  pub(crate) services: Vec<ServiceData>,
}

#[derive(Clone)]
pub struct Package {
  arena: Arena,
}

impl Package {
  pub fn build_templates(&self) -> Vec<FileTemplate> {
    let mut templates: Vec<FileTemplate> = Vec::new();
    let arena = self.arena.borrow();
    let package = arena.name.clone();

    for file in &arena.files {
      let file_messages: Vec<MessageTemplate> = file
        .messages
        .iter()
        .map(|id| {
          let msg = &arena.messages[*id];

          msg.build_template(&self.arena.borrow())
        })
        .collect();

      let built_enums: Vec<EnumTemplate> = file
        .enums
        .iter()
        .map(|id| arena.enums[*id].clone().into())
        .collect();

      templates.push(FileTemplate {
        name: file.name.clone(),
        package: package.clone(),
        messages: file_messages,
        imports: file.imports.clone(),
        enums: built_enums,
      });
    }

    templates
  }

  pub fn new(name: &str) -> Self {
    Package {
      arena: Rc::new(RefCell::new(PackageData {
        name: name.into(),
        ..Default::default()
      })),
    }
  }
}

impl Package {
  pub fn new_file(&self, name: &str) -> FileBuilder {
    let mut arena = self.arena.borrow_mut();
    let file_id = arena.files.len();

    arena.files.push(FileData {
      name: name.into(),
      ..Default::default()
    });
    FileBuilder {
      id: file_id,
      arena: self.arena.clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Debug, Clone)]
pub struct FileBuilder<S: FileState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  _phantom: PhantomData<S>,
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

impl<S: FileState> FileBuilder<S> {
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

  pub fn get_data(&self) -> FileData {
    self.arena.borrow().files[self.id].clone()
  }

  pub fn get_name(&self) -> String {
    self.arena.borrow().files[self.id].name.to_string()
  }

  pub fn new_service(&self, name: &str) -> ServiceBuilder {
    let mut arena = self.arena.borrow_mut();
    let package_name = arena.name.clone();
    let service_id = arena.services.len();

    arena.files[self.id].services.push(service_id);

    arena.services.push(ServiceData {
      file_id: self.id,
      name: name.into(),
      package: package_name,
      ..Default::default()
    });

    ServiceBuilder {
      id: service_id,
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
}

#[derive(Clone, Debug, Default)]
pub struct FileData {
  pub name: Box<str>,
  pub messages: Vec<usize>,
  pub enums: Vec<usize>,
  pub imports: HashSet<Box<str>>,
  pub services: Vec<usize>,
}
