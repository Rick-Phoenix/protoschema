#![allow(dead_code)]

use std::marker::PhantomData;

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

#[derive(Debug, Clone)]
pub struct Empty;

pub mod enums;
pub mod fields;
mod message;
pub mod oneofs;
pub mod rendering;
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

pub(crate) fn from_str_slice(strings: &[&str]) -> Box<[Box<str>]> {
  let mut vec = Vec::with_capacity(strings.len());
  vec.extend(strings.iter().map(|&s| s.into()));
  vec.into_boxed_slice()
}
