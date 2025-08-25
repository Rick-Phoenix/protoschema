#![allow(dead_code)]

use std::marker::PhantomData;

pub use paste::paste;

pub use crate::{
  field_type::{FieldType, MapKey},
  options::{OptionValue, ProtoOption},
};

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

pub mod common_resources;
pub mod enums;
pub mod errors;
pub mod extensions;
pub mod fields;
pub mod files;
mod message;
pub mod oneofs;
pub mod package;
pub mod rendering;
pub mod services;
pub mod validators;

#[macro_use]
mod macros;

pub mod field_type;
#[macro_use]
pub mod options;

pub mod common_options {
  use std::sync::Arc;

  use crate::{OptionValue, ProtoOption};

  pub fn oneof_required() -> ProtoOption {
    ProtoOption {
      name: "(buf.validate.oneof).required",
      value: Arc::new(OptionValue::Bool(true)),
    }
  }

  pub fn deprecated() -> ProtoOption {
    ProtoOption {
      name: "deprecated",
      value: Arc::new(OptionValue::Bool(true)),
    }
  }
}
