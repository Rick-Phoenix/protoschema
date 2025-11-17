use std::marker::PhantomData;

use super::*;
use crate::*;

fn test() {
  let x: MapValidatorBuilder<bool> = MapValidator::builder();

  x.keys(|v| v.const_(false));
}

impl<KI, VI, S: map_validator_builder::State> MapValidatorBuilder<KI, VI, S>
where
  S::Keys: map_validator_builder::IsUnset,
{
  pub fn keys<F, FinalBuilder>(
    self,
    config_fn: F,
  ) -> MapValidatorBuilder<KI, VI, map_validator_builder::SetKeys<S>>
  where
    ValidatorMap: ProtoValidator<KI>,
    FinalBuilder: Into<ProtoOption>,
    F: for<'a> FnOnce(<ValidatorMap as ProtoValidator<KI>>::Builder<'a>) -> FinalBuilder,
  {
    let keys_opts = ValidatorMap::build_rules(config_fn);
    self.keys_internal(keys_opts)
  }
}

impl<KI, VI, S: map_validator_builder::State> MapValidatorBuilder<KI, VI, S>
where
  S::Values: map_validator_builder::IsUnset,
{
  pub fn values<F, FinalBuilder>(
    self,
    config_fn: F,
  ) -> MapValidatorBuilder<KI, VI, map_validator_builder::SetValues<S>>
  where
    ValidatorMap: ProtoValidator<VI>,
    FinalBuilder: Into<ProtoOption>,
    F: for<'a> FnOnce(<ValidatorMap as ProtoValidator<VI>>::Builder<'a>) -> FinalBuilder,
  {
    let values_opts = ValidatorMap::build_rules(config_fn);
    self.values_internal(values_opts)
  }
}

#[derive(Clone, Debug, Builder)]
pub struct MapValidator<KeyItems = (), ValueItems = ()> {
  #[builder(default)]
  _key_items: PhantomData<KeyItems>,
  #[builder(default)]
  _values_items: PhantomData<ValueItems>,

  #[builder(setters(vis = "", name = keys_internal))]
  /// The options that will apply to this map's keys.
  /// This is mostly useful when calling the map definition macros, which will automatically convert validators into the option to use here.
  pub keys: Option<ProtoOption>,
  #[builder(setters(vis = "", name = values_internal))]
  /// The options that will apply to this map's values.
  /// This is mostly useful when calling the map definition macros, which will automatically convert validators into the option to use here.
  pub values: Option<ProtoOption>,
  /// The minimum amount of key-value pairs that this field should have in order to be valid.
  pub min_pairs: Option<u64>,
  /// The maximum amount of key-value pairs that this field should have in order to be valid.
  pub max_pairs: Option<u64>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  /// These will apply to the map field as a whole.
  /// To apply cel rules to the individual keys or values, use the validators for those instead.
  pub cel: Option<Arc<[CelRule]>>,
  #[builder(with = || true)]
  /// Marks the field as required. This is essentially the same as setting min_pairs to 1.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

// impl_ignore!(no_lifetime, MapValidatorBuilder);

reusable_string!(MIN_PAIRS);
reusable_string!(MAX_PAIRS);

impl<KeyItems, ValueItems> From<MapValidator<KeyItems, ValueItems>> for ProtoOption {
  #[track_caller]
  fn from(validator: MapValidator<KeyItems, ValueItems>) -> Self {
    let mut rules: OptionValueList = Vec::new();

    insert_option!(validator, rules, min_pairs, Uint);
    insert_option!(validator, rules, max_pairs, Uint);

    if let Some(keys_option) = validator.keys {
      rules.push(("keys".into(), (keys_option.value).clone()));
    }

    if let Some(values_option) = validator.values {
      rules.push(("values".into(), (values_option.value).clone()));
    }

    let mut option_value: OptionValueList =
      vec![("map".into(), OptionValue::Message(rules.into()))];

    insert_cel_rule!(validator, option_value);
    insert_option!(validator, option_value, required, bool);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(option_value.into()),
    }
  }
}
