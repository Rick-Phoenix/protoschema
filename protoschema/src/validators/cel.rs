use crate::OptionValue;

#[derive(Debug, Clone)]
pub struct CelRule {
  pub id: Box<str>,
  pub message: Box<str>,
  pub expression: Box<str>,
}

pub fn cel_rule<T: AsRef<str>>(id: T, message: T, expression: T) -> CelRule {
  CelRule {
    id: id.as_ref().into(),
    message: message.as_ref().into(),
    expression: expression.as_ref().into(),
  }
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
