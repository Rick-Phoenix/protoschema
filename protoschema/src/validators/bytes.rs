use bon::Builder;

use crate::{
  validators::{cel::CelRule, validate_lists, Ignore, OptionValueList},
  OptionValue, ProtoOption,
};

macro_rules! insert_bytes_option {
  ($validator:ident, $values:ident, $field:ident) => {
    $validator.$field.map(|v| {
      $values.push((
        stringify!($field).into(),
        OptionValue::String(format_bytes_as_proto_string_literal(v).into()),
      ))
    })
  };

  ($validator:ident, $values:ident, $field:ident, list) => {
    $validator.$field.map(|v| {
      $values.push((
        stringify!($field).into(),
        OptionValue::List(
          v.iter()
            .map(|i| OptionValue::String(format_bytes_as_proto_string_literal(i).into()))
            .collect::<Vec<OptionValue>>()
            .into_boxed_slice(),
        ),
      ))
    })
  };
}

/// Used by the [`bytes`](crate::bytes) macro to define validation rules.
#[derive(Clone, Debug, Builder)]
pub struct BytesValidator<'a> {
  /// Specifies the exact length for this bytes field to be considered valid.
  pub len: Option<u64>,
  /// The minimum length for this field in order to be considered valid.
  pub min_len: Option<u64>,
  /// The maximum length for this field in order to be considered valid.
  pub max_len: Option<u64>,
  /// The pattern that this field must match in order to be valid.
  pub pattern: Option<&'a str>,
  /// A prefix that this field must have in order to be valid.
  pub prefix: Option<&'a [u8]>,
  /// A suffix that this field must have in order to be valid.
  pub suffix: Option<&'a [u8]>,
  /// A subset of bytes that this field must contain in order to be valid.
  pub contains: Option<&'a [u8]>,
  /// Only the values in this list will be considered valid for this field.
  pub in_: Option<&'a [&'a [u8]]>,
  /// The values in this list will be considered invalid for this field.
  pub not_in: Option<&'a [&'a [u8]]>,
  #[builder(setters(vis = "", name = well_known))]
  pub well_known: Option<WellKnown>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<&'a [u8]>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  pub cel: Option<&'a [CelRule]>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_ignore!(BytesValidatorBuilder);

impl<'a, S: bytes_validator_builder::State> From<BytesValidatorBuilder<'a, S>> for ProtoOption {
  #[track_caller]
  fn from(value: BytesValidatorBuilder<'a, S>) -> Self {
    value.build().into()
  }
}

impl<'a> From<BytesValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: BytesValidator<'a>) -> Self {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      values.push((
        "const".into(),
        OptionValue::String(format_bytes_as_proto_string_literal(const_val).into()),
      ));
    }

    validate_lists(validator.in_, validator.not_in).unwrap_or_else(|invalid| {
      panic!(
        "The following values are present inside of 'in' and 'not_in': {:?}",
        invalid
      )
    });

    if validator.len.is_none() {
      insert_option!(validator, values, min_len, uint);
      insert_option!(validator, values, max_len, uint);
    } else {
      insert_option!(validator, values, len, uint);
    }

    insert_option!(validator, values, pattern, string);
    insert_bytes_option!(validator, values, contains);
    insert_bytes_option!(validator, values, prefix);
    insert_bytes_option!(validator, values, suffix);
    insert_bytes_option!(validator, values, in_, list);
    insert_bytes_option!(validator, values, not_in, list);

    if let Some(v) = validator.well_known {
      v.to_option(&mut values)
    }

    let mut option_value: OptionValueList = vec![(
      "bytes".into(),
      OptionValue::Message(values.into_boxed_slice()),
    )];

    insert_cel_rule!(validator, option_value);
    insert_option!(validator, option_value, required, bool);

    ProtoOption {
      name,
      value: OptionValue::Message(option_value.into_boxed_slice()).into(),
    }
  }
}

#[doc(hidden)]
#[track_caller]
pub fn build_bytes_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(BytesValidatorBuilder) -> BytesValidatorBuilder<S>,
  S: bytes_validator_builder::IsComplete,
{
  let builder = BytesValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}

#[derive(Clone, Debug, Copy)]
pub enum WellKnown {
  Ip,
  Ipv4,
  Ipv6,
}

use bytes_validator_builder::{IsUnset, SetWellKnown, State};

macro_rules! well_known_impl {
  ($name:ident) => {
    paste::paste! {
      pub fn [< $name:snake >](self) -> BytesValidatorBuilder<'a, SetWellKnown<S>>
        where
          S::WellKnown: IsUnset,
        {
          self.well_known(WellKnown::$name)
        }
    }
  };
}

impl<'a, S: State> BytesValidatorBuilder<'a, S> {
  well_known_impl!(Ip);
  well_known_impl!(Ipv4);
  well_known_impl!(Ipv6);
}

impl WellKnown {
  pub(crate) fn to_option(self, option_values: &mut OptionValueList) {
    match self {
      WellKnown::Ip => option_values.push(("ip".into(), OptionValue::Bool(true))),
      WellKnown::Ipv4 => option_values.push(("ipv4".into(), OptionValue::Bool(true))),
      WellKnown::Ipv6 => option_values.push(("ipv6".into(), OptionValue::Bool(true))),
    };
  }
}

fn format_bytes_as_proto_string_literal(bytes: &[u8]) -> String {
  let mut result = String::new();
  result.push('"');

  for &byte in bytes {
    match byte {
      0x20..=0x21 | 0x23..=0x5B | 0x5D..=0x7E => {
        result.push(byte as char);
      }
      b'\\' => result.push_str("\\\\"),
      b'"' => result.push_str("\\\""),
      _ => {
        result.push_str(&format!("\\x{:02x}", byte));
      }
    }
  }

  result.push('"');
  result
}
