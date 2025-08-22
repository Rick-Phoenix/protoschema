use std::collections::BTreeMap;

use bon::Builder;
use maplit::btreemap;

use crate::{
  validators::{cel::CelRule, Ignore},
  OptionValue, ProtoOption,
};

macro_rules! get_list_check {
  (f32, $in_list:expr, $not_in_list:expr) => {{
    let hashable_in_list: Option<Vec<u32>> =
      $in_list.map(|l| l.iter().map(|v| v.to_bits()).collect());

    let hashable_not_in_list: Option<Vec<u32>> =
      $not_in_list.map(|l| l.iter().map(|v| v.to_bits()).collect());

    super::validate_lists(hashable_in_list.as_deref(), hashable_not_in_list.as_deref())
      .unwrap_or_else(|invalid| {
        panic!(
          "The following values are present inside of 'in' and 'not_in': {:?}",
          invalid
            .iter()
            .map(|v| f32::from_bits(*v))
            .collect::<Vec<f32>>()
        )
      });
  }};

  (f64, $in_list:expr, $not_in_list:expr) => {{
    let hashable_in_list: Option<Vec<u64>> =
      $in_list.map(|l| l.iter().map(|v| v.to_bits()).collect());

    let hashable_not_in_list: Option<Vec<u64>> =
      $not_in_list.map(|l| l.iter().map(|v| v.to_bits()).collect());

    super::validate_lists(hashable_in_list.as_deref(), hashable_not_in_list.as_deref())
      .unwrap_or_else(|invalid| {
        panic!(
          "The following values are present inside of 'in' and 'not_in': {:?}",
          invalid
            .iter()
            .map(|v| f64::from_bits(*v))
            .collect::<Vec<f64>>()
        )
      });
  }};

  ($_:ty, $in_list:expr, $not_in_list:expr) => {{
    super::validate_lists($in_list, $not_in_list).unwrap_or_else(|invalid| {
      panic!(
        "The following values are present inside of 'in' and 'not_in': {:?}",
        invalid
      )
    });
  }};
}

macro_rules! get_fields {
  (f64, $_:ident) => {
    #[derive(Clone, Debug, Builder)]
    pub struct DoubleValidator<'a> {
      pub const_: Option<f64>,
      pub lt: Option<f64>,
      pub lte: Option<f64>,
      pub gt: Option<f64>,
      pub gte: Option<f64>,
      pub in_: Option<&'a [f64]>,
      pub not_in: Option<&'a [f64]>,
      #[builder(with = || true)]
      pub finite: Option<bool>,
      pub cel: Option<&'a [CelRule]>,
      #[builder(with = || true)]
      pub required: Option<bool>,
      #[builder(setters(vis = "", name = ignore))]
      pub ignore: Option<Ignore>,
    }
  };

  (f32, $_:ident) => {
    #[derive(Clone, Debug, Builder)]
    pub struct FloatValidator<'a> {
      pub const_: Option<f32>,
      pub lt: Option<f32>,
      pub lte: Option<f32>,
      pub gt: Option<f32>,
      pub gte: Option<f32>,
      pub in_: Option<&'a [f32]>,
      pub not_in: Option<&'a [f32]>,
      #[builder(with = || true)]
      pub finite: Option<bool>,
      pub cel: Option<&'a [CelRule]>,
      #[builder(with = || true)]
      pub required: Option<bool>,
      #[builder(setters(vis = "", name = ignore))]
      pub ignore: Option<Ignore>,
    }
  };

  ($rust_type:ident, $proto_type:ident) => {
    paste::paste! {
      #[derive(Clone, Debug, Builder)]
        pub struct [< $proto_type:camel Validator >]<'a> {
        pub const_: Option<$rust_type>,
        pub lt: Option<$rust_type>,
        pub lte: Option<$rust_type>,
        pub gt: Option<$rust_type>,
        pub gte: Option<$rust_type>,
        pub in_: Option<&'a [$rust_type]>,
        pub not_in: Option<&'a [$rust_type]>,
        pub cel: Option<&'a [CelRule]>,
        pub required: Option<bool>,
        #[builder(setters(vis = "", name = ignore))]
        pub ignore: Option<Ignore>,
      }
    }
  };
}

macro_rules! get_options {
  (Float, $validator:ident, $values:ident) => {
    insert_option!($validator, $values, lt, Float);
    insert_option!($validator, $values, lte, Float);
    insert_option!($validator, $values, gt, Float);
    insert_option!($validator, $values, gte, Float);
    insert_option!($validator, $values, in_, [Float]);
    insert_option!($validator, $values, not_in, [Float]);
    insert_option!($validator, $values, finite, bool);
  };

  ($option_value_variant:ident, $validator:ident, $values:ident) => {
    insert_option!($validator, $values, lt, $option_value_variant);
    insert_option!($validator, $values, lte, $option_value_variant);
    insert_option!($validator, $values, gt, $option_value_variant);
    insert_option!($validator, $values, gte, $option_value_variant);
    insert_option!($validator, $values, in_, [$option_value_variant]);
    insert_option!($validator, $values, not_in, [$option_value_variant]);
  };
}

macro_rules! numeric_validator {
  ($proto_type:ident, $rust_type:ty, $option_value_variant:ident) => {
    paste::paste! {
      get_fields!($rust_type, $proto_type);

      impl_ignore!([< $proto_type:camel ValidatorBuilder >]);

      impl<'a, S: [< $proto_type _validator_builder >]::State> From<[< $proto_type:camel ValidatorBuilder >]<'a, S>> for ProtoOption {
        #[track_caller]
        fn from(builder: [< $proto_type:camel ValidatorBuilder >]<S>) -> ProtoOption {
          builder.build().into()
        }
      }

      impl<'a> From<[< $proto_type:camel Validator >]<'a>> for ProtoOption {
        #[track_caller]
        fn from(validator: [< $proto_type:camel Validator >]) -> ProtoOption {
          let name = "(buf.validate.field)";

          let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

          if let Some(const_val) = validator.const_ {
            values.insert("const".into(), OptionValue::from(const_val));
            return ProtoOption {
              name: name.into(),
              value: OptionValue::Message(values).into(),
            };
          }

          super::validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte);
          get_list_check!($rust_type, validator.in_, validator.not_in);
          get_options!($option_value_variant, validator, values);

          let mut options_map: BTreeMap<Box<str>, OptionValue> = btreemap! {
            stringify!($proto_type).into() => OptionValue::Message(values)
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
      pub fn [< build_ $proto_type _validator_option >]<F, S>(config_fn: F) -> ProtoOption
      where
        F: FnOnce([< $proto_type:camel ValidatorBuilder >]) -> [< $proto_type:camel ValidatorBuilder >]<S>,
        S: [< $proto_type _validator_builder >]::IsComplete,
      {
        let builder = [< $proto_type:camel Validator >]::builder();
        let validator = config_fn(builder).build();
        validator.into()
      }
    }
  };
}

numeric_validator!(int64, i64, Int);
numeric_validator!(int32, i32, Int);
numeric_validator!(sint64, i64, Int);
numeric_validator!(sint32, i32, Int);
numeric_validator!(sfixed64, i64, Int);
numeric_validator!(sfixed32, i32, Int);
numeric_validator!(uint64, u64, Uint);
numeric_validator!(uint32, u32, Uint);
numeric_validator!(fixed64, u64, Uint);
numeric_validator!(fixed32, u32, Uint);
numeric_validator!(float, f32, Float);
numeric_validator!(double, f64, Float);
