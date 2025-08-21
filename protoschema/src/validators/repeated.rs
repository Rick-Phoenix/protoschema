use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{
  validators::{
    any::*,
    booleans::*,
    bytes::*,
    duration::*,
    enums::*,
    numeric::*,
    string::{StringValidator, StringValidatorBuilder},
    timestamp::*,
  },
  OptionValue, ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct RepeatedValidator {
  #[builder(into)]
  pub items: Option<ProtoOption>,
  pub min_items: Option<u64>,
  pub max_items: Option<u64>,
  pub unique: Option<bool>,
}

impl RepeatedValidator {
  #[track_caller]
  pub fn convert_to_proto_option(&self) -> ProtoOption {
    let name = "(buf.validate.field).repeated";
    let validator = self;

    let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

    insert_option!(validator, values, unique, bool);
    insert_option!(validator, values, min_items, Uint);
    insert_option!(validator, values, max_items, Uint);

    if let Some(items_option) = &validator.items {
      values.insert(
        "items".into(),
        OptionValue::Message(btreemap! {
          items_option
          .name
          .strip_prefix("(buf.validate.field).")
          .expect("error during prefix stripping for items rules")
          .into() => items_option.value.clone()
        }),
      );
    }

    ProtoOption {
      name,
      value: OptionValue::Message(values),
    }
  }
}

macro_rules! repeated_validator {
  ($validator_type:ident) => {
    paste::paste! {
      #[track_caller]
      pub fn [< build_repeated_  $validator_type  _validator_option >]<F, S>(config_fn: F) -> ProtoOption
      where
        F: FnOnce(RepeatedValidatorBuilder, [< $validator_type:camel ValidatorBuilder >]) -> RepeatedValidatorBuilder<S>,
        S: repeated_validator_builder::IsComplete,
      {
        let repeated_validator_builder = RepeatedValidator::builder();
        let items_builder = [< $validator_type:camel Validator >]::builder();
        let validator = config_fn(repeated_validator_builder, items_builder).build();

        validator.convert_to_proto_option()
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

#[track_caller]
pub fn build_repeated_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(RepeatedValidatorBuilder) -> RepeatedValidatorBuilder<S>,
  S: repeated_validator_builder::IsComplete,
{
  let builder = RepeatedValidator::builder();
  let validator = config_fn(builder).build();

  validator.convert_to_proto_option()
}
