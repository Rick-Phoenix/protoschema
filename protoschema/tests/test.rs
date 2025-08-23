#![allow(clippy::cloned_ref_to_slice_refs)]

use askama::Template;
use protoschema::{
  enum_field, enum_map, enum_variants, extension, message_body, msg_field, msg_map, oneof,
  proto_enum, reusable_fields, schema::Package, services, string, OptionValue, ProtoOption,
};

#[test]
fn main_test() {
  let second_package = Package::new("myapp.v2");
  let external_file = second_package.new_file("myapp/v2/abcde.proto");
  let imported_msg = external_file.new_message("ExternalMsg");

  let package = Package::new("myapp.v1");

  let file = package.new_file("abc");

  let opt = ProtoOption {
    name: "abc".into(),
    value: OptionValue::Bool(true).into(),
  };

  let msg = file.new_message("MyMsg");
  let msg2 = file.new_message("MyMsg2");

  let field = msg_field!(repeated imported_msg, "my_msg_field", |r, i| r.items(i.cel(&[])));

  let reusable_variants = enum_variants!(
    1 => "ABC",
    2 => "BCD"
  );

  let example_enum = proto_enum!(
    file.new_enum("file_enum"),
    reserved_names = ["abc"],
    options = [opt.clone()],
    include(reusable_variants),
    1 => "UNSPECIFIED"
  );

  services!(
    file,
    MyService {
      options = [ opt.clone(), opt.clone() ];
      GetUser(msg => msg2) [ opt.clone() ];
      GetUser2(msg => msg2);
    };

    MyService2 {
      options = [ opt.clone(), opt.clone() ];
      GetUser(msg => msg2) [ opt.clone() ];
      GetUser2(msg => msg2);
    };
  );

  let reusable_fields = reusable_fields!(
    1 => string!("abc"),
    2 => string!("abc")
  );

  let isolated_field = string!("abc");

  message_body! {
    msg,

    options = [ opt.clone(), opt.clone() ],
    reserved_names = [ "one", "two" ],
    reserved = [ 2, 2..4 ],

    1 => isolated_field,
    2 => string!("abc").options([opt.clone(), opt.clone(), opt.clone()]),
    3 => string!(repeated "abc", |r, i| r.items(i.min_len(4).ignore_if_zero_value())),
    6 => enum_map!("abc", <string, example_enum>, |m, k, v| m.min_pairs(3).keys(k.min_len(15)).values(v.defined_only())),
    5 => enum_field!(example_enum, "enum_without_validator"),
    7 => enum_field!(example_enum, "enum_with_validator", |v| v.defined_only()),
    10 => enum_field!(repeated example_enum, "repeated_enum_field", |r, i| r.items(i.defined_only())),
    9 => msg_map!("abc", <string, msg2>, |m, k, v| m.min_pairs(15).keys(k.min_len(25)).values(v.cel(&[]))),

    enum "my_enum" {
      options = [ opt.clone() ],
      reserved_names = [ "one", "two" ],
      reserved = [ 1, 2..4 ],
      include(reusable_variants),

      1 => "UNSPECIFIED",
    }

    oneof "my_oneof" {
      options = [ opt.clone() ],

      6 => field.clone(),
      7 => field.clone()
    }

    10 => field.clone(),
  };

  extension!(file, msg2 {
    15 => string!("abc").options([opt.clone(), opt.clone(), opt.clone()])
  });

  let file_renders = &package.build_templates()[0];

  println!("{:#?}", file_renders);

  let render = file_renders.render().unwrap();

  println!("{}", render);
}
