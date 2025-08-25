use std::sync::{Arc, LazyLock};

use crate::{OptionValue, ProtoOption};

pub static DESCRIPTOR_PROTO_FILE: LazyLock<Arc<str>> =
  LazyLock::new(|| "google/protobuf/descriptor.proto".into());

pub fn oneof_required() -> ProtoOption {
  ProtoOption {
    name: "(buf.validate.oneof).required",
    value: Arc::new(OptionValue::Bool(true)),
  }
}

pub fn allow_alias() -> ProtoOption {
  ProtoOption {
    name: "allow_alias",
    value: Arc::new(OptionValue::Bool(true)),
  }
}

pub fn deprecated() -> ProtoOption {
  ProtoOption {
    name: "deprecated",
    value: Arc::new(OptionValue::Bool(true)),
  }
}
