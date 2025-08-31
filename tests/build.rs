use std::path::{Path, PathBuf};

use prost_build::Config;
use protocheck_build::{compile_protos_with_validators, get_proto_files_recursive};
use protoschema::{
  common::allow_alias, enum_field, enum_map, enum_variants, extension, message, msg_field, msg_map,
  oneof, packages::Package, proto_enum, reusable_fields, services, string, timestamp, uint64,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let package = Package::new("myapp.v1");
  let file = package.new_file("user");

  extension!(file, MessageOptions {
    1565 => string!("my_custom_message_option"),
  });

  extension!(file, FileOptions {
    1565 => string!("my_custom_file_option"),
  });

  let reusable_variants = enum_variants!(
    0 => "UNSPECIFIED",
  );

  let reusable_fields = reusable_fields!(
    1 => uint64!("id"),
    2 => timestamp!("created_at"),
    3 => timestamp!("updated_at"),
  );

  let user_status_enum = proto_enum!(
    file.new_enum("user_status"),
    options = [ allow_alias() ],
    reserved = [ 405, 200..205 ],
    include(reusable_variants),
    1 => "ACTIVE",
    2 => "INACTIVE",
    2 => "PASSIVE"
  );

  let referrers_enum = proto_enum!(
    file.new_enum("referrers"),

    include(reusable_variants),

    1 => "GITHUB",
    2 => "REDDIT"
  );

  let user_msg = file.new_message("User");

  let user_request_msg = message!(file.new_message("GetUserRequest"), include(reusable_fields));

  let second_package = Package::new("myapp.v2");
  let external_file = second_package.new_file("post");
  let post_msg = external_file.new_message("Post");
  let post_request_msg = message!(
    external_file.new_message("GetPostRequest"),
    include(reusable_fields),

    4 => uint64!("user_id"),
  );

  let reusable_oneof = oneof!("activity",
    102 => enum_field!(user_status_enum, "user_current_status"),
    103 => timestamp!("account_deletion_date")
  );

  services!(
    file,
    UserService {
      GetUser(user_request_msg => user_msg),
      GetPostByUserId(post_request_msg => post_msg),
    };
  );

  message! {
    user_msg,

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
    include_oneof(reusable_oneof),
    4 => msg_field!(repeated user_msg, "best_friend"),
    5 => string!("password", |pw| pw.min_len(8)),
    6 => string!("repeated_password", |pw| pw.min_len(8)),
    7 => enum_field!(user_status_enum, "last_status", |status| status.defined_only()),
    8 => enum_map!("last_30_days_statuses", <int32, user_status_enum>, |m, k, v| m.min_pairs(30).keys(k.lt(31)).values(v.defined_only())),
    9 => msg_map!("friends", <uint64, user_msg>),
    10 => msg_field!(post_msg, "last_post"),

    enum "favorite_category" {
      include(reusable_variants),

      1 => "PETS",
      2 => "COOKING"
    }

    enum "tier" {
      reserved_names = [ "one", "two" ],
      reserved = [ 405, 200..205 ],
      include(reusable_variants),

      1 => "SILVER",
      2 => "GOLD"
    }

    oneof "contact" {
      11 => string!("email", |v| v.email()),
      12 => enum_field!(referrers_enum, "referrer"),
    }
  };

  let proto_root = Path::new("proto");
  package.render_templates(proto_root)?;
  second_package.render_templates(proto_root)?;

  // End of the protoschema setup.
  // Now we just use protocheck with the generated files.

  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not find OUT_DIR"));
  let final_descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  let proto_files = get_proto_files_recursive(&PathBuf::from("proto/myapp"))?;

  let mut config = Config::new();
  config
    .file_descriptor_set_path(final_descriptor_path.clone())
    .bytes(["."])
    .enable_type_names()
    .out_dir(out_dir.clone());

  compile_protos_with_validators(
    &mut config,
    &proto_files,
    proto_include_paths,
    &["myapp.v1", "myapp.v2"],
  )?;

  config.compile_protos(&proto_files, proto_include_paths)?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    final_descriptor_path.display()
  );
  Ok(())
}
