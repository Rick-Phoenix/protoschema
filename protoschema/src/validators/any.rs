use std::collections::BTreeMap;

use bon::Builder;

use crate::{OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct AnyValidator<'a> {
  pub in_: Option<&'a [&'a str]>,
  pub not_in: Option<&'a [&'a str]>,
}

pub fn build_bytes_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(AnyValidatorBuilder) -> AnyValidatorBuilder<S>,
  S: any_validator_builder::IsComplete,
{
  let builder = AnyValidator::builder();
  let validator = config_fn(builder).build();
  let name = "(buf.validate.field).any";

  let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

  insert_option!(validator, values, in_, [string]);
  insert_option!(validator, values, not_in, [string]);

  ProtoOption {
    name,
    value: OptionValue::Message(values),
  }
}
