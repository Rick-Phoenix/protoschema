use bon::Builder;

use crate::{
  validators::{cel::CelRule, validate_lists, Ignore, OptionValueList},
  OptionValue, ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct EnumValidator<'a> {
  pub in_: Option<&'a [i32]>,
  pub not_in: Option<&'a [i32]>,
  pub const_: Option<i32>,
  #[builder(with = || true)]
  pub defined_only: Option<bool>,
  pub cel: Option<&'a [CelRule]>,
  #[builder(with = || true)]
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
