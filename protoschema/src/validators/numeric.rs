use bon::Builder;

use crate::{
  validators::{cel::CelRule, Ignore, OptionValueList},
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
    /// A struct that can be used to generate a [`ProtoOption`] containing protovalidate rules for a protobuf 'double' field.
    #[derive(Clone, Debug, Builder)]
    pub struct DoubleValidator {
      /// Only this specific value will be considered valid for this field.
      pub const_: Option<f64>,
      /// This field's value will be valid only if it is smaller than the specified amount
      pub lt: Option<f64>,
      /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
      pub lte: Option<f64>,
      /// This field's value will be valid only if it is greater than the specified amount
      pub gt: Option<f64>,
      /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
      pub gte: Option<f64>,
      /// Only the values in this list will be considered valid for this field.
      #[builder(into)]
      pub in_: Option<Box<[f64]>>,
      /// The values in this list will be considered invalid for this field.
      #[builder(into)]
      pub not_in: Option<Box<[f64]>>,
      /// Specifies that this field must be finite (so it can't represent Infinity or NaN)
      #[builder(with = || true)]
      pub finite: Option<bool>,
      /// Adds custom validation using one or more [`CelRule`]s to this field.
      #[builder(into)]
      pub cel: Option<Box<[CelRule]>>,
      /// Marks the field as invalid if unset.
      #[builder(with = || true)]
      pub required: Option<bool>,
      #[builder(setters(vis = "", name = ignore))]
      pub ignore: Option<Ignore>,
    }
  };

  (f32, $_:ident) => {
    /// A struct that can be used to generate a [`ProtoOption`] containing protovalidate rules for a protobuf 'float' field.
    #[derive(Clone, Debug, Builder)]
    pub struct FloatValidator {
      /// Only this specific value will be considered valid for this field.
      pub const_: Option<f32>,
      /// This field's value will be valid only if it is smaller than the specified amount
      pub lt: Option<f32>,
      /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
      pub lte: Option<f32>,
      /// This field's value will be valid only if it is greater than the specified amount
      pub gt: Option<f32>,
      /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
      pub gte: Option<f32>,
      /// Only the values in this list will be considered valid for this field.
      #[builder(into)]
      pub in_: Option<Box<[f32]>>,
      /// The values in this list will be considered invalid for this field.
      #[builder(into)]
      pub not_in: Option<Box<[f32]>>,
      #[builder(with = || true)]
      /// Specifies that this field must be finite (so it can't represent Infinity or NaN)
      pub finite: Option<bool>,
      /// Adds custom validation using one or more [`CelRule`]s to this field.
      #[builder(into)]
      pub cel: Option<Box<[CelRule]>>,
      /// Marks the field as invalid if unset.
      #[builder(with = || true)]
      pub required: Option<bool>,
      #[builder(setters(vis = "", name = ignore))]
      pub ignore: Option<Ignore>,
    }
  };

  ($rust_type:ident, $proto_type:ident) => {
    paste::paste! {
      #[doc = concat!("A struct that can be used to generate a [`ProtoOption`] containing protovalidate rules for a protobuf `")]
      #[doc = concat!(stringify!($proto_type))]
      #[doc = "` field."]
      #[derive(Clone, Debug, Builder)]
        pub struct [< $proto_type:camel Validator >] {
        /// Only this specific value will be considered valid for this field.
        pub const_: Option<$rust_type>,
        /// This field's value will be valid only if it is smaller than the specified amount
        pub lt: Option<$rust_type>,
        /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
        pub lte: Option<$rust_type>,
        /// This field's value will be valid only if it is greater than the specified amount
        pub gt: Option<$rust_type>,
        /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
        pub gte: Option<$rust_type>,
        /// Only the values in this list will be considered valid for this field.
        #[builder(into)]
        pub in_: Option<Box<[$rust_type]>>,
        /// The values in this list will be considered invalid for this field.
        #[builder(into)]
        pub not_in: Option<Box<[$rust_type]>>,
        /// Adds custom validation using one or more [`CelRule`]s to this field.
        #[builder(into)]
        pub cel: Option<Box<[CelRule]>>,
        /// Marks the field as invalid if unset.
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

      impl_ignore!(no_lifetime, [< $proto_type:camel ValidatorBuilder >]);

      impl<S: [< $proto_type _validator_builder >]::State> From<[< $proto_type:camel ValidatorBuilder >]<S>> for ProtoOption {
        #[doc(hidden)]
        #[track_caller]
        fn from(builder: [< $proto_type:camel ValidatorBuilder >]<S>) -> ProtoOption {
          builder.build().into()
        }
      }

      impl From<[< $proto_type:camel Validator >]> for ProtoOption {
        #[doc(hidden)]
        #[track_caller]
        fn from(validator: [< $proto_type:camel Validator >]) -> ProtoOption {
          let name = "(buf.validate.field)";

          let mut values: OptionValueList = Vec::new();

          if let Some(const_val) = validator.const_ {
            values.push(("const".into(), OptionValue::from(const_val)));
          }

          super::validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte);
          get_list_check!($rust_type, validator.in_.as_deref(), validator.not_in.as_deref());
          get_options!($option_value_variant, validator, values);

          let mut option_value: OptionValueList = vec! [
            (stringify!($proto_type).into(), OptionValue::Message(values.into_boxed_slice()))
          ];

          insert_cel_rule!(validator, option_value);
          insert_option!(validator, option_value, required, bool);

          ProtoOption {
            name: name.into(),
            value: OptionValue::Message(option_value.into_boxed_slice()).into(),
          }
        }
      }

      #[doc(hidden)]
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
