#![allow(dead_code)]

use std::marker::PhantomData;

pub use field_type::ImportedItemPath;

pub use crate::{
  field_type::FieldType,
  option::{OptionValue, ProtoOption},
};

mod sealed {
  pub struct Sealed;
}

#[derive(Clone, Debug)]
pub struct Set<T>(PhantomData<fn() -> T>);
#[derive(Clone, Debug)]
pub struct Unset<T>(PhantomData<fn() -> T>);

pub trait IsSet {}
pub trait IsUnset {}

impl<T> IsSet for Set<T> {}
impl<T> IsUnset for Unset<T> {}

pub struct Empty;

pub mod enums;
pub mod fields;
mod message;
pub mod oneofs;
pub mod schema;
#[macro_use]
mod macros;
mod field_type;
#[macro_use]
mod option;

#[derive(Clone, Debug)]
pub struct Range {
  pub start: u32,
  pub end: u32,
}
