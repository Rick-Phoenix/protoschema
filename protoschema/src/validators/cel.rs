use crate::OptionValue;

#[derive(Debug, Clone)]
pub struct CelRule {
  pub id: Box<str>,
  pub message: Box<str>,
  pub expression: Box<str>,
}

impl From<CelRule> for OptionValue {
  fn from(value: CelRule) -> Self {
    OptionValue::Message(
      vec![
        ("id".into(), OptionValue::String(value.id)),
        ("message".into(), OptionValue::String(value.message)),
        ("expression".into(), OptionValue::String(value.expression)),
      ]
      .into_boxed_slice(),
    )
  }
}
