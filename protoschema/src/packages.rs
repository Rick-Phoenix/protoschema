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
  errors::TemplatingError,
  files::{FileBuilder, FileData},
  messages::MessageData,
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

/// A struct representing a protobuf package.
#[derive(Clone)]
pub struct Package {
  path: Box<str>,
  data: Arena,
}

impl Package {
  /// Gets the name of this package
  pub fn get_name(&self) -> Arc<str> {
    self.data.borrow().name.clone()
  }

  /// Creates a new package
  pub fn new<T: AsRef<str>>(name: T) -> Self {
    Package {
      path: name.as_ref().replace(".", "/").into(),
      data: Rc::new(RefCell::new(PackageData {
        name: name.as_ref().into(),
        ..Default::default()
      })),
    }
  }

  /// Creates a new file belonging to this package.
  /// The ".proto" suffix is added automatically to the name being given.
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

  /// Builds all of the FileTemplates for this package, and returns them.
  /// This is only useful if you want to manually process the template's data.
  /// To write the templates directly, use [`render_templates`](crate::packages::Package::render_templates)
  pub fn build_templates(&self) -> Vec<FileTemplate> {
    let arena = self.data.borrow_mut();
    let templates: Vec<FileTemplate> = arena
      .files
      .iter()
      .map(|f| f.build_template(&arena))
      .collect();
    templates
  }

  /// Writes the protobuf files defined in this Package schema.
  ///
  /// The only argument it accepts is the proto_root, namely the root directory for the protobuf project.
  /// It will write the files by joining the root to the file names.
  ///
  /// # Examples
  /// With this input:
  /// ```rust
  #[doc = include_str!("../tests/test.rs")]
  /// ```
  ///
  /// The following file would be generated at `proto/myapp/v1/user.proto`
  /// ```proto
  #[doc = include_str!("../proto/myapp/v1/user.proto")]
  /// ```
  pub fn render_templates(&self, proto_root: &Path) -> Result<(), TemplatingError> {
    let templates = self.build_templates();

    for template in templates {
      let path = proto_root.join(template.name.as_ref());

      create_dir_all(
        path
          .parent()
          .ok_or(TemplatingError::MissingParentDirectory(path.clone()))?,
      )
      .map_err(|e| TemplatingError::DirCreationFailure {
        dir: path.clone(),
        source: e,
      })?;

      let mut file = File::create(&path).map_err(|e| TemplatingError::FileCreationFailure {
        file: path.clone(),
        source: e,
      })?;

      template
        .write_into(&mut file)
        .map_err(|e| TemplatingError::TemplateWritingFailure {
          file: path.clone(),
          source: e,
        })?;
    }

    Ok(())
  }
}
