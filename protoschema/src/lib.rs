#![allow(dead_code)]

use bon::Builder;

pub use crate::{
  field_type::FieldType,
  option::{OptionValue, ProtoOption},
};

mod field_type;
#[macro_use]
mod option;

use std::{cell::RefCell, rc::Rc};

type Pool = Rc<RefCell<SchemaInner>>;

pub(crate) struct SchemaInner {
  pub(crate) packages: Vec<PackageData>,
  pub(crate) files: Vec<FileData>,
  pub(crate) messages: Vec<MessageData>,
}

#[derive(Clone)]
pub struct Schema {
  pool: Pool,
}

impl Default for Schema {
  fn default() -> Self {
    Self::new()
  }
}

impl Schema {
  pub fn new() -> Self {
    let inner = SchemaInner {
      packages: vec![],
      files: vec![],
      messages: vec![],
    };
    let pool = Rc::new(RefCell::new(inner));

    Self { pool }
  }

  pub fn new_package(&self, name: &'static str) -> Package {
    let package = PackageData {
      name: name.into(),
      files: vec![],
    };
    let mut pool = self.pool.borrow_mut();
    let id = pool.packages.len();
    pool.packages.push(package);

    Package {
      schema: self.clone(),
      id,
    }
  }
}

pub struct FileData {
  pub name: Box<str>,
  pub options: Vec<ProtoOption>,
  pub messages: Vec<usize>,
  pub imports: Vec<String>,
  pub enums: Vec<usize>,
  pub package_id: usize,
}

pub struct Package {
  schema: Schema,
  id: usize,
}

impl Package {
  pub fn add_file(&self) -> File {
    let mut pool = self.schema.pool.get_mut();

    let file_id = pool.files.len();
    let file_data = FileData {  }
  }
}

#[derive(Builder)]
pub struct File2 {
  package_id: usize
}

impl File2{
  pub fn from_package(package: &PackageData) -> Self {
    File2::builder().package_id(1).build()
  } 
}

pub struct File {
  schema: Schema,
  id: usize,
}

pub struct PackageData {
  pub name: Box<str>,
  pub files: Vec<usize>,
}

pub struct Range {
  pub start: u32,
  pub end: u32,
}

pub struct EnumData {
  pub name: Box<str>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Vec<ProtoOption>,
  pub variants: Vec<(u32, String)>,
  pub file_id: usize,
  pub parent_message_id: Option<usize>,
}

pub struct MessageData {
  pub name: Box<str>,
  pub reserved_numbers: Box<[u32]>,
  pub reserved_ranges: Box<[Range]>,
  pub reserved_names: Box<[Box<str>]>,
  pub options: Vec<ProtoOption>,
  pub fields: Box<[usize]>,
  pub enums: Vec<usize>,
  pub file_id: usize,
}

pub struct FieldData {
  pub name: Box<str>,
  pub ty: FieldType,
  pub tag: u32,
  pub options: Vec<ProtoOption>,
}
