use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
pub struct MapValidator {
  pub min_pairs: Option<u64>,
  pub max_pairs: Option<u64>,
}

#[track_caller]
pub fn build_map_validator_option<F, S>(
  config_fn: F,
  keys_rules: Option<ProtoOption>,
  values_rules: Option<ProtoOption>,
) -> ProtoOption
where
  F: FnOnce(MapValidatorBuilder) -> MapValidatorBuilder<S>,
  S: map_validator_builder::IsComplete,
{
  let builder = MapValidator::builder();
  let validator = config_fn(builder).build();
  let name = "(buf.validate.field).map";

  let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

  insert_option!(validator, values, min_pairs, Uint);
  insert_option!(validator, values, max_pairs, Uint);

  if let Some(keys_option) = keys_rules {
    values.insert(
      "keys".into(),
      OptionValue::Message(btreemap! {
        keys_option
        .name
        .strip_prefix("(buf.validate.field).")
        .expect("error during prefix stripping for keys rules")
        .into() => keys_option.value
      }),
    );
  }

  if let Some(values_option) = values_rules {
    values.insert(
      "values".into(),
      OptionValue::Message(btreemap! {
        values_option
        .name
        .strip_prefix("(buf.validate.field).")
        .expect("error during prefix stripping for values rules")
        .into() => values_option.value
      }),
    );
  }

  ProtoOption {
    name,
    value: OptionValue::Message(values),
  }
}
