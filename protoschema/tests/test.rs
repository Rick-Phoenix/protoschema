use paste::paste;
use protoschema::{
  add_field, field, message_fields, msg_field, parse_field_type, string, FieldData, FieldType,
  OptionValue, Package, ProtoOption,
};

#[test]
fn main_test() {
  let schema = Package::default();

  let file = schema.new_file("abc");

  let opt = ProtoOption {
    name: "abc",
    value: OptionValue::Bool(true),
  };

  let msg = file.new_message("MyMsg");

  let field = msg_field!(msg, abc = 5);

  let msg = message_fields!(
    msg,
    [
      string!(abc = 5),
      string!(abc = 5),
      string!(abc = 5),
      string!(abc = 5, [opt.clone(), opt]),
      field
    ]
  );

  let built = msg.build(&schema);

  built.get_message_type();
}
