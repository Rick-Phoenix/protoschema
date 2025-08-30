use bon::Builder;

use crate::{
  field_type::Duration,
  validators::{cel::CelRule, validate_comparables, validate_lists, Ignore, OptionValueList},
  OptionValue, ProtoOption,
};

/// Used by the [`duration`](crate::duration) macro to define validation rules.
#[derive(Clone, Debug, Builder)]
pub struct DurationValidator {
  /// Only the values in this list will be considered valid for this field.
  #[builder(into)]
  pub in_: Option<Box<[Duration]>>,
  /// The values in this list will be considered invalid for this field.
  #[builder(into)]
  pub not_in: Option<Box<[Duration]>>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<Duration>,
  /// This field's value will be valid only if it is smaller than the specified amount
  pub lt: Option<Duration>,
  /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
  pub lte: Option<Duration>,
  /// This field's value will be valid only if it is greater than the specified amount
  pub gt: Option<Duration>,
  /// This field's value will be valid only if it is greater than, or equal to, the specified amount
  pub gte: Option<Duration>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Box<[CelRule]>>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(no_lifetime, DurationValidatorBuilder);

impl<S: duration_validator_builder::State> From<DurationValidatorBuilder<S>> for ProtoOption {
  #[track_caller]
  fn from(value: DurationValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

impl From<DurationValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: DurationValidator) -> Self {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      values.push(("const".into(), OptionValue::Duration(const_val)));
    }

    validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte);
    validate_lists(validator.in_.as_deref(), validator.not_in.as_deref()).unwrap_or_else(
      |invalid| {
        panic!(
          "The following values are present inside of 'in' and 'not_in': {:?}",
          invalid
        )
      },
    );

    insert_option!(validator, values, lt, duration);
    insert_option!(validator, values, lte, duration);
    insert_option!(validator, values, gt, duration);
    insert_option!(validator, values, gte, duration);
    insert_option!(validator, values, in_, [duration]);
    insert_option!(validator, values, not_in, [duration]);

    let mut option_value: OptionValueList = vec![(
      "duration".into(),
      OptionValue::Message(values.into_boxed_slice()),
    )];

    insert_cel_rule!(validator, option_value);
    insert_option!(validator, option_value, required, bool);

    ProtoOption {
      name,
      value: OptionValue::Message(option_value.into_boxed_slice()).into(),
    }
  }
}

#[doc(hidden)]
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
