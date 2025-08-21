use std::collections::BTreeMap;

use bon::Builder;

use crate::{validators::cel::CelRule, OptionValue, ProtoOption};

#[derive(Debug, Clone, Builder)]
pub struct MessageValidator<'a> {
  pub cel: Option<&'a [CelRule]>,
  pub required: Option<bool>,
}

impl<'a, S: message_validator_builder::State> From<MessageValidatorBuilder<'a, S>> for ProtoOption {
  #[track_caller]
  fn from(value: MessageValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<MessageValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: MessageValidator) -> Self {
    let name = "(buf.validate.field)";

    let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

    insert_cel_rule!(validator, values);
    insert_option!(validator, values, required, bool);

    ProtoOption {
      name,
      value: OptionValue::Message(values),
    }
  }
}

#[track_caller]
pub fn build_message_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(MessageValidatorBuilder) -> MessageValidatorBuilder<S>,
  S: message_validator_builder::IsComplete,
{
  let builder = MessageValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}
