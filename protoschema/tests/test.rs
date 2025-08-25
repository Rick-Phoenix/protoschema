use std::path::Path;

use proto_types::Duration;
use protoschema::{
  enum_field, enum_map, enum_option, enum_variants, extension, message, message_option, msg_field,
  msg_map,
  options::{list_value, proto_option},
  package::Package,
  proto_enum, services, string,
};

#[test]
fn main_test() -> Result<(), Box<dyn std::error::Error>> {
  let package = Package::new("myapp.v1");

  let second_package = Package::new("myapp.v2");
  let external_file = second_package.new_file("myapp/v2/abcde");
  let imported_msg = external_file.new_message("ExternalMsg");
  let imported_nested_msg = imported_msg.new_message("NestedMsg");
  let imported_enum = imported_msg.new_enum("ExternalEnum");

  let file = package.new_file("abc");

  let test_opts = [
    proto_option("string_opt", "abcde"),
    proto_option(
      "duration_opt",
      Duration {
        seconds: 3600,
        nanos: 0,
      },
    ),
    proto_option("enum_value", enum_option!("UNSPECIFIED")),
    proto_option(
      "message_option",
      message_option!(
        "subject" => "hobbits",
        "location" => "isengard",
      ),
    ),
    proto_option(
      "lists",
      message_option!(
        "num_list" => list_value([1, 2, 4, 5]),
        "string_list" => list_value(["hello", "there", "general", "kenobi"]),
        "list_of_messages" => list_value([
          message_option!("name" => "cats", "are_cute" => true),
          message_option!("name" => "dogs", "are_cute" => true),
        ]),
      ),
    ),
  ];

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
    reserved_names = ["abcde"],
    reserved = [ 405, 200..205 ],
    options = test_opts.clone(),
    0 => "UNSPECIFIED",
    1 => "ABC"
  );

  proto_enum!(
    file.new_enum("file_enum2"),
    0 => "UNSPECIFIED"
  );

  file.add_options(test_opts.clone());

  let msgclone = msg.clone();

  extension!(file, msgclone {
    15 => string!("abc").add_options(test_opts.clone()),
    19 => string!("abc").add_options(test_opts.clone()),
  });

  services!(
    file,
    MyService {
      options = test_opts.clone();
      GetUser(msg => msg2) { test_opts.clone() };
      GetUser2(msg => msg2);
    };

    MyService2 {
      options = test_opts.clone();
      GetUser(msg => msg2) { test_opts.clone() };
      GetUser2(msg => msg2);
    };
  );

  message! {
    msg3,

    options = test_opts.clone(),
    reserved_names = [ "one", "two" ],
    reserved = [ 405, 200..205 ],
    cel = [{ id = "abc", msg = "abc", expr = "abc" }],

    3 => string!(repeated "abc", |r, i| r.items(i.min_len(4).ignore_if_zero_value())),
    4 => enum_map!("abc", <string, example_enum>, |m, k, v| m.min_pairs(3).keys(k.min_len(15)).values(v.defined_only())),
    6 => enum_field!(example_enum, "enum_with_validator", |v| v.defined_only()),
    7 => enum_field!(repeated example_enum, "repeated_enum_field", |r, i| r.items(i.defined_only())),
    8 => msg_map!("abc", <string, msg2>, |m, k, _| m.min_pairs(15).keys(k.min_len(25))),

    enum "my_enum" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 405, 200..205 ],
      include(reusable_variants),
    }

    enum "my_enum2" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 405, 200..205 ],
      include(reusable_variants),
    }

    oneof "my_oneof" {
      options = test_opts.clone(),

      12 => field.clone().add_options(test_opts.clone())
    }
  };

  message! {
    msg,

    options = test_opts.clone(),
    reserved_names = [ "one", "two" ],
    reserved = [ 405, 200..205 ],
    cel = [{ id = "abc", msg = "abc", expr = "abc" }],

    2 => string!(optional "abc").add_options(test_opts.clone()),
    3 => string!(repeated "abc", |r, i| r.items(i.min_len(4).ignore_if_zero_value())),
    4 => enum_map!("abc", <string, example_enum>, |m, k, v| m.min_pairs(3).keys(k.min_len(15)).values(v.defined_only())),
    5 => enum_field!(optional example_enum, "enum_without_validator"),
    6 => enum_field!(example_enum, "enum_with_validator", |v| v.defined_only()),
    7 => enum_field!(repeated example_enum, "repeated_enum_field", |r, i| r.items(i.defined_only())),
    8 => msg_map!("abc", <string, msg2>, |m, k, _| m.min_pairs(15).keys(k.min_len(25))),
    9 => enum_field!(imported_enum, "imported_enum"),
    10 => field.clone(),

    enum "my_enum" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 405, 200..205 ],
      include(reusable_variants),
    }

    enum "my_enum2" {
      options = test_opts.clone(),
      reserved_names = [ "one", "two" ],
      reserved = [ 405, 200..205 ],
      include(reusable_variants),
    }

    oneof "my_oneof" {
      options = test_opts.clone(),

      12 => field.clone().add_options(test_opts.clone())
    }
  };

  extension!(file, msg2 {
    15 => string!("abc").add_options(test_opts.clone()),
    19 => string!("abc").add_options(test_opts.clone()),
  });

  let proto_root = Path::new("proto");
  package.render_templates(proto_root)?;
  Ok(())
}
