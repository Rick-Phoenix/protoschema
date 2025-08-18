use askama::Template;
use maplit::btreemap;
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

  let field = msg_field!(msg, mymsgfield);

  let msg = msg.fields(btreemap! {
    1 => field.clone(),
    2 => string!(abc).options(vec![opt.clone(), opt.clone(), opt.clone()]),
    3 => string!(abc, |v| v.min_len(5).max_len(15))
  });

  let msg2 = msg.new_message("MyNestedMsg");

  let field2 = msg_field!(msg2, mymsgfield2);

  let msg2 = msg2.fields(btreemap! {
    1 => field2.clone(),
  });

  let msg3 = file.new_message("MyMsg2").fields(btreemap! {
    1 => field2.clone(),
    2 => field.clone()
  });

  let file_renders = &package.build_templates()[0];

  let render = file_renders.render().unwrap();

  println!("{}", render);
}
