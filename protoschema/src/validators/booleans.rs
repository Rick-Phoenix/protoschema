use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{OptionValue, ProtoOption};

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

    let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

    if let Some(const_val) = validator.const_ {
      values.insert("const".into(), OptionValue::Bool(const_val));
    }

    let mut options_map: BTreeMap<Box<str>, OptionValue> = btreemap! {
      "bool".into() => OptionValue::Message(values)
    };

    insert_option!(validator, options_map, required, bool);

    ProtoOption {
      name,
      value: OptionValue::Message(options_map),
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
