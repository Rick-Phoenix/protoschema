use paste::paste;
use protoschema::{
  field, msg_field, parse_field_type, string, Field, FieldType, OptionValue, Package, ProtoOption,
};

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
      string!(abc = 5, [opt.clone(), opt]),
    ])
    .build();

  println!("{:#?}", msg);
}
