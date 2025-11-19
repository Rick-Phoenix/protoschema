use std::{fmt::Display, marker::PhantomData};

use bon::Builder;
use int_validator_builder::{IsUnset, SetIgnore, State};

use super::*;
use crate::*;

// Validator implementations
impl<N> ProtoValidator<IntValidator<N>> for ValidatorMap
where
  N: IntWrapper,
{
  type Builder = IntValidatorBuilder<N>;

  fn builder() -> IntValidatorBuilder<N> {
    IntValidator::builder()
  }
}

impl<N> ProtoValidator<FloatValidator<N>> for ValidatorMap
where
  N: FloatWrapper,
{
  type Builder = FloatValidatorBuilder<N>;

  fn builder() -> FloatValidatorBuilder<N> {
    FloatValidator::builder()
  }
}

impl<S: State, N: IntWrapper> IntValidatorBuilder<N, S>
where
  S::Ignore: IsUnset,
{
  /// Rules defined for this field will be ignored if the field is set to its protobuf zero value.
  pub fn ignore_if_zero_value(self) -> IntValidatorBuilder<N, SetIgnore<S>> {
    self.ignore(Ignore::IfZeroValue)
  }

  /// Rules set for this field will always be ignored.
  pub fn ignore_always(self) -> IntValidatorBuilder<N, SetIgnore<S>> {
    self.ignore(Ignore::Always)
  }
}

#[derive(Clone, Debug, Builder)]
pub struct IntValidator<Num>
where
  Num: IntWrapper,
{
  #[builder(default)]
  _wrapper: PhantomData<Num>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<Num::RustInt>,
  /// This field's value will be valid only if it is smaller than the specified amount
  pub lt: Option<Num::RustInt>,
  /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
  pub lte: Option<Num::RustInt>,
  /// This field's value will be valid only if it is greater than the specified amount
  pub gt: Option<Num::RustInt>,
  /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
  pub gte: Option<Num::RustInt>,
  /// Only the values in this list will be considered valid for this field.
  #[builder(into)]
  pub in_: Option<Arc<[Num::RustInt]>>,
  /// The values in this list will be considered invalid for this field.
  #[builder(into)]
  pub not_in: Option<Arc<[Num::RustInt]>>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

pub trait IntWrapper {
  type RustInt: PartialOrd + PartialEq + Copy + Into<OptionValue> + Hash + Debug + Display + Eq;

  fn proto_type() -> Arc<str>;
}

macro_rules! impl_int_wrapper {
  ($rust_type:ty, $proto_type:ident, primitive) => {
    impl IntWrapper for $rust_type {
      type RustInt = $rust_type;
      fn proto_type() -> Arc<str> {
        $proto_type.clone()
      }
    }

    impl_numeric_validator!($rust_type);
  };

  ($rust_type:ty, $wrapper_name:ident) => {
    pub struct $wrapper_name;

    impl IntWrapper for $wrapper_name {
      type RustInt = $rust_type;
      fn proto_type() -> Arc<str> {
        $crate::paste!([< $wrapper_name:upper >]).clone()
      }
    }

    impl_numeric_validator!($wrapper_name);
  };
}

macro_rules! impl_numeric_validator {
  ($rust_type:ty) => {
    impl ProtoValidator<$rust_type> for ValidatorMap {
      type Builder = IntValidatorBuilder<$rust_type>;

      fn builder() -> IntValidatorBuilder<$rust_type> {
        IntValidator::builder()
      }
    }

    impl<S: int_validator_builder::State> ValidatorBuilderFor<$rust_type>
      for IntValidatorBuilder<$rust_type, S>
    {
    }
  };
}

impl_int_wrapper!(i32, Sint32);
impl_int_wrapper!(i32, INT32, primitive);

impl<S: int_validator_builder::State, N> From<IntValidatorBuilder<N, S>> for ProtoOption
where
  S: int_validator_builder::IsComplete,
  N: IntWrapper,
{
  #[track_caller]
  fn from(value: IntValidatorBuilder<N, S>) -> Self {
    value.build().into()
  }
}

impl<N> From<IntValidator<N>> for ProtoOption
where
  N: IntWrapper,
{
  #[track_caller]
  fn from(validator: IntValidator<N>) -> Self {
    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      values.push((CONST_.clone(), const_val.into()));
    }

    validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte).unwrap();
    validate_lists(validator.in_.as_deref(), validator.not_in.as_deref()).unwrap();

    insert_option!(validator, values, lt);
    insert_option!(validator, values, lte);
    insert_option!(validator, values, gt);
    insert_option!(validator, values, gte);
    insert_option!(validator, values, in_);
    insert_option!(validator, values, not_in);

    let mut outer_rules: OptionValueList =
      vec![(N::proto_type(), OptionValue::Message(values.into()))];

    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, required);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}

