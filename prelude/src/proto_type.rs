use crate::*;

pub trait AsProtoType {
  fn proto_type() -> ProtoType;
}

#[derive(Debug, Clone)]
pub struct ProtoPath {
  pub package: Arc<str>,
  pub file: Arc<str>,
}
