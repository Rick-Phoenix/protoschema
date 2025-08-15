use protoschema::{proto_str_list, FieldType, Message, OptionValue, ProtoField, ProtoOption};

#[test]
fn main_test() {
  let hello = Message::builder()
    .name("abc".into())
    .fields(vec![ProtoField {
      name: "abc".into(),
      ty: FieldType::String,
      tag: 12,
      options: vec![
        ProtoOption {
          name: "deprecated",
          value: OptionValue::Bool(true),
        },
        ProtoOption {
          name: "(buf.validate.field).string",
          value: proto_str_list!("val1", "val2"),
        },
      ],
    }])
    .build();
  println!("{}", askama::Template::render(&hello).unwrap());
}
