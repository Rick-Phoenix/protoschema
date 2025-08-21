use std::collections::BTreeMap;

use bon::Builder;

use crate::{validators::validate_lists, OptionValue, ProtoOption};

#[derive(Clone, Debug, Builder)]
#[builder(derive(Clone))]
pub struct StringValidator<'a> {
  pub len: Option<u64>,
  pub min_len: Option<u64>,
  pub max_len: Option<u64>,
  pub len_bytes: Option<u64>,
  pub min_bytes: Option<u64>,
  pub max_bytes: Option<u64>,
  pub pattern: Option<&'a str>,
  pub prefix: Option<&'a str>,
  pub suffix: Option<&'a str>,
  pub contains: Option<&'a str>,
  pub not_contains: Option<&'a str>,
  pub in_: Option<&'a [&'a str]>,
  pub not_in: Option<&'a [&'a str]>,
  #[builder(setters(vis = "", name = well_known))]
  pub well_known: Option<WellKnown>,
  pub const_: Option<&'a str>,
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
    let name = "(buf.validate.field).string";

    let mut values: BTreeMap<Box<str>, OptionValue> = BTreeMap::new();

    if let Some(const_val) = validator.const_ {
      values.insert("const".into(), OptionValue::String(const_val.into()));

      return ProtoOption {
        name,
        value: OptionValue::Message(values),
      };
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

    if validator.len_bytes.is_none() {
      insert_option!(validator, values, min_bytes, uint);
      insert_option!(validator, values, max_bytes, uint);
    } else {
      insert_option!(validator, values, len_bytes, uint);
    }

    insert_option!(validator, values, pattern, string);
    insert_option!(validator, values, prefix, string);
    insert_option!(validator, values, suffix, string);
    insert_option!(validator, values, contains, string);
    insert_option!(validator, values, not_contains, string);
    insert_option!(validator, values, in_, [string]);
    insert_option!(validator, values, not_in, [string]);

    if let Some(v) = validator.well_known {
      v.to_option(&mut values)
    }

    ProtoOption {
      name,
      value: OptionValue::Message(values),
    }
  }
}

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
      pub fn [< $name:snake >](self) -> StringValidatorBuilder<'a, SetWellKnown<S>>
        where
          S::WellKnown: IsUnset,
        {
          self.well_known(WellKnown::$name)
        }
    }
  };
}

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
  pub(crate) fn to_option(self, option_values: &mut BTreeMap<Box<str>, OptionValue>) {
    match self {
      WellKnown::Email => option_values.insert("email".into(), OptionValue::Bool(true)),
      WellKnown::Hostname => option_values.insert("hostname".into(), OptionValue::Bool(true)),
      WellKnown::Ip => option_values.insert("ip".into(), OptionValue::Bool(true)),
      WellKnown::Ipv4 => option_values.insert("ipv4".into(), OptionValue::Bool(true)),
      WellKnown::Ipv6 => option_values.insert("ipv6".into(), OptionValue::Bool(true)),
      WellKnown::Uri => option_values.insert("uri".into(), OptionValue::Bool(true)),
      WellKnown::UriRef => option_values.insert("uri_ref".into(), OptionValue::Bool(true)),
      WellKnown::Address => option_values.insert("address".into(), OptionValue::Bool(true)),
      WellKnown::Uuid => option_values.insert("uuid".into(), OptionValue::Bool(true)),
      WellKnown::Tuuid => option_values.insert("tuuid".into(), OptionValue::Bool(true)),
      WellKnown::IpWithPrefixlen => {
        option_values.insert("ip_with_prefixlen".into(), OptionValue::Bool(true))
      }
      WellKnown::Ipv4WithPrefixlen => {
        option_values.insert("ipv4_with_prefixlen".into(), OptionValue::Bool(true))
      }
      WellKnown::Ipv6WithPrefixlen => {
        option_values.insert("ipv6_with_prefixlen".into(), OptionValue::Bool(true))
      }
      WellKnown::IpPrefix => option_values.insert("ip_prefix".into(), OptionValue::Bool(true)),
      WellKnown::Ipv4Prefix => option_values.insert("ipv4_prefix".into(), OptionValue::Bool(true)),
      WellKnown::Ipv6Prefix => option_values.insert("ipv6_prefix".into(), OptionValue::Bool(true)),
      WellKnown::HostAndPort => {
        option_values.insert("host_and_port".into(), OptionValue::Bool(true))
      }
      WellKnown::HeaderNameLoose => {
        option_values.insert(
          "well_known_regex".into(),
          OptionValue::Identifier("KNOWN_REGEX_HTTP_HEADER_NAME".into()),
        );
        option_values.insert("strict".into(), OptionValue::Bool(false))
      }
      WellKnown::HeaderNameStrict => {
        option_values.insert(
          "well_known_regex".into(),
          OptionValue::Identifier("KNOWN_REGEX_HTTP_HEADER_NAME".into()),
        );
        option_values.insert("strict".into(), OptionValue::Bool(true))
      }
      WellKnown::HeaderValueLoose => {
        option_values.insert(
          "well_known_regex".into(),
          OptionValue::Identifier("KNOWN_REGEX_HTTP_HEADER_VALUE".into()),
        );
        option_values.insert("strict".into(), OptionValue::Bool(false))
      }
      WellKnown::HeaderValueStrict => {
        option_values.insert(
          "well_known_regex".into(),
          OptionValue::Identifier("KNOWN_REGEX_HTTP_HEADER_VALUE".into()),
        );
        option_values.insert("strict".into(), OptionValue::Bool(true))
      }
    };
  }
}
