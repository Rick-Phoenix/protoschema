#![allow(unused)]

#[macro_use]
mod macros;

use std::{cmp::Ordering, ops::Range};

pub(crate) use attributes::*;
use attributes::*;
pub(crate) use convert_case::ccase;
use proc_macro::TokenStream;
use proc_macro2::Span;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
  parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated, token::Token, Attribute,
  Data, DataEnum, DataStruct, DeriveInput, Error, Expr, ExprClosure, Fields, Ident, Item, ItemMod,
  Lit, LitStr, Meta, MetaList, Path, RangeLimits, Token, Type,
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

#[proc_macro_attribute]
pub fn proto_module(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let mut module = parse_macro_input!(input as ItemMod);

  let ModuleAttrs { file, package } = parse_macro_input!(attrs as ModuleAttrs);

  let injected_attr: Attribute = parse_quote! { #[proto(file = #file, package = #package)] };

  if let Some((_, content)) = &mut module.content {
    for item in content {
      match item {
        // TODO: Not adding the attributes if file and package are already defined
        Item::Struct(s) if has_proto_derive(&s.attrs).unwrap() => {
          s.attrs.push(injected_attr.clone());
        }
        Item::Enum(e) if has_proto_derive(&e.attrs).unwrap() => {
          e.attrs.push(injected_attr.clone());
        }
        _ => {}
      }
    }
  }

  quote!(#module).into()
}
