use bon::Builder;

use crate::{
  validators::{cel::CelRule, Ignore, OptionValueList},
  OptionValue, ProtoOption,
};

/// Used by the [`msg_field`](crate::msg_field) macro to define validation rules.
#[derive(Debug, Clone, Builder)]
pub struct MessageValidator {
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Box<[CelRule]>>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(no_lifetime, MessageValidatorBuilder);

impl<S: message_validator_builder::State> From<MessageValidatorBuilder<S>> for ProtoOption {
  #[track_caller]
  fn from(value: MessageValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

impl From<MessageValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: MessageValidator) -> Self {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    insert_cel_rule!(validator, values);
    insert_option!(validator, values, required, bool);

    ProtoOption {
      name,
      value: OptionValue::Message(values.into_boxed_slice()).into(),
    }
  }
}

#[doc(hidden)]
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
