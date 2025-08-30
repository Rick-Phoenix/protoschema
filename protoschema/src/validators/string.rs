use bon::Builder;
use regex::Regex;

use crate::{
  validators::{cel::CelRule, validate_lists, Ignore, OptionValueList},
  OptionValue, ProtoOption,
};

/// Used by the [`string`](crate::string) macro to define validation rules.
#[derive(Clone, Debug, Builder)]
#[builder(derive(Clone))]
pub struct StringValidator<'a> {
  /// The exact character length that this field's value must have in order to be considered valid.
  pub len: Option<u64>,
  /// The minimum character length for this field's value to be considered valid.
  pub min_len: Option<u64>,
  /// The maximum character length for this field's value to be considered valid.
  pub max_len: Option<u64>,
  /// The exact byte length that this field's value must have in order to be considered valid.
  pub len_bytes: Option<u64>,
  /// The minimum byte length for this field's value to be considered valid.
  pub min_bytes: Option<u64>,
  /// The minimum bytte length for this field's value to be considered valid.
  pub max_bytes: Option<u64>,
  /// A regex pattern that this field's value should match in order to be considered valid.
  pub pattern: Option<Regex>,
  /// The prefix that this field's value should contain in order to be considered valid.
  pub prefix: Option<&'a str>,
  /// The suffix that this field's value should contain in order to be considered valid.
  pub suffix: Option<&'a str>,
  /// The substring that this field's value should contain in order to be considered valid.
  pub contains: Option<&'a str>,
  /// The substring that this field's value must not contain in order to be considered valid.
  pub not_contains: Option<&'a str>,
  /// Only the values in this list will be considered valid for this field.
  #[builder(into)]
  pub in_: Option<Box<[&'a str]>>,
  /// All the values in this list will be considered invalid for this field.
  #[builder(into)]
  pub not_in: Option<Box<[&'a str]>>,
  #[builder(setters(vis = "", name = well_known))]
  pub well_known: Option<WellKnown>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<&'a str>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Box<[CelRule]>>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl<'a, S: State> From<StringValidatorBuilder<'a, S>> for ProtoOption {
  #[track_caller]
  fn from(value: StringValidatorBuilder<S>) -> ProtoOption {
    value.build().into()
  }
}

impl<'a> From<StringValidator<'a>> for ProtoOption {
  #[track_caller]
  fn from(validator: StringValidator) -> ProtoOption {
    let name = "(buf.validate.field)";

    let mut values: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      values.push(("const".into(), OptionValue::String(const_val.into())));
    }

    validate_lists(validator.in_.as_deref(), validator.not_in.as_deref()).unwrap_or_else(
      |invalid| {
        panic!(
          "The following values are present inside of 'in' and 'not_in': {:?}",
          invalid
        )
      },
    );

    if validator.len.is_none() {
      insert_option!(validator, values, min_len, uint);
      insert_option!(validator, values, max_len, uint);
    } else {
      insert_option!(validator, values, len, uint);
    }

    if validator.len_bytes.is_none() {
      insert_option!(validator, values, min_bytes, uint);
      insert_option!(validator, values, max_bytes, uint);
    } else {
      insert_option!(validator, values, len_bytes, uint);
    }

    if let Some(pattern) = validator.pattern {
      values.push((
        "pattern".into(),
        OptionValue::String(pattern.as_str().into()),
      ))
    }

    insert_option!(validator, values, prefix, string);
    insert_option!(validator, values, suffix, string);
    insert_option!(validator, values, contains, string);
    insert_option!(validator, values, not_contains, string);
    insert_option!(validator, values, in_, [string]);
    insert_option!(validator, values, not_in, [string]);

    if let Some(v) = validator.well_known {
      v.to_option(&mut values)
    }

    let mut option_value: OptionValueList = vec![(
      "string".into(),
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
pub fn build_string_validator_option<F, S>(config_fn: F) -> ProtoOption
where
  F: FnOnce(StringValidatorBuilder) -> StringValidatorBuilder<S>,
  S: string_validator_builder::IsComplete,
{
  let builder = StringValidator::builder();
  let validator = config_fn(builder).build();
  validator.into()
}

/// All of the variants for protovalidate's well known string rules
#[derive(Clone, Debug, Copy)]
pub enum WellKnown {
  Email,
  Hostname,
  Ip,
  Ipv4,
  Ipv6,
  Uri,
  UriRef,
  Address,
  Uuid,
  Tuuid,
  IpWithPrefixlen,
  Ipv4WithPrefixlen,
  Ipv6WithPrefixlen,
  IpPrefix,
  Ipv4Prefix,
  Ipv6Prefix,
  HostAndPort,
  HeaderNameLoose,
  HeaderNameStrict,
  HeaderValueLoose,
  HeaderValueStrict,
}

use string_validator_builder::{IsUnset, SetWellKnown, State};

macro_rules! well_known_impl {
  ($name:ident) => {
    paste::paste! {
      #[doc = concat!("Sets a rule for this field to match the WellKnown::")]
      #[doc = concat!(stringify!($name))]
      #[doc = concat!(" variant.")]
      pub fn [< $name:snake >](self) -> StringValidatorBuilder<'a, SetWellKnown<S>>
        where
          S::WellKnown: IsUnset,
        {
          self.well_known(WellKnown::$name)
        }
    }
  };
}

impl_ignore!(StringValidatorBuilder);

impl<'a, S: State> StringValidatorBuilder<'a, S> {
  well_known_impl!(Email);
  well_known_impl!(Hostname);
  well_known_impl!(Ip);
  well_known_impl!(Ipv4);
  well_known_impl!(Ipv6);
  well_known_impl!(Uri);
  well_known_impl!(UriRef);
  well_known_impl!(Address);
  well_known_impl!(Uuid);
  well_known_impl!(Tuuid);
  well_known_impl!(IpWithPrefixlen);
  well_known_impl!(Ipv4WithPrefixlen);
  well_known_impl!(Ipv6WithPrefixlen);
  well_known_impl!(IpPrefix);
  well_known_impl!(Ipv4Prefix);
  well_known_impl!(Ipv6Prefix);
  well_known_impl!(HostAndPort);
  well_known_impl!(HeaderNameLoose);
  well_known_impl!(HeaderNameStrict);
  well_known_impl!(HeaderValueLoose);
  well_known_impl!(HeaderValueStrict);
}

impl WellKnown {
  pub(crate) fn to_option(self, option_values: &mut OptionValueList) {
    match self {
      WellKnown::Email => option_values.push(("email".into(), OptionValue::Bool(true))),
      WellKnown::Hostname => option_values.push(("hostname".into(), OptionValue::Bool(true))),
      WellKnown::Ip => option_values.push(("ip".into(), OptionValue::Bool(true))),
      WellKnown::Ipv4 => option_values.push(("ipv4".into(), OptionValue::Bool(true))),
      WellKnown::Ipv6 => option_values.push(("ipv6".into(), OptionValue::Bool(true))),
      WellKnown::Uri => option_values.push(("uri".into(), OptionValue::Bool(true))),
      WellKnown::UriRef => option_values.push(("uri_ref".into(), OptionValue::Bool(true))),
      WellKnown::Address => option_values.push(("address".into(), OptionValue::Bool(true))),
      WellKnown::Uuid => option_values.push(("uuid".into(), OptionValue::Bool(true))),
      WellKnown::Tuuid => option_values.push(("tuuid".into(), OptionValue::Bool(true))),
      WellKnown::IpWithPrefixlen => {
        option_values.push(("ip_with_prefixlen".into(), OptionValue::Bool(true)))
      }
      WellKnown::Ipv4WithPrefixlen => {
        option_values.push(("ipv4_with_prefixlen".into(), OptionValue::Bool(true)))
      }
      WellKnown::Ipv6WithPrefixlen => {
        option_values.push(("ipv6_with_prefixlen".into(), OptionValue::Bool(true)))
      }
      WellKnown::IpPrefix => option_values.push(("ip_prefix".into(), OptionValue::Bool(true))),
      WellKnown::Ipv4Prefix => option_values.push(("ipv4_prefix".into(), OptionValue::Bool(true))),
      WellKnown::Ipv6Prefix => option_values.push(("ipv6_prefix".into(), OptionValue::Bool(true))),
      WellKnown::HostAndPort => {
        option_values.push(("host_and_port".into(), OptionValue::Bool(true)))
      }
      WellKnown::HeaderNameLoose => {
        option_values.push((
          "well_known_regex".into(),
          OptionValue::Enum("KNOWN_REGEX_HTTP_HEADER_NAME".into()),
        ));
        option_values.push(("strict".into(), OptionValue::Bool(false)))
      }
      WellKnown::HeaderNameStrict => {
        option_values.push((
          "well_known_regex".into(),
          OptionValue::Enum("KNOWN_REGEX_HTTP_HEADER_NAME".into()),
        ));
        option_values.push(("strict".into(), OptionValue::Bool(true)))
      }
      WellKnown::HeaderValueLoose => {
        option_values.push((
          "well_known_regex".into(),
          OptionValue::Enum("KNOWN_REGEX_HTTP_HEADER_VALUE".into()),
        ));
        option_values.push(("strict".into(), OptionValue::Bool(false)))
      }
      WellKnown::HeaderValueStrict => {
        option_values.push((
          "well_known_regex".into(),
          OptionValue::Enum("KNOWN_REGEX_HTTP_HEADER_VALUE".into()),
        ));
        option_values.push(("strict".into(), OptionValue::Bool(true)))
      }
    };
  }
}
