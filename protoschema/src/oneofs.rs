use std::marker::PhantomData;

use bon::Builder;
pub(crate) use oneof_data_builder::*;

use crate::{fields::Field, sealed, Empty, IsSet, OptionValue, ProtoOption, Set, Unset};

pub const ONEOF_REQUIRED: ProtoOption = ProtoOption {
  name: "(buf.validate.oneof).required",
  value: OptionValue::Bool(true),
};

#[derive(Clone, Debug, Builder)]
pub struct OneofData {
  pub name: String,
  pub parent_message_id: usize,
  pub fields: Vec<Field>,
  #[builder(default)]
  #[builder(setters(vis = "", name = options_internal))]
  pub options: Box<[ProtoOption]>,
}

impl<S: oneof_data_builder::State> OneofDataBuilder<S> {
  pub fn options(
    self,
    options: &[ProtoOption],
  ) -> OneofDataBuilder<oneof_data_builder::SetOptions<S>>
  where
    S::Options: IsUnset,
  {
    self.options_internal(options.into())
  }
}

pub trait OneofState: ::core::marker::Sized {
  type Fields;
  type Options;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}
pub trait IsComplete: OneofState {
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}
#[doc(hidden)]
impl<S: OneofState> IsComplete for S
where
  S::Fields: IsSet,
  S::Options: IsSet,
{
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
mod members {
  pub struct fields;
  pub struct options;
}

pub struct SetFields<S: OneofState = Empty>(PhantomData<fn() -> S>);
pub struct SetOptions<S: OneofState = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl OneofState for Empty {
  type Fields = Unset<members::fields>;
  type Options = Unset<members::options>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: OneofState> OneofState for SetFields<S> {
  type Fields = Set<members::fields>;
  type Options = S::Options;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: OneofState> OneofState for SetOptions<S> {
  type Fields = S::Fields;
  type Options = Set<members::options>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
