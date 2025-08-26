use std::path::Path;

use proto_types::Duration;
use protoschema::{
  common::allow_alias,
  enum_field, enum_map, enum_option, enum_variants, extension, message, message_option, msg_field,
  msg_map,
  options::{list_value, proto_option},
  packages::Package,
  proto_enum, reusable_fields, services, string, timestamp, uint64,
};

#[test]
fn main_test() -> Result<(), Box<dyn std::error::Error>> {
  let package = Package::new("myapp.v1");

  let second_package = Package::new("myapp.v2");
  let external_file = second_package.new_file("post");
  let post_msg = external_file.new_message("Post");
  let post_status_enum = external_file.new_enum("post_status");
  let post_metadata_msg = post_msg.new_message("Metadata");
  let post_category_enum = post_msg.new_enum("post_category");

  let file = package.new_file("user");

  let example_option = proto_option(
    "example",
    message_option!(
      "num_list" => list_value([1, 2, 4, 5]),
      "string_list" => list_value(["hello", "there", "general", "kenobi"]),
      "list_of_messages" => list_value([
        message_option!("name" => "cats", "are_cute" => true),
        message_option!("name" => "dogs", "are_cute" => true),
      ]),
    ),
  );

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

  file.add_options(test_opts.clone());

  let user_msg = file.new_message("User");
  let section_msg = file.new_message("Section");
  let subsection_msg = section_msg.new_message("Subsection");
  let sub_subsection_msg = subsection_msg.new_message("SubSubsection");

  extension!(file, MessageOptions {
    15 => string!("abc"),
  });

  extension!(file, FileOptions {
    15 => string!("abc"),
  });

  let reusable_variants = enum_variants!(
    0 => "UNSPECIFIED",
  );

  let reusable_fields = reusable_fields!(
    100 => timestamp!("created_at"),
    101 => timestamp!("updated_at"),
  );

  let user_status_enum = proto_enum!(
    file.new_enum("user_status"),
    options = [ allow_alias() ],
    reserved = [ 405, 200..205 ],
    include(reusable_variants.clone()),
    1 => "ACTIVE" { [example_option.clone()] },
    2 => "INACTIVE",
    2 => "PASSIVE"
  );

  let referrers_enum = proto_enum!(
    file.new_enum("referrers"),

    include(reusable_variants.clone()),

    1 => "GITHUB",
    2 => "REDDIT"
  );

  services!(
    file,
    MyService1 {
      options = [ example_option.clone() ],
      Handler1(user_msg => section_msg) { [ example_option.clone() ] },
      Handler2(post_msg => post_metadata_msg),
    };
  );

  message! {
    user_msg,

    options = [ example_option.clone() ],
    reserved_names = [ "one", "two" ],
    reserved = [ 405, 200..205 ],
    cel = [
      {
        id = "passwords_match",
        msg = "the passwords do not match",
        expr = "this.password == this.repeated_password"
      }
    ],

    include(reusable_fields),
    1 => uint64!("id", |id| id.gt(0)),
    2 => msg_field!(repeated user_msg, "best_friend"),
    3 => string!("password", |pw| pw.min_len(8)),
    4 => string!("repeated_password", |pw| pw.min_len(8)),
    5 => enum_field!(user_status_enum, "last_status", |status| status.defined_only()),
    6 => enum_map!("last_30_days_statuses", <int32, user_status_enum>, |m, k, v| m.min_pairs(30).keys(k.lt(31)).values(v.defined_only())),
    7 => msg_map!("friends", <uint64, user_msg>),
    8 => msg_field!(post_msg, "last_post"),
    9 => enum_field!(post_status_enum, "last_post_status"),
    10 => msg_field!(post_metadata_msg, "last_post_metadata"),


    enum "favorite_category" {
      include(reusable_variants.clone()),

      1 => "PETS",
      2 => "COOKING"
    }

    enum "tier" {
      options = [ example_option.clone() ],
      reserved_names = [ "one", "two" ],
      reserved = [ 405, 200..205 ],
      include(reusable_variants.clone()),

      1 => "SILVER",
      2 => "GOLD"
    }

    oneof "contact" {
      options = [ example_option.clone() ],

      11 => string!("email", |v| v.email()),
      12 => enum_field!(referrers_enum, "referrer"),
    }
  };

  message! {
    subsection_msg,

    options = [ example_option.clone() ],

    1 => string!("name"),
    2 => msg_field!(post_msg, "top_trending_post"),
  };

  message! {
    sub_subsection_msg,

    1 => uint64!("id"),
    2 => string!("name"),
    3 => enum_field!(post_category_enum, "category"),
  };

  let proto_root = Path::new("proto");
  package.render_templates(proto_root)?;
  Ok(())
}
