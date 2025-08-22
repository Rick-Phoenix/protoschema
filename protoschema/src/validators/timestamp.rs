use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{
  field_type::{Duration, Timestamp},
  validators::{cel::CelRule, validate_comparables, Ignore},
  OptionValue, ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct TimestampValidator<'a> {
  pub const_: Option<Timestamp>,
  pub lt: Option<Timestamp>,
  pub lte: Option<Timestamp>,
  #[builder(with = || true)]
  pub lt_now: Option<bool>,
  pub gt: Option<Timestamp>,
  pub gte: Option<Timestamp>,
  #[builder(with = || true)]
  pub gt_now: Option<bool>,
  pub within: Option<Duration>,
  pub cel: Option<&'a [CelRule]>,
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(TimestampValidatorBuilder);

impl<'a, S: timestamp_validator_builder::State> From<TimestampValidatorBuilder<'a, S>>
  for ProtoOption
{
  #[track_caller]
  fn from(value: TimestampValidatorBuilder<'a, S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<TimestampValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: TimestampValidator<'a>) -> Self {
    let name = "(buf.validate.field)";

    let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

    if let Some(const_val) = validator.const_ {
      values.insert("const".into(), OptionValue::Timestamp(const_val));
      return ProtoOption {
        name: name.into(),
        value: OptionValue::Message(values).into(),
      };
    }

    validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte);

    if let Some(true) = validator.lt_now {
      if validator.lt.is_some() || validator.lte.is_some() {
        panic!("Cannot use lt_now with lt or lte")
      }

      if let Some(gt) = validator.gt && gt.is_future() {
      panic!("Gt cannot be in the future if lt_now is true")
    }
      if let Some(gte) = validator.gte && gte.is_future() {
      panic!("Gte cannot be in the future if lt_now is true")
    }
    }

    if let Some(true) = validator.gt_now {
      if validator.gt.is_some() || validator.gte.is_some() {
        panic!("Cannot use gt_now with gt or gte")
      }

      if let Some(lt) = validator.lt && lt.is_past() {
      panic!("Lt cannot be in the past if gt_now is true")
    }
      if let Some(lte) = validator.lte && lte.is_past() {
      panic!("Lte cannot be in the past if gt_now is true")
    }
    }

    insert_option!(validator, values, lt, timestamp);
    insert_option!(validator, values, lte, timestamp);
    insert_option!(validator, values, gt, timestamp);
    insert_option!(validator, values, gte, timestamp);
    insert_option!(validator, values, lt_now, bool);
    insert_option!(validator, values, gt_now, bool);
    insert_option!(validator, values, within, duration);

    let mut options_map: BTreeMap<Box<str>, OptionValue> = btreemap! {
      "timestamp".into() => OptionValue::Message(values)
    };

    insert_cel_rule!(validator, options_map);
    insert_option!(validator, options_map, required, bool);

    ProtoOption {
      name: name.into(),
      value: OptionValue::Message(options_map).into(),
    }
  }
}

#[track_caller]
pub fn build_timestamp_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(TimestampValidatorBuilder) -> TimestampValidatorBuilder<S>,
  S: timestamp_validator_builder::IsComplete,
{
  let builder = TimestampValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}
