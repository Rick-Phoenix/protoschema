use crate::fields::Field;

#[derive(Clone, Debug, Default)]
pub struct Extension {
  pub target: Box<str>,
  pub fields: Box<[Field]>,
  pub imports: Vec<Box<str>>,
}
