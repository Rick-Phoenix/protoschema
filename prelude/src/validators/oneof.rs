use std::sync::LazyLock;

use super::*;
use crate::*;

#[allow(clippy::type_complexity)]
static REQUIRED_OPTION: LazyLock<Arc<[(Arc<str>, OptionValue)]>> =
  LazyLock::new(|| [(REQUIRED.clone(), OptionValue::Bool(true))].into());

pub fn oneof_required() -> ProtoOption {
  ProtoOption {
    name: BUF_VALIDATE_ONEOF.clone(),
    value: OptionValue::Message(REQUIRED_OPTION.clone()),
  }
}
