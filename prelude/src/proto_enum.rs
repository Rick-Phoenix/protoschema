use crate::*;

pub trait ProtoEnumTrait {}

#[derive(Debug, Default, Clone)]
pub struct ProtoEnum {
  pub name: Arc<str>,
  pub full_name: &'static str,
  pub package: Arc<str>,
  pub file: Arc<str>,
  pub variants: Vec<EnumVariant>,
  pub reserved_numbers: Vec<Range<i32>>,
  pub reserved_names: Vec<&'static str>,
  pub options: Vec<ProtoOption>,
}

#[derive(Debug, Default, Clone)]
pub struct EnumVariant {
  pub name: String,
  pub tag: i32,
  pub options: Vec<ProtoOption>,
}
