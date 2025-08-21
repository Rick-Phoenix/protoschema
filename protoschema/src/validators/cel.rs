use std::collections::BTreeMap;

use maplit::btreemap;

use crate::OptionValue;

#[derive(Debug, Clone)]
pub struct CelRule {
  pub id: Box<str>,
  pub message: Box<str>,
  pub expression: Box<str>,
}

impl From<CelRule> for OptionValue {
  fn from(value: CelRule) -> Self {
    let rule: BTreeMap<Box<str>, OptionValue> = btreemap! {
      "id".into() => OptionValue::String(value.id),
      "message".into() => OptionValue::String(value.message),
      "expression".into() => OptionValue::String(value.expression),
    };

    OptionValue::Message(rule)
  }
}
