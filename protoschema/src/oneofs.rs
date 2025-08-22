use bon::Builder;
pub(crate) use oneof_data_builder::*;

use crate::{fields::Field, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct OneofData {
  pub name: String,
  pub parent_message_id: usize,
  pub fields: Vec<Field>,
  #[builder(default)]
  #[builder(setters(vis = "", name = options_internal))]
  pub options: Box<[ProtoOption]>,
}

impl<S: oneof_data_builder::State> OneofDataBuilder<S> {
  pub fn options(
    self,
    options: &[ProtoOption],
  ) -> OneofDataBuilder<oneof_data_builder::SetOptions<S>>
  where
    S::Options: IsUnset,
  {
    self.options_internal(options.into())
  }
}
