#![allow(clippy::cloned_ref_to_slice_refs)]

use askama::Template;
use paste::paste;
use protoschema::{
  double, message_body, msg_field, proto_enum, schema::Package, string, OptionValue, ProtoOption,
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

  message_body! {
    msg,

    options = [ opt.clone(), opt.clone() ],
    reserved_names = [ "one", "two" ],
    reserved = [ 2, 2..4 ],

    1 => field.clone(),
    2 => string!("abc").options(&[opt.clone(), opt.clone(), opt.clone()]),
    3 => string!("abc", |v| v.min_len(5).max_len(15).email()),
    5 => double!("abc", |v| v.lt(5.1).gt(6.1)),

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

  proto_enum!(
    file.new_enum("file_enum"),
    options = [opt.clone()],
    1 => "UNSPECIFIED"
  );

  let file_renders = &package.build_templates()[0];

  println!("{:#?}", file_renders);

  let render = file_renders.render().unwrap();

  println!("{}", render);
}
