use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{
  field_type::Duration,
  validators::{cel::CelRule, validate_comparables, validate_lists, Ignore},
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
  pub cel: Option<&'a [CelRule]>,
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(DurationValidatorBuilder);

impl<'a, S: duration_validator_builder::State> From<DurationValidatorBuilder<'a, S>>
  for ProtoOption
{
  #[track_caller]
  fn from(value: DurationValidatorBuilder<'a, S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<DurationValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: DurationValidator<'a>) -> Self {
    let name = "(buf.validate.field)";

    let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

    if let Some(const_val) = validator.const_ {
      values.insert("const".into(), OptionValue::Duration(const_val));
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

    let mut options_map: BTreeMap<Box<str>, OptionValue> = btreemap! {
      "duration".into() => OptionValue::Message(values)
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
pub fn build_duration_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(DurationValidatorBuilder) -> DurationValidatorBuilder<S>,
  S: duration_validator_builder::IsComplete,
{
  let builder = DurationValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}
