use std::collections::BTreeMap;

use bon::Builder;

use crate::{validators::validate_lists, OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct EnumValidator<'a> {
  pub in_: Option<&'a [i32]>,
  pub not_in: Option<&'a [i32]>,
  pub const_: Option<i32>,
  pub defined_only: Option<bool>,
}

#[track_caller]
pub fn build_enum_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(EnumValidatorBuilder) -> EnumValidatorBuilder<S>,
  S: enum_validator_builder::IsComplete,
{
  let builder = EnumValidator::builder();
  let validator = config_fn(builder).build();
  let name = "(buf.validate.field).enum";

  let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

  if let Some(const_val) = validator.const_ {
    values.insert("const".into(), OptionValue::Int(const_val as i64));
    return ProtoOption {
      name,
      value: OptionValue::Message(values),
    };
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

  ProtoOption {
    name,
    value: OptionValue::Message(values),
  }
}
