use bon::Builder;

use super::*;
use crate::*;

#[derive(Clone, Debug, Builder)]
pub struct BoolValidator {
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<bool>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
}

impl_validator!(BoolValidator, bool);

reusable_string!(BOOL);

impl From<BoolValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: BoolValidator) -> Self {
    let mut rules: OptionValueList = Vec::new();

    insert_option!(validator, rules, const_, bool);

    let mut outer_rules: OptionValueList = vec![(BOOL.clone(), OptionValue::Message(rules.into()))];

    insert_option!(validator, outer_rules, required, bool);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}
