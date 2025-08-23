use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
  enums::EnumData,
  files::{FileBuilder, FileData},
  message::MessageData,
  rendering::FileTemplate,
  services::ServiceData,
};

pub(crate) type Arena = Rc<RefCell<PackageData>>;

#[derive(Default, Debug)]
pub(crate) struct PackageData {
  pub(crate) name: Arc<str>,
  pub(crate) files: Vec<FileData>,
  pub(crate) messages: Vec<MessageData>,
  pub(crate) enums: Vec<EnumData>,
  pub(crate) services: Vec<ServiceData>,
}

#[derive(Clone)]
pub struct Package {
  data: Arena,
}

impl Package {
  pub fn get_name(&self) -> Arc<str> {
    self.data.borrow().name.clone()
  }

  pub fn new(name: &str) -> Self {
    Package {
      data: Rc::new(RefCell::new(PackageData {
        name: name.into(),
        ..Default::default()
      })),
    }
  }

  pub fn new_file(&self, name: &str) -> FileBuilder {
    let mut arena = self.data.borrow_mut();
    let file_id = arena.files.len();

    arena.files.push(FileData {
      name: name.into(),
      ..Default::default()
    });
    FileBuilder {
      id: file_id,
      arena: self.data.clone(),
    }
  }

  pub fn build_templates(&self) -> Vec<FileTemplate> {
    let arena = self.data.borrow_mut();
    let templates: Vec<FileTemplate> = arena
      .files
      .iter()
      .map(|f| f.build_template(&arena))
      .collect();
    templates
  }
}
