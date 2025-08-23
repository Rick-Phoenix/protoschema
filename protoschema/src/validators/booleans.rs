use bon::Builder;

use crate::{validators::OptionValueList, OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct BoolValidator {
  pub const_: Option<bool>,
  #[builder(with = || true)]
  pub required: Option<bool>,
}

impl<S: bool_validator_builder::State> From<BoolValidatorBuilder<S>> for ProtoOption {
  #[track_caller]
  fn from(value: BoolValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

impl From<BoolValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: BoolValidator) -> Self {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      values.push(("const".into(), OptionValue::Bool(const_val)));
    }

    let mut option_value: OptionValueList = vec![(
      "bool".into(),
      OptionValue::Message(values.into_boxed_slice()),
    )];

    insert_option!(validator, option_value, required, bool);

    ProtoOption {
      name,
      value: OptionValue::Message(option_value.into_boxed_slice()).into(),
    }
  }
}

#[track_caller]
pub fn build_bool_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(BoolValidatorBuilder) -> BoolValidatorBuilder<S>,
  S: bool_validator_builder::IsComplete,
{
  let builder = BoolValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}
