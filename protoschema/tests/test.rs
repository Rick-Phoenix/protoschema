#![allow(clippy::cloned_ref_to_slice_refs)]

use askama::Template;
use protoschema::{
  enum_field, enum_map, message_body, msg_field, proto_enum, schema::Package, string, OptionValue,
  ProtoOption,
};

#[test]
fn main_test() {
  let package = Package::new("myapp.v1");

  let file = package.new_file("abc");

  let opt = ProtoOption {
    name: "abc",
    value: OptionValue::Bool(true),
  };

  let msg = file.new_message("MyMsg");

  let field = msg_field!(msg, "my_msg_field");

  let example_enum = proto_enum!(
    file.new_enum("file_enum"),
    options = [opt.clone()],
    1 => "UNSPECIFIED"
  );

  message_body! {
    msg,

    options = [ opt.clone(), opt.clone() ],
    reserved_names = [ "one", "two" ],
    reserved = [ 2, 2..4 ],

    2 => string!("abc").options(&[opt.clone(), opt.clone(), opt.clone()]),
    6 => enum_map!("abc", <string, example_enum>, |m, k, v| m.min_pairs(3).keys(k.min_len(15)).values(v.defined_only(true))),
    7 => enum_field!(example_enum, "enum_field", |v| v.defined_only(true)),

    enum "my_enum" {
      options = [ opt.clone() ],
      reserved_names = [ "one", "two" ],
      reserved = [ 1, 2..4 ],

      1 => "UNSPECIFIED",
    }

    oneof "my_oneof" {
      options = [ opt.clone() ],

      6 => field.clone(),
      7 => field.clone()
    }

    10 => field.clone(),

  };

  let file_renders = &package.build_templates()[0];

  println!("{:#?}", file_renders);

  let render = file_renders.render().unwrap();

  println!("{}", render);
}
