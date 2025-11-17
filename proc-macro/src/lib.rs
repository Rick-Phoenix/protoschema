#![allow(unused)]

#[macro_use]
mod macros;

use std::{cmp::Ordering, ops::Range};

pub(crate) use attributes::*;
use attributes::*;
pub(crate) use convert_case::ccase;
use proc_macro::TokenStream;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
  parse::Parse, parse_macro_input, punctuated::Punctuated, token::Token, Attribute, Data, DataEnum,
  DataStruct, DeriveInput, Error, Expr, ExprClosure, Fields, Ident, Lit, LitStr, Meta, Path,
  RangeLimits, Token, Type,
};

use crate::{
  enum_derive::process_enum_derive, message_derive::process_message_derive,
  oneof_derive::process_oneof_derive,
};

mod enum_derive;
mod message_derive;
mod oneof_derive;

mod attributes;

#[proc_macro_derive(Message, attributes(proto))]
pub fn message_derive(input: TokenStream) -> TokenStream {
  process_message_derive(input)
}

#[proc_macro_derive(Enum, attributes(proto))]
pub fn enum_derive(input: TokenStream) -> TokenStream {
  process_enum_derive(input)
}

#[proc_macro_derive(Oneof, attributes(proto))]
pub fn oneof_derive(input: TokenStream) -> TokenStream {
  process_oneof_derive(input)
}
