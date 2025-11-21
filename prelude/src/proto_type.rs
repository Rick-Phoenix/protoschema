use crate::*;

pub trait AsProtoType {
  fn proto_type() -> ProtoType;
}

#[derive(Debug, Clone)]
pub enum ProtoType {
  Single(TypeInfo),
  Repeated(TypeInfo),
  Optional(TypeInfo),
  Map { keys: TypeInfo, values: TypeInfo },
}

impl TypeInfo {
  pub fn register_import(&self, imports: &mut HashSet<Arc<str>>) {
    self
      .path
      .as_ref()
      .map(|path| imports.insert(path.file.clone()));
  }
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
  pub name: &'static str,
  pub path: Option<ProtoPath>,
}

#[derive(Debug, Clone)]
pub struct ProtoPath {
  pub package: Arc<str>,
  pub file: Arc<str>,
}
