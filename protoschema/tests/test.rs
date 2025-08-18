use paste::paste;
use protoschema::{
  field,
  fields::{self, build_string_validator_option},
  msg_field, parse_field_type,
  schema::Package,
  string, FieldType, OptionValue, ProtoOption,
};

use crate::fields::Field;

#[test]
fn main_test() {
  let package = Package::new("myapp.v1");

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
      string!(abc = 5).options(vec![opt.clone(), opt.clone(), opt.clone()]),
      string!(abc = 5, |v| v.min_len(5).max_len(15)),
    ])
    .get_data();

  println!(
    "{:#?}",
    msg
      .fields
      .iter()
      .for_each(|f| { println!("{}", f.get_type_str("abcde", "myapp.v12")) })
  );
}
