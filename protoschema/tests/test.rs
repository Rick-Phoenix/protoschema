use paste::paste;
use protoschema::{
  field,
  fields::{self, build_string_validator_option},
  msg_field, parse_field_type, string, FieldType, OptionValue, Package, ProtoOption,
};

use crate::fields::Field;

#[test]
fn main_test() {
  let package = Package::default();

  let file = package.new_file("abc");

  let opt = ProtoOption {
    name: "abc",
    value: OptionValue::Bool(true),
  };

  let msg = file.new_message("MyMsg");

  let field = msg_field!(msg, mymsgfield = 5);

  let msg = msg
    .fields([
      field,
      string!(abc = 5),
      string!(abc = 5, |v| v.min_len(5).max_len(15)),
    ])
    .get_data();

  println!("{:#?}", msg);
}
