use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{
  validators::{
    string::{StringValidator, StringValidatorBuilder},
    FieldValidator,
  },
  OptionValue, ProtoOption,
};

#[derive(Clone, Debug, Builder)]
pub struct RepeatedValidator {
  #[builder(setters(vis = "", name = items_internal))]
  pub items: Option<ProtoOption>,
  pub min_items: Option<u64>,
  pub max_items: Option<u64>,
  pub unique: Option<bool>,
}

use repeated_validator_builder::{IsUnset, SetItems, State};

impl<S: State> RepeatedValidatorBuilder<S> {
  pub fn items<V: FieldValidator>(self, validator: V) -> RepeatedValidatorBuilder<SetItems<S>>
  where
    S::Items: IsUnset,
  {
    self.items_internal(validator.convert_to_proto_option())
  }
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
