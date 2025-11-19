use std::marker::PhantomData;

use bon::Builder;
use repeated_validator_builder::{IsComplete, IsUnset, SetIgnore, State};

use super::*;
use crate::*;

pub struct ProtoRepeated<T>(PhantomData<T>);

macro_rules! impl_repeated_validator {
  ($name:ident) => {
    impl<T> ProtoValidator<$name<T>> for ValidatorMap
    where
      ValidatorMap: ProtoValidator<T>,
    {
      type Builder = RepeatedValidatorBuilder<T>;

      fn builder() -> Self::Builder {
        RepeatedValidator::builder()
      }
    }

    impl<T, S: State> ValidatorBuilderFor<$name<T>> for RepeatedValidatorBuilder<T, S> {}
  };
}

impl_repeated_validator!(ProtoRepeated);
impl_repeated_validator!(Vec);

impl<T, S: State> RepeatedValidatorBuilder<T, S>
where
  S::Items: repeated_validator_builder::IsUnset,
{
  #[track_caller]
  pub fn items<F, FinalBuilder>(
    self,
    config_fn: F,
  ) -> RepeatedValidatorBuilder<T, repeated_validator_builder::SetItems<S>>
  where
    F: FnOnce(<ValidatorMap as ProtoValidator<T>>::Builder) -> FinalBuilder,
    FinalBuilder: ValidatorBuilderFor<T>,
    ValidatorMap: ProtoValidator<T>,
  {
    let builder = ValidatorMap::builder();
    let options = config_fn(builder).into();
    self.items_internal(options)
  }
}

impl<S: State, T> RepeatedValidatorBuilder<T, S>
where
  S::Ignore: IsUnset,
{
  /// Rules set for this field will always be ignored.
  pub fn ignore_always(self) -> RepeatedValidatorBuilder<T, SetIgnore<S>> {
    self.ignore(Ignore::Always)
  }
}

#[derive(Clone, Debug, Builder)]
#[builder(state_mod(vis = "pub"))]
pub struct RepeatedValidator<T> {
  #[builder(default)]
  _inner_type: PhantomData<T>,

  #[builder(setters(vis = "", name = items_internal))]
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
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  /// Marks the field as required. Since repeated fields are always present in protobuf, this is essentially the same as setting min_items to 1
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl<T, S: State> From<RepeatedValidatorBuilder<T, S>> for ProtoOption
where
  S: IsComplete,
{
  #[track_caller]
  fn from(value: RepeatedValidatorBuilder<T, S>) -> Self {
    value.build().into()
  }
}

impl<T> From<RepeatedValidator<T>> for ProtoOption {
  #[track_caller]
  fn from(validator: RepeatedValidator<T>) -> ProtoOption {
    let mut rules: OptionValueList = Vec::new();

    insert_option!(validator, rules, unique);
    insert_option!(validator, rules, min_items);
    insert_option!(validator, rules, max_items);

    if let Some(items_option) = validator.items {
      rules.push((ITEMS.clone(), items_option.value.clone()));
    }

    let mut outer_rules: OptionValueList = vec![];

    outer_rules.push((REPEATED.clone(), OptionValue::Message(rules.into())));

    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, required);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}
