use std::marker::PhantomData;

use bon::Builder;

use crate::{sealed, Empty, IsSet, ProtoOption, Set, Unset};

#[derive(Clone, Debug, Builder)]
pub struct OneofData {
  pub name: String,
  pub parent_message_id: usize,
  pub package: String,
  pub file: String,
  pub fields: Vec<usize>,
  pub options: Vec<ProtoOption>,
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
