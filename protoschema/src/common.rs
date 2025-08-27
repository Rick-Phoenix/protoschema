use std::sync::{Arc, LazyLock};

use crate::{OptionValue, ProtoOption};

/// The path to the validate.proto file.
pub static VALIDATE_PROTO_FILE: LazyLock<Arc<str>> =
  LazyLock::new(|| "buf/validate/validate.proto".into());

/// The path to the descriptor file in the google.protobuf package.
pub static DESCRIPTOR_PROTO_FILE: LazyLock<Arc<str>> =
  LazyLock::new(|| "google/protobuf/descriptor.proto".into());

/// The allow_alias option for enums.
pub fn allow_alias() -> ProtoOption {
  ProtoOption {
    name: "allow_alias",
    value: Arc::new(OptionValue::Bool(true)),
  }
}

/// A helper to create the [`ProtoOption`] that corresponds to 'deprecated = true'.
pub fn deprecated() -> ProtoOption {
  ProtoOption {
    name: "deprecated",
    value: Arc::new(OptionValue::Bool(true)),
  }
}
