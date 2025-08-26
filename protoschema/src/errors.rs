use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplatingError {
  #[error("Could not read the parent directory for {0:?}")]
  MissingParentDirectory(PathBuf),
  #[error("Could not create the directory '{dir}': {source:?}")]
  DirCreationFailure { dir: PathBuf, source: io::Error },
  #[error("Could not create the file '{file}': {source:?}")]
  FileCreationFailure { file: PathBuf, source: io::Error },
  #[error("Could not write the template to the file '{file}': {source:?}")]
  TemplateWritingFailure { file: PathBuf, source: io::Error },
}
