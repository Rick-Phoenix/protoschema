use std::marker::PhantomData;

use bon::Builder;
pub(crate) use field_builder::*;

use crate::{fields::string_validator_builder::IsComplete, FieldType, OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct Field {
  #[builder(field)]
  pub options: Vec<ProtoOption>,
  pub parent_message_id: usize,
  pub name: Box<str>,
  pub tag: u32,
  pub field_type: FieldType,
}

impl<S: field_builder::State> FieldBuilder<S> {
  pub fn option(mut self, option: ProtoOption) -> Self {
    self.options.push(option);
    self
  }

  pub fn options(mut self, options: Vec<ProtoOption>) -> Self {
    self.options = options;
    self
  }
}

#[derive(Clone, Debug, Builder)]
pub struct StringValidator {
  pub min_len: usize,
  pub max_len: usize,
}

impl From<StringValidator> for ProtoOption {
  fn from(value: StringValidator) -> Self {
    ProtoOption {
      name: "(buf.validate.field).string",
      value: OptionValue::String("abc".to_string()),
    }
  }
}

pub trait IntoProtoOption {
  fn into_option(self) -> ProtoOption;
}

impl IntoProtoOption for ProtoOption {
  fn into_option(self) -> ProtoOption {
    self
  }
}

pub fn build_string_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(StringValidatorBuilder) -> StringValidatorBuilder<S>,
  S: IsComplete,
{
  let builder = StringValidator::builder();
  let configured_builder = config_fn(builder);
  configured_builder.build().into()
}
