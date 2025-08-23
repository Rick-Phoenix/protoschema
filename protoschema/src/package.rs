use std::{
  cell::RefCell,
  fs::{create_dir_all, File},
  path::Path,
  rc::Rc,
  sync::Arc,
};

use askama::Template;

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
  path: Box<str>,
  data: Arena,
}

impl Package {
  pub fn get_name(&self) -> Arc<str> {
    self.data.borrow().name.clone()
  }

  pub fn new<T: AsRef<str>>(name: T) -> Self {
    Package {
      path: name.as_ref().replace(".", "/").into(),
      data: Rc::new(RefCell::new(PackageData {
        name: name.as_ref().into(),
        ..Default::default()
      })),
    }
  }

  pub fn new_file<T: AsRef<str>>(&self, name: T) -> FileBuilder {
    let mut arena = self.data.borrow_mut();
    let file_id = arena.files.len();

    arena.files.push(FileData {
      name: format!("{}/{}.proto", &self.path, name.as_ref()).into(),
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

  pub fn render_templates(&self, proto_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let templates = self.build_templates();

    for template in templates {
      let path = proto_root.join(template.name.as_ref());
      create_dir_all(path.parent().unwrap())?;
      let mut file = File::create(path)?;
      template.write_into(&mut file)?;
    }

    Ok(())
  }
}
