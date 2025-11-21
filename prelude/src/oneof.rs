use crate::*;

pub trait ProtoOneof {
  fn fields(tag_allocator: &mut TagAllocator) -> Vec<ProtoField>;
}

#[derive(Debug, Default, Clone)]
pub struct Oneof {
  pub name: Arc<str>,
  pub fields: Vec<ProtoField>,
  pub options: Vec<ProtoOption>,
}