pub trait FloatWrapper {
  type RustType: PartialOrd + PartialEq + Copy + Into<OptionValue> + Debug + Display;

  fn proto_type() -> Arc<str>;
}

impl FloatWrapper for f32 {
  type RustType = f32;

  fn proto_type() -> Arc<str> {
    FLOAT.clone()
  }
}

impl FloatWrapper for f64 {
  type RustType = f64;

  fn proto_type() -> Arc<str> {
    DOUBLE.clone()
  }
}

impl ProtoValidator<f32> for ValidatorMap {
  type Builder = FloatValidatorBuilder<f32>;

  fn builder() -> Self::Builder {
    FloatValidator::builder()
  }
}

impl<S: float_validator_builder::State> ValidatorBuilderFor<f32> for FloatValidatorBuilder<f32, S> {}

impl ProtoValidator<f64> for ValidatorMap {
  type Builder = FloatValidatorBuilder<f64>;

  fn builder() -> Self::Builder {
    FloatValidator::builder()
  }
}

impl<S: float_validator_builder::State> ValidatorBuilderFor<f64> for FloatValidatorBuilder<f64, S> {}

impl<Num: FloatWrapper> FloatValidator<Num> {
  pub(crate) fn validate_lists(&self) -> Result<(), Vec<Num::RustType>> {
    if let Some(in_list) = &self.in_ && let Some(not_in_list) = &self.not_in {
      let mut overlapping: Vec<Num::RustType> = Vec::new();

      let (shorter_list, longer_list) = if in_list.len() < not_in_list.len() {
        (in_list, not_in_list)
      } else {
        (not_in_list, in_list)
      };

      for num in longer_list.iter() {
        if shorter_list.contains(num) {
          overlapping.push(*num);
        }
      }

      if !overlapping.is_empty() {
        Err(overlapping)
      } else {
        Ok(())
      }

    } else {
      Ok(())
    }
  }
}

impl<S: float_validator_builder::State, N: FloatWrapper> FloatValidatorBuilder<N, S>
where
  S::Ignore: IsUnset,
{
  /// Rules defined for this field will be ignored if the field is set to its protobuf zero value.
  pub fn ignore_if_zero_value(
    self,
  ) -> FloatValidatorBuilder<N, float_validator_builder::SetIgnore<S>> {
    self.ignore(Ignore::IfZeroValue)
  }

  /// Rules set for this field will always be ignored.
  pub fn ignore_always(self) -> FloatValidatorBuilder<N, float_validator_builder::SetIgnore<S>> {
    self.ignore(Ignore::Always)
  }
}

#[derive(Clone, Debug, Builder)]
pub struct FloatValidator<Num>
where
  Num: FloatWrapper,
{
  #[builder(default)]
  _wrapper: PhantomData<Num>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<Num::RustType>,
  /// This field's value will be valid only if it is smaller than the specified amount
  pub lt: Option<Num::RustType>,
  /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
  pub lte: Option<Num::RustType>,
  /// This field's value will be valid only if it is greater than the specified amount
  pub gt: Option<Num::RustType>,
  /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
  pub gte: Option<Num::RustType>,
  /// Only the values in this list will be considered valid for this field.
  #[builder(into)]
  pub in_: Option<Arc<[Num::RustType]>>,
  /// The values in this list will be considered invalid for this field.
  #[builder(into)]
  pub not_in: Option<Arc<[Num::RustType]>>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
  /// Specifies that this field must be finite (so it can't represent Infinity or NaN)
  #[builder(with = || true)]
  pub finite: Option<bool>,
}

impl<S: float_validator_builder::State, N> From<FloatValidatorBuilder<N, S>> for ProtoOption
where
  S: float_validator_builder::IsComplete,
  N: FloatWrapper,
{
  #[track_caller]
  fn from(value: FloatValidatorBuilder<N, S>) -> Self {
    value.build().into()
  }
}

impl<N> From<FloatValidator<N>> for ProtoOption
where
  N: FloatWrapper,
{
  #[track_caller]
  fn from(validator: FloatValidator<N>) -> Self {
    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      values.push((CONST_.clone(), const_val.into()));
    }

    validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte).unwrap();
    validator.validate_lists().unwrap();

    insert_option!(validator, values, lt);
    insert_option!(validator, values, lte);
    insert_option!(validator, values, gt);
    insert_option!(validator, values, gte);
    insert_option!(validator, values, in_);
    insert_option!(validator, values, not_in);

    let mut outer_rules: OptionValueList = vec![];

    outer_rules.push((N::proto_type(), OptionValue::Message(values.into())));

    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, required);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}
