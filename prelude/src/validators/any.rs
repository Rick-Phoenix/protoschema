use bon::Builder;
use proto_types::Any;

use super::*;
use crate::*;

#[derive(Clone, Debug, Builder)]
pub struct AnyValidator<'a> {
  /// Only the type_urls defined in this list will be considered valid for this field.
  #[builder(into)]
  pub in_: Option<&'a [&'a str]>,
  /// The type_urls defined in this list will be considered invalid for this field.
  #[builder(into)]
  pub not_in: Option<&'a [&'a str]>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  /// Marks the field as invalid if unset.
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

reusable_string!(ANY);

impl_validator!(AnyValidator, Any, with_lifetime);

impl_ignore!(AnyValidatorBuilder);

impl<'a> From<AnyValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: AnyValidator<'a>) -> Self {
    let mut rules: OptionValueList = Vec::new();

    validate_lists(validator.in_, validator.not_in).unwrap_or_else(|invalid| {
      panic!(
        "The following values are present inside of 'in' and 'not_in': {:?}",
        invalid
      )
    });

    insert_option!(validator, rules, in_, [string]);
    insert_option!(validator, rules, not_in, [string]);

    let mut outer_rules: OptionValueList = vec![(ANY.clone(), OptionValue::Message(rules.into()))];

    insert_cel_rule!(validator, outer_rules);
    insert_option!(validator, outer_rules, required, bool);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}
