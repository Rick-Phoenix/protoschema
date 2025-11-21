use crate::*;

pub trait ProtoEnumTrait {}

#[derive(Debug, Default, Clone)]
pub struct ProtoEnum {
  pub name: Arc<str>,
  pub full_name: &'static str,
  pub package: Arc<str>,
  pub file: Arc<str>,
  pub variants: Vec<(u32, EnumVariant)>,
  pub reserved_numbers: Vec<Range<u32>>,
  pub reserved_names: Vec<&'static str>,
  pub options: Vec<ProtoOption>,
}

#[derive(Debug, Default, Clone)]
pub struct EnumVariant {
  pub name: String,
  pub options: Vec<ProtoOption>,
}
