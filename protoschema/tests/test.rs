#![allow(clippy::cloned_ref_to_slice_refs)]

use std::{path::Path, sync::Arc};

use protoschema::{
  common_options::{deprecated, oneof_required},
  enum_field, enum_map, enum_variants, extension, message_body, msg_field, msg_map,
  package::Package,
  proto_enum, services, string,
  validators::cel::CelRule,
  OptionValue, ProtoOption,
};

#[test]
fn main_test() -> Result<(), Box<dyn std::error::Error>> {
  let second_package = Package::new("myapp.v2");
  let external_file = second_package.new_file("myapp/v2/abcde");
  let imported_msg = external_file.new_message("ExternalMsg");
  let imported_nested_msg = imported_msg.new_message("NestedMsg");
  let imported_enum = imported_msg.new_enum("ExternalEnum");

  let package = Package::new("myapp.v1");

  let file = package.new_file("abc");

  let opt = ProtoOption {
    name: "cel",
    value: Arc::new(OptionValue::Message(
      vec![
        (
          "list1".into(),
          OptionValue::List(
            vec![
              CelRule {
                id: "abc".into(),
                message: "abc".into(),
                expression: "abc".into(),
              }
              .into(),
              CelRule {
                id: "abc".into(),
                message: "abc".into(),
                expression: "abc".into(),
              }
              .into(),
            ]
            .into_boxed_slice(),
          ),
        ),
        (
          "list2".into(),
          OptionValue::List(
            vec![
              OptionValue::Int(4),
              OptionValue::Int(4),
              OptionValue::Int(4),
            ]
            .into(),
          ),
        ),
      ]
      .into_boxed_slice(),
    )),
  };

  let opt2 = deprecated();

  let test_opts = vec![opt.clone(), opt2.clone()];

  let msg = file.new_message("MyMsg");
  let msg2 = file.new_message("MyMsg2");
  let msg3 = msg.new_message("NestedMsg");

  let field = msg_field!(imported_nested_msg, "my_msg_field");

  let reusable_variants = enum_variants!(
    0 => "UNSPECIFIED",
    2 => "BCD"
  );

  let example_enum = proto_enum!(
    file.new_enum("file_enum"),
    reserved_names = ["abc"],
    options = test_opts.clone(),
    0 => "UNSPECIFIED",
    1 => "ABC"
  );

  proto_enum!(
    file.new_enum("file_enum2"),
    reserved_names = ["abc"],
    options = test_opts.clone(),
    0 => "UNSPECIFIED",
    1 => "ABC"
  );

  let file = file.add_options(test_opts.clone());

  let isolated_field = string!("abc");

  let msgclone = msg.clone();

  let file = extension!(file, msgclone {
    15 => string!("abc").add_options(test_opts.clone()),
    19 => string!("abc").add_options(test_opts.clone()),
    20 => isolated_field.clone()
  });

  services!(
    file,
    MyService {
      options = [ opt.clone(), opt2.clone() ];
      GetUser(msg => msg2) [ opt.clone(), opt2.clone() ];
      GetUser2(msg => msg2);
    };

    MyService2 {
      options = [ opt.clone(), opt2.clone() ];
      GetUser(msg => msg2) [ opt.clone(), opt2.clone() ];
      GetUser2(msg => msg2);
    };
  );

  message_body! {
    msg3,

    options = test_opts.clone(),
    reserved_names = [ "one", "two" ],
    reserved = [ 2, 2..4 ],
    cel = [{ id = "abc", msg = "abc", expr = "abc" }],

    1 => isolated_field.clone(),
    3 => string!(repeated "abc", |r, i| r.items(i.min_len(4).ignore_if_zero_value())),
    4 => enum_map!("abc", <string, example_enum>, |m, k, v| m.min_pairs(3).keys(k.min_len(15)).values(v.defined_only())),
    6 => enum_field!(example_enum, "enum_with_validator", |v| v.defined_only()),
    7 => enum_field!(repeated example_enum, "repeated_enum_field", |r, i| r.items(i.defined_only())),
    8 => msg_map!("abc", <string, msg2>, |m, k, _| m.min_pairs(15).keys(k.min_len(25))),

    enum "my_enum" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 1, 2..4 ],
      include(reusable_variants),
    }

    enum "my_enum2" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 1, 2..4 ],
      include(reusable_variants),
    }

    oneof "my_oneof" {
      options = test_opts.clone(),

      11 => isolated_field.clone(),
      12 => field.clone().add_options(test_opts.clone())
    }
  };

  message_body! {
    msg,

    options = test_opts.clone(),
    reserved_names = [ "one", "two" ],
    reserved = [ 2, 2..4 ],
    cel = [{ id = "abc", msg = "abc", expr = "abc" }],

    1 => isolated_field.clone(),
    2 => string!("abc").add_options(test_opts.clone()),
    3 => string!(repeated "abc", |r, i| r.items(i.min_len(4).ignore_if_zero_value())),
    4 => enum_map!("abc", <string, example_enum>, |m, k, v| m.min_pairs(3).keys(k.min_len(15)).values(v.defined_only())),
    5 => enum_field!(example_enum, "enum_without_validator"),
    6 => enum_field!(example_enum, "enum_with_validator", |v| v.defined_only()),
    7 => enum_field!(repeated example_enum, "repeated_enum_field", |r, i| r.items(i.defined_only())),
    8 => msg_map!("abc", <string, msg2>, |m, k, _| m.min_pairs(15).keys(k.min_len(25))),
    9 => enum_field!(imported_enum, "imported_enum"),
    10 => field.clone(),

    enum "my_enum" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 1, 2..4 ],
      include(reusable_variants),
    }

    enum "my_enum2" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 1, 2..4 ],
      include(reusable_variants),
    }

    oneof "my_oneof" {
      options = test_opts.clone(),

      11 => isolated_field.clone(),
      12 => field.clone().add_options([opt.clone(), opt.clone()])
    }
  };

  extension!(file, msg2 {
    15 => string!("abc").add_options(test_opts.clone()),
    19 => string!("abc").add_options(test_opts.clone()),
  });

  package.render_templates(Path::new("proto"))?;
  Ok(())
}
