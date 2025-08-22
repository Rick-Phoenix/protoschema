use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{
  validators::{cel::CelRule, validate_lists, Ignore},
  OptionValue, ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct AnyValidator<'a> {
  pub in_: Option<&'a [&'a str]>,
  pub not_in: Option<&'a [&'a str]>,
  pub cel: Option<&'a [CelRule]>,
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(AnyValidatorBuilder);

impl<'a, S: any_validator_builder::State> From<AnyValidatorBuilder<'a, S>> for ProtoOption {
  #[track_caller]
  fn from(value: AnyValidatorBuilder<'a, S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<AnyValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: AnyValidator<'a>) -> Self {
    let name = "(buf.validate.field)";

    let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

    validate_lists(validator.in_, validator.not_in).unwrap_or_else(|invalid| {
      panic!(
        "The following values are present inside of 'in' and 'not_in': {:?}",
        invalid
      )
    });

    insert_option!(validator, values, in_, [string]);
    insert_option!(validator, values, not_in, [string]);

    let mut options_map: BTreeMap<Box<str>, OptionValue> = btreemap! {
      "any".into() => OptionValue::Message(values)
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
pub fn build_any_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(AnyValidatorBuilder) -> AnyValidatorBuilder<S>,
  S: any_validator_builder::IsComplete,
{
  let builder = AnyValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}
