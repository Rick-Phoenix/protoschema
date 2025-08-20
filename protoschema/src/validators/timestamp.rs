use std::collections::BTreeMap;

use bon::Builder;

use crate::{
  field_type::{Duration, Timestamp},
  validators::validate_comparables,
  OptionValue, ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct TimestampValidator {
  pub const_: Option<Timestamp>,
  pub lt: Option<Timestamp>,
  pub lte: Option<Timestamp>,
  pub lt_now: Option<bool>,
  pub gt: Option<Timestamp>,
  pub gte: Option<Timestamp>,
  pub gt_now: Option<bool>,
  pub within: Option<Duration>,
}

#[track_caller]
pub fn build_timestamp_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(TimestampValidatorBuilder) -> TimestampValidatorBuilder<S>,
  S: timestamp_validator_builder::IsComplete,
{
  let builder = TimestampValidator::builder();
  let validator = config_fn(builder).build();
  let name = "(buf.validate.field).timestamp";

  let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

  if let Some(const_val) = validator.const_ {
    values.insert("const".into(), OptionValue::Timestamp(const_val));
    return ProtoOption {
      name,
      value: OptionValue::Message(values),
    };
  }

  validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte);

  insert_option!(validator, values, lt, timestamp);
  insert_option!(validator, values, lte, timestamp);
  insert_option!(validator, values, gt, timestamp);
  insert_option!(validator, values, gte, timestamp);
  insert_option!(validator, values, lt_now, bool);
  insert_option!(validator, values, gt_now, bool);
  insert_option!(validator, values, within, duration);

  ProtoOption {
    name,
    value: OptionValue::Message(values),
  }
}
