use std::path::Path;

use prost_build::Config;
use protocheck_build::compile_protos_with_validators;
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
    100 => timestamp!("created_at"),
    101 => timestamp!("updated_at"),
  );

  let user_msg = file.new_message("User");
  let section_msg = file.new_message("Section");
  let subsection_msg = section_msg.new_message("Subsection");
  let sub_subsection_msg = subsection_msg.new_message("SubSubsection");

  let second_package = Package::new("myapp.v2");
  let external_file = second_package.new_file("post");
  let post_msg = external_file.new_message("Post");
  // Defining variants using the builder syntax
  let post_status_enum = external_file
    .new_enum("post_status")
    .variants(reusable_variants.clone());
  let post_category_enum = post_msg
    .new_enum("post_category")
    .variants(reusable_variants.clone());
  let post_metadata_msg = post_msg.new_message("Metadata");

  let user_status_enum = proto_enum!(
    file.new_enum("user_status"),
    options = [ allow_alias() ],
    reserved = [ 405, 200..205 ],
    include(reusable_variants),
    1 => "ACTIVE",
    2 => "INACTIVE",
    2 => "PASSIVE"
  );

  let reusable_oneof = oneof!("activity",
    102 => enum_field!(user_status_enum, "user_current_status"),
    103 => timestamp!("account_deletion_date")
  );

  let referrers_enum = proto_enum!(
    file.new_enum("referrers"),

    include(reusable_variants),

    1 => "GITHUB",
    2 => "REDDIT"
  );

  services!(
    file,
    MyService1 {
      Handler1(user_msg => section_msg),
      Handler2(post_msg => post_metadata_msg),
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

  message! {
    subsection_msg,

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
  second_package.render_templates(proto_root)?;

  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not find OUT_DIR"));
  let final_descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  let proto_files = &["proto/myapp/v1/user.proto", "proto/myapp/v2/post.proto"];

  let mut config = Config::new();
  config
    .file_descriptor_set_path(final_descriptor_path.clone())
    .bytes(["."])
    .enable_type_names()
    .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
    .out_dir(out_dir.clone());

  compile_protos_with_validators(&mut config, proto_files, proto_include_paths, &["myapp.v1"])?;

  config.compile_protos(proto_files, proto_include_paths)?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    final_descriptor_path.display()
  );
  Ok(())
}
