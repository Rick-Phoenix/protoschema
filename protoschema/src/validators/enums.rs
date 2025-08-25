use bon::Builder;

use crate::{
  validators::{cel::CelRule, validate_lists, Ignore, OptionValueList},
  OptionValue, ProtoOption,
};

/// Used by the [`enum_field`](crate::enum_field) macro to define validation rules.
#[derive(Clone, Debug, Builder)]
pub struct EnumValidator<'a> {
  /// Only the values in this list will be considered valid for this field.
  pub in_: Option<&'a [i32]>,
  /// The values in this list will be considered invalid for this field.
  pub not_in: Option<&'a [i32]>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<i32>,
  #[builder(with = || true)]
  /// Marks that this field will only accept values that are defined in the enum that it's referring to.
  pub defined_only: Option<bool>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  pub cel: Option<&'a [CelRule]>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(EnumValidatorBuilder);

use enum_validator_builder::State;

impl<'a, S: State> From<EnumValidatorBuilder<'a, S>> for ProtoOption {
  #[track_caller]
  fn from(value: EnumValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<EnumValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: EnumValidator) -> Self {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      values.push(("const".into(), OptionValue::Int(const_val as i64)));
    }

    validate_lists(validator.in_, validator.not_in).unwrap_or_else(|invalid| {
      panic!(
        "The following values are present inside of 'in' and 'not_in': {:?}",
        invalid
      )
    });

    insert_option!(validator, values, defined_only, bool);
    insert_option!(validator, values, in_, [i32]);
    insert_option!(validator, values, not_in, [i32]);

    let mut option_value: OptionValueList = vec![(
      "enum".into(),
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
pub fn build_enum_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(EnumValidatorBuilder) -> EnumValidatorBuilder<S>,
  S: enum_validator_builder::IsComplete,
{
  let builder = EnumValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}
