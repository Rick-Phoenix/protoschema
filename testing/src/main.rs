#![allow(unused)]

use std::{collections::HashMap, sync::Arc};

use prelude::{
  validators::{
    repeated_validator_builder::{SetItems, State},
    MapValidatorBuilder, ProtoMap, ProtoRepeated, RepeatedValidator, RepeatedValidatorBuilder,
    Sint32, StringValidator, StringValidatorBuilder,
  },
  EnumVariant, Message, Oneof, ProtoEnum, ProtoField, ProtoFile, ProtoOption, ProtoPath, ProtoType,
  ProtoValidator, ValidatorBuilderFor, ValidatorMap,
};
use proc_macro_impls::{proto_module, Enum, Message, Oneof};

pub trait ProtoMessage: ProtoType {}

// #[derive(Oneof)]
// enum PseudoOneof {
//   A(String),
//   B(u32),
// }

// #[derive(Enum)]
// enum Bcd {
//   AbcDeg,
//   B,
//   C,
// }

fn string_validator() -> StringValidatorBuilder {
  StringValidator::builder()
}

fn repeated_validator() -> impl ValidatorBuilderFor<Vec<i32>> {
  let validator: RepeatedValidatorBuilder<i32> = RepeatedValidator::builder();

  validator.items(|i| i.lt(20)).min_items(1)
}

#[proc_macro_impls::proto_module(file = "abc.proto", package = "myapp.v1")]
mod inner {
  use super::*;

  #[derive(Message)]
  pub struct Abc {
    #[proto(validate = string_validator())]
    name: String,
    // #[proto(validate = repeated_validator())]
    // num: Vec<i32>,
    // #[proto(type_(ProtoMap<String, Sint32>))]
    // #[proto(validate = |v| v.min_pairs(0).keys(|k| k.min_len(25)).values(|v| v.lt(25)))]
    // map: HashMap<String, i32>,
  }
}

use inner::*;

fn main() {
  let mut file = ProtoFile::new("abc.proto", "myapp.v1");

  let msg = Abc::to_message(&mut file);

  let msg2 = msg.clone();
  let nested = msg.nested_message(msg2);

  // let nested_enum = Bcd::to_nested_enum(nested);
}
