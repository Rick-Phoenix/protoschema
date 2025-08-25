use std::sync::{Arc, LazyLock};

pub static DESCRIPTOR_PROTO_FILE: LazyLock<Arc<str>> =
  LazyLock::new(|| "google/protobuf/descriptor.proto".into());

static GOOGLE_PROTOBUF_PACKAGE: LazyLock<Arc<str>> = LazyLock::new(|| "google.protobuf".into());
