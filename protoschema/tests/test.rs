use askama::Template;
use maplit::btreemap;
use paste::paste;
use protoschema::{
  fields::{self, build_string_validator_option},
  message_body, msg_field, oneof,
  oneofs::OneofData,
  parse_field_type,
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

  let msg = message_body! {
    msg,
    options = vec![ opt.clone() ],
    1 => field.clone(),
    2 => string!("abc").options(vec![opt.clone(), opt.clone(), opt.clone()]),
    3 => string!("abc", |v| v.min_len(5).max_len(15)),
    10 => field.clone(),

    oneof "my_oneof" {
      options = vec![ opt.clone() ],
      6 => field.clone(),
      7 => field.clone()
    },

    enum "my_enum" {
      reserved_names = [ "one", "two" ],
      reserved = [ 1, 2..4 ],
      1 => "UNSPECIFIED",
    }

  };

  println!("{:#?}", msg.get_data());

  let file_renders = &package.build_templates()[0];

  let render = file_renders.render().unwrap();

  println!("{}", render);
}
