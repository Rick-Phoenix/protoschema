use std::marker::PhantomData;

#[doc(hidden)]
pub use paste::paste;

#[doc(inline)]
pub use crate::{
  field_type::{FieldType, MapKey},
  options::*,
  packages::Package,
};

pub mod common;
pub mod enums;
pub mod errors;
pub mod extensions;
pub mod field_type;
pub mod fields;
pub mod files;
pub mod messages;
pub mod oneofs;
pub mod packages;
pub mod rendering;
pub mod services;
pub mod validators;
#[macro_use]
pub mod options;

#[macro_use]
pub mod macros;

mod sealed {
  pub struct Sealed;
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Set<T>(PhantomData<fn() -> T>);
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Unset<T>(PhantomData<fn() -> T>);

#[doc(hidden)]
pub trait IsSet {}
#[doc(hidden)]
pub trait IsUnset {}

#[doc(hidden)]
impl<T> IsSet for Set<T> {}
#[doc(hidden)]
impl<T> IsUnset for Unset<T> {}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct Empty;
