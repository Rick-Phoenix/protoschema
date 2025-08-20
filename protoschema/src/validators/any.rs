use std::collections::BTreeMap;

use bon::Builder;

use crate::{validators::validate_lists, OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct AnyValidator<'a> {
  pub in_: Option<&'a [&'a str]>,
  pub not_in: Option<&'a [&'a str]>,
}

#[track_caller]
pub fn build_any_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(AnyValidatorBuilder) -> AnyValidatorBuilder<S>,
  S: any_validator_builder::IsComplete,
{
  let builder = AnyValidator::builder();
  let validator = config_fn(builder).build();
  let name = "(buf.validate.field).any";

  let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

  validate_lists(validator.in_, validator.not_in).unwrap_or_else(|invalid| {
    panic!(
      "The following values are present inside of 'in' and 'not_in': {:?}",
      invalid
    )
  });

  insert_option!(validator, values, in_, [string]);
  insert_option!(validator, values, not_in, [string]);

  ProtoOption {
    name,
    value: OptionValue::Message(values),
  }
}
