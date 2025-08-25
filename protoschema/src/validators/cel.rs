use crate::OptionValue;

/// The structure of a custom Cel rule used to define validation logic with protovalidate.
/// The id should be a unique identifier for the given rule, which will appear in error messages.
/// The message is the error message that will show up in case of validation failure.
/// The expression is a [Cel](https://cel.dev/) expression that will be executed on validation by protovalidate-compatible libraries such as [protocheck](https://github.com/Rick-Phoenix/protocheck) (for rust) or [protovalidate-es](https://github.com/bufbuild/protovalidate-es) (for javascript).
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
