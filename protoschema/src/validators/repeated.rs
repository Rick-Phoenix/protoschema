use bon::Builder;

use crate::{
  validators::{
    any::*,
    booleans::*,
    bytes::*,
    cel::CelRule,
    duration::*,
    enums::*,
    message::{MessageValidator, MessageValidatorBuilder},
    numeric::*,
    string::{StringValidator, StringValidatorBuilder},
    timestamp::*,
    Ignore, OptionValueList,
  },
  OptionValue, ProtoOption,
};

/// A struct that can be used to generate a [`ProtoOption`] containing protovalidate rules for a repeated field.
/// Used by the various kinds of field macros such as [`string`](crate::string) or [`int64`](crate::int64) to define validation rules when the field is marked as repeated.
#[derive(Clone, Debug, Builder)]
pub struct RepeatedValidator<'a> {
  #[builder(into)]
  /// The rules to apply to the individual items in this field's list. Usually defined via the various field macros, which automatically convert field validator instances into the correct [`ProtoOption`] to place here.
  pub items: Option<ProtoOption>,
  /// The minimum amount of items that this field must contain in order to be valid.
  pub min_items: Option<u64>,
  /// The maximum amount of items that this field must contain in order to be valid.
  pub max_items: Option<u64>,
  #[builder(with = || true)]
  /// Specifies that this field must contain only unique values (only applies to scalar fields).
  pub unique: Option<bool>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  /// These will apply to the list as a whole. To apply rules to the individual items, use the items validator instead.
  pub cel: Option<&'a [CelRule]>,
  /// Marks the field as required. Since repeated fields are always present in protobuf, this is essentially the same as setting min_items to 1
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(RepeatedValidatorBuilder);

impl<'a, S: repeated_validator_builder::State> From<RepeatedValidatorBuilder<'a, S>>
  for ProtoOption
{
  #[track_caller]
  fn from(value: RepeatedValidatorBuilder<'a, S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<RepeatedValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: RepeatedValidator) -> ProtoOption {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    insert_option!(validator, values, unique, bool);
    insert_option!(validator, values, min_items, Uint);
    insert_option!(validator, values, max_items, Uint);

    if let Some(items_option) = validator.items {
      values.push(("items".into(), (*items_option.value).clone()));
    }

    let mut option_value: OptionValueList = vec![(
      "repeated".into(),
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

macro_rules! repeated_validator {
  ($validator_type:ident) => {
    $crate::paste! {
      #[doc(hidden)]
      #[track_caller]
      pub fn [< build_repeated_  $validator_type  _validator_option >]<'a, F, S: repeated_validator_builder::State>(config_fn: F) -> ProtoOption
      where
        F: FnOnce(RepeatedValidatorBuilder<'a>, [< $validator_type:camel ValidatorBuilder >]) -> RepeatedValidatorBuilder<'a, S>,
      {
        let repeated_validator_builder: RepeatedValidatorBuilder<'a> = RepeatedValidator::builder();
        let items_builder = [< $validator_type:camel Validator >]::builder();
        let validator = config_fn(repeated_validator_builder, items_builder).build();

        validator.into()
      }
    }
  };
}

repeated_validator!(string);
repeated_validator!(any);
repeated_validator!(duration);
repeated_validator!(timestamp);
repeated_validator!(bytes);
repeated_validator!(bool);
repeated_validator!(enum);
repeated_validator!(int64);
repeated_validator!(int32);
repeated_validator!(sint64);
repeated_validator!(sint32);
repeated_validator!(sfixed64);
repeated_validator!(sfixed32);
repeated_validator!(uint64);
repeated_validator!(uint32);
repeated_validator!(fixed64);
repeated_validator!(fixed32);
repeated_validator!(double);
repeated_validator!(float);
repeated_validator!(message);
