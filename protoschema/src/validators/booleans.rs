use std::collections::BTreeMap;

use bon::Builder;

use crate::{OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct BoolValidator {
  pub const_: Option<bool>,
}

pub fn build_bytes_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(BoolValidatorBuilder) -> BoolValidatorBuilder<S>,
  S: bool_validator_builder::IsComplete,
{
  let builder = BoolValidator::builder();
  let validator = config_fn(builder).build();
  let name = "(buf.validate.field).bool";

  let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

  if let Some(const_val) = validator.const_ {
    values.insert("const".into(), OptionValue::Bool(const_val));
  }

  ProtoOption {
    name,
    value: OptionValue::Message(values),
  }
}
