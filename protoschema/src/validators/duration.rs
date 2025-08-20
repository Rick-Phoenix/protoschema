use std::collections::BTreeMap;

use bon::Builder;

use crate::{
  field_type::Duration,
  validators::{validate_comparables, validate_lists},
  OptionValue, ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct DurationValidator<'a> {
  pub in_: Option<&'a [Duration]>,
  pub not_in: Option<&'a [Duration]>,
  pub const_: Option<Duration>,
  pub lt: Option<Duration>,
  pub lte: Option<Duration>,
  pub gt: Option<Duration>,
  pub gte: Option<Duration>,
  pub defined_only: Option<bool>,
}

#[track_caller]
pub fn build_duration_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(DurationValidatorBuilder) -> DurationValidatorBuilder<S>,
  S: duration_validator_builder::IsComplete,
{
  let builder = DurationValidator::builder();
  let validator = config_fn(builder).build();
  let name = "(buf.validate.field).duration";

  let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

  if let Some(const_val) = validator.const_ {
    values.insert("const".into(), OptionValue::Duration(const_val));
    return ProtoOption {
      name,
      value: OptionValue::Message(values),
    };
  }

  validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte);
  validate_lists(validator.in_, validator.not_in).unwrap_or_else(|invalid| {
    panic!(
      "The following values are present inside of 'in' and 'not_in': {:?}",
      invalid
    )
  });

  insert_option!(validator, values, lt, duration);
  insert_option!(validator, values, lte, duration);
  insert_option!(validator, values, gt, duration);
  insert_option!(validator, values, gte, duration);
  insert_option!(validator, values, in_, [duration]);
  insert_option!(validator, values, not_in, [duration]);

  ProtoOption {
    name,
    value: OptionValue::Message(values),
  }
}
