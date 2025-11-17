use std::marker::PhantomData;

use bon::Builder;

use super::*;
use crate::*;

pub struct Sint32;

#[derive(Clone, Debug, Builder)]
pub struct IntValidator<Num>
where
  Num: NumWrapper,
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

reusable_string!(SINT32);

macro_rules! impl_num_wrapper {
  ($rust_type:ty, $proto_type:ty) => {
    impl NumWrapper for $proto_type {
      type WrappedBuilder = IntValidatorBuilder<$proto_type>;
      type RustInt = $rust_type;
      fn proto_type() -> Arc<str> {
        $crate::paste!([< $proto_type:upper >]).clone()
      }

      fn wrapped_builder() -> Self::WrappedBuilder {
        IntValidator::builder()
      }
    }
  };
}

pub trait NumWrapper {
  type WrappedBuilder;
  type RustInt: PartialOrd + PartialEq + Copy + Into<OptionValue>;

  fn proto_type() -> Arc<str>;

  fn wrapped_builder() -> Self::WrappedBuilder;
}

impl_num_wrapper!(i32, Sint32);

impl<W> ProtoValidator<W> for ValidatorMap
where
  W: NumWrapper,
{
  type Builder<'a> = W::WrappedBuilder;
  fn builder() -> W::WrappedBuilder {
    W::wrapped_builder()
  }
}

impl<N> From<IntValidator<N>> for ProtoOption
where
  N: NumWrapper,
{
  fn from(validator: IntValidator<N>) -> Self {
    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      // values.push((CONST_.clone(), OptionValue::from(const_val)));
    }

    validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte);
    get_list_check!($rust_type, validator.in_.as_deref(), validator.not_in.as_deref());

    insert_option2!(validator, values, lt);
    insert_option2!(validator, values, lte);
    insert_option2!(validator, values, gt);
    insert_option2!(validator, values, gte);
    insert_option2!(validator, values, in_);
    insert_option2!(validator, values, not_in);

    let mut option_value: OptionValueList =
      vec![(N::proto_type(), OptionValue::Message(values.into()))];

    insert_cel_rule!(validator, option_value);
    insert_option!(validator, option_value, required, bool);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(option_value.into()),
    }
  }
}
