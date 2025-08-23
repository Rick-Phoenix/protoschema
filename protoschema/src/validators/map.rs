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

#[derive(Clone, Debug, Builder)]
pub struct MapValidator<'a> {
  #[builder(into)]
  pub keys: Option<ProtoOption>,
  #[builder(into)]
  pub values: Option<ProtoOption>,
  pub min_pairs: Option<u64>,
  pub max_pairs: Option<u64>,
  pub cel: Option<&'a [CelRule]>,
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(MapValidatorBuilder);

impl<'a, S: map_validator_builder::State> From<MapValidatorBuilder<'a, S>> for ProtoOption {
  #[track_caller]
  fn from(value: MapValidatorBuilder<'a, S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<MapValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: MapValidator) -> Self {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    insert_option!(validator, values, min_pairs, Uint);
    insert_option!(validator, values, max_pairs, Uint);

    if let Some(keys_option) = validator.keys {
      values.push(("keys".into(), (*keys_option.value).clone()));
    }

    if let Some(values_option) = validator.values {
      values.push(("values".into(), (*values_option.value).clone()));
    }

    let mut option_value: OptionValueList = vec![(
      "map".into(),
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

macro_rules! map_validator {
  ($keys_type:ident, $values_type:ident) => {
    $crate::paste! {
      #[track_caller]
      pub fn [< build_map_ $keys_type _keys_ $values_type _values  _validator >]<'a, F, S>(config_fn: F) -> ProtoOption
      where
        F: FnOnce(MapValidatorBuilder<'a>, [< $keys_type:camel ValidatorBuilder >], [< $values_type:camel ValidatorBuilder >]) -> MapValidatorBuilder<'a, S>,
        S: map_validator_builder::State,
      {
        let map_validator_builder = MapValidator::builder();
        let keys_builder = [< $keys_type:camel Validator >]::builder();
        let values_builder = [< $values_type:camel Validator >]::builder();
        let validator = config_fn(map_validator_builder, keys_builder, values_builder).build();

        validator.into()
      }
    }
  };
}

map_validator!(int32, double);
map_validator!(int32, float);
map_validator!(int32, int64);
map_validator!(int32, uint64);
map_validator!(int32, int32);
map_validator!(int32, fixed64);
map_validator!(int32, fixed32);
map_validator!(int32, bool);
map_validator!(int32, string);
map_validator!(int32, bytes);
map_validator!(int32, uint32);
map_validator!(int32, enum);
map_validator!(int32, sfixed32);
map_validator!(int32, sfixed64);
map_validator!(int32, sint32);
map_validator!(int32, sint64);
map_validator!(int32, duration);
map_validator!(int32, timestamp);
map_validator!(int32, any);
map_validator!(int32, message);

map_validator!(int64, double);
map_validator!(int64, float);
map_validator!(int64, int64);
map_validator!(int64, uint64);
map_validator!(int64, int32);
map_validator!(int64, fixed64);
map_validator!(int64, fixed32);
map_validator!(int64, bool);
map_validator!(int64, string);
map_validator!(int64, bytes);
map_validator!(int64, uint32);
map_validator!(int64, enum);
map_validator!(int64, sfixed32);
map_validator!(int64, sfixed64);
map_validator!(int64, sint32);
map_validator!(int64, sint64);
map_validator!(int64, duration);
map_validator!(int64, timestamp);
map_validator!(int64, any);
map_validator!(int64, message);

map_validator!(uint32, double);
map_validator!(uint32, float);
map_validator!(uint32, int64);
map_validator!(uint32, uint64);
map_validator!(uint32, int32);
map_validator!(uint32, fixed64);
map_validator!(uint32, fixed32);
map_validator!(uint32, bool);
map_validator!(uint32, string);
map_validator!(uint32, bytes);
map_validator!(uint32, uint32);
map_validator!(uint32, enum);
map_validator!(uint32, sfixed32);
map_validator!(uint32, sfixed64);
map_validator!(uint32, sint32);
map_validator!(uint32, sint64);
map_validator!(uint32, duration);
map_validator!(uint32, timestamp);
map_validator!(uint32, any);
map_validator!(uint32, message);

map_validator!(uint64, double);
map_validator!(uint64, float);
map_validator!(uint64, int64);
map_validator!(uint64, uint64);
map_validator!(uint64, int32);
map_validator!(uint64, fixed64);
map_validator!(uint64, fixed32);
map_validator!(uint64, bool);
map_validator!(uint64, string);
map_validator!(uint64, bytes);
map_validator!(uint64, uint32);
map_validator!(uint64, enum);
map_validator!(uint64, sfixed32);
map_validator!(uint64, sfixed64);
map_validator!(uint64, sint32);
map_validator!(uint64, sint64);
map_validator!(uint64, duration);
map_validator!(uint64, timestamp);
map_validator!(uint64, any);
map_validator!(uint64, message);

map_validator!(sint32, double);
map_validator!(sint32, float);
map_validator!(sint32, int64);
map_validator!(sint32, uint64);
map_validator!(sint32, int32);
map_validator!(sint32, fixed64);
map_validator!(sint32, fixed32);
map_validator!(sint32, bool);
map_validator!(sint32, string);
map_validator!(sint32, bytes);
map_validator!(sint32, uint32);
map_validator!(sint32, enum);
map_validator!(sint32, sfixed32);
map_validator!(sint32, sfixed64);
map_validator!(sint32, sint32);
map_validator!(sint32, sint64);
map_validator!(sint32, duration);
map_validator!(sint32, timestamp);
map_validator!(sint32, any);
map_validator!(sint32, message);

map_validator!(sint64, double);
map_validator!(sint64, float);
map_validator!(sint64, int64);
map_validator!(sint64, uint64);
map_validator!(sint64, int32);
map_validator!(sint64, fixed64);
map_validator!(sint64, fixed32);
map_validator!(sint64, bool);
map_validator!(sint64, string);
map_validator!(sint64, bytes);
map_validator!(sint64, uint32);
map_validator!(sint64, enum);
map_validator!(sint64, sfixed32);
map_validator!(sint64, sfixed64);
map_validator!(sint64, sint32);
map_validator!(sint64, sint64);
map_validator!(sint64, duration);
map_validator!(sint64, timestamp);
map_validator!(sint64, any);
map_validator!(sint64, message);

map_validator!(fixed32, double);
map_validator!(fixed32, float);
map_validator!(fixed32, int64);
map_validator!(fixed32, uint64);
map_validator!(fixed32, int32);
map_validator!(fixed32, fixed64);
map_validator!(fixed32, fixed32);
map_validator!(fixed32, bool);
map_validator!(fixed32, string);
map_validator!(fixed32, bytes);
map_validator!(fixed32, uint32);
map_validator!(fixed32, enum);
map_validator!(fixed32, sfixed32);
map_validator!(fixed32, sfixed64);
map_validator!(fixed32, sint32);
map_validator!(fixed32, sint64);
map_validator!(fixed32, duration);
map_validator!(fixed32, timestamp);
map_validator!(fixed32, any);
map_validator!(fixed32, message);

map_validator!(fixed64, double);
map_validator!(fixed64, float);
map_validator!(fixed64, int64);
map_validator!(fixed64, uint64);
map_validator!(fixed64, int32);
map_validator!(fixed64, fixed64);
map_validator!(fixed64, fixed32);
map_validator!(fixed64, bool);
map_validator!(fixed64, string);
map_validator!(fixed64, bytes);
map_validator!(fixed64, uint32);
map_validator!(fixed64, enum);
map_validator!(fixed64, sfixed32);
map_validator!(fixed64, sfixed64);
map_validator!(fixed64, sint32);
map_validator!(fixed64, sint64);
map_validator!(fixed64, duration);
map_validator!(fixed64, timestamp);
map_validator!(fixed64, any);
map_validator!(fixed64, message);

map_validator!(sfixed32, double);
map_validator!(sfixed32, float);
map_validator!(sfixed32, int64);
map_validator!(sfixed32, uint64);
map_validator!(sfixed32, int32);
map_validator!(sfixed32, fixed64);
map_validator!(sfixed32, fixed32);
map_validator!(sfixed32, bool);
map_validator!(sfixed32, string);
map_validator!(sfixed32, bytes);
map_validator!(sfixed32, uint32);
map_validator!(sfixed32, enum);
map_validator!(sfixed32, sfixed32);
map_validator!(sfixed32, sfixed64);
map_validator!(sfixed32, sint32);
map_validator!(sfixed32, sint64);
map_validator!(sfixed32, duration);
map_validator!(sfixed32, timestamp);
map_validator!(sfixed32, any);
map_validator!(sfixed32, message);

map_validator!(sfixed64, double);
map_validator!(sfixed64, float);
map_validator!(sfixed64, int64);
map_validator!(sfixed64, uint64);
map_validator!(sfixed64, int32);
map_validator!(sfixed64, fixed64);
map_validator!(sfixed64, fixed32);
map_validator!(sfixed64, bool);
map_validator!(sfixed64, string);
map_validator!(sfixed64, bytes);
map_validator!(sfixed64, uint32);
map_validator!(sfixed64, enum);
map_validator!(sfixed64, sfixed32);
map_validator!(sfixed64, sfixed64);
map_validator!(sfixed64, sint32);
map_validator!(sfixed64, sint64);
map_validator!(sfixed64, duration);
map_validator!(sfixed64, timestamp);
map_validator!(sfixed64, any);
map_validator!(sfixed64, message);

map_validator!(bool, double);
map_validator!(bool, float);
map_validator!(bool, int64);
map_validator!(bool, uint64);
map_validator!(bool, int32);
map_validator!(bool, fixed64);
map_validator!(bool, fixed32);
map_validator!(bool, bool);
map_validator!(bool, string);
map_validator!(bool, bytes);
map_validator!(bool, uint32);
map_validator!(bool, enum);
map_validator!(bool, sfixed32);
map_validator!(bool, sfixed64);
map_validator!(bool, sint32);
map_validator!(bool, sint64);
map_validator!(bool, duration);
map_validator!(bool, timestamp);
map_validator!(bool, any);
map_validator!(bool, message);

map_validator!(string, double);
map_validator!(string, float);
map_validator!(string, int64);
map_validator!(string, uint64);
map_validator!(string, int32);
map_validator!(string, fixed64);
map_validator!(string, fixed32);
map_validator!(string, bool);
map_validator!(string, string);
map_validator!(string, bytes);
map_validator!(string, uint32);
map_validator!(string, enum);
map_validator!(string, sfixed32);
map_validator!(string, sfixed64);
map_validator!(string, sint32);
map_validator!(string, sint64);
map_validator!(string, duration);
map_validator!(string, timestamp);
map_validator!(string, any);
map_validator!(string, message);
