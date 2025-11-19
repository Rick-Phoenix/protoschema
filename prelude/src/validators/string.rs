use bon::Builder;
use regex::Regex;
use string_validator_builder::{IsUnset, SetIn, SetNotIn, SetWellKnown, State};

use super::*;
use crate::{
  validators::{cel::CelRule, validate_lists, OptionValueList},
  *,
};

impl<S: State> StringValidatorBuilder<S>
where
  S::In: IsUnset,
{
  pub fn in_<T: Into<Arc<str>>, I: IntoIterator<Item = T>>(
    self,
    list: I,
  ) -> StringValidatorBuilder<SetIn<S>> {
    let list = create_string_list(list);
    self.in_internal(list)
  }
}

impl<S: State> StringValidatorBuilder<S>
where
  S::NotIn: IsUnset,
{
  pub fn not_in<T: Into<Arc<str>>, I: IntoIterator<Item = T>>(
    self,
    list: I,
  ) -> StringValidatorBuilder<SetNotIn<S>> {
    let list = create_string_list(list);
    self.not_in_internal(list)
  }
}

#[derive(Clone, Debug, Builder)]
#[builder(derive(Clone))]
#[builder(on(Arc<str>, into))]
pub struct StringValidator {
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
  pub prefix: Option<Arc<str>>,
  /// The suffix that this field's value should contain in order to be considered valid.
  pub suffix: Option<Arc<str>>,
  /// The substring that this field's value should contain in order to be considered valid.
  pub contains: Option<Arc<str>>,
  /// The substring that this field's value must not contain in order to be considered valid.
  pub not_contains: Option<Arc<str>>,
  /// Only the values in this list will be considered valid for this field.
  #[builder(setters(vis = "", name = in_internal))]
  pub in_: Option<Arc<[Arc<str>]>>,
  /// All the values in this list will be considered invalid for this field.
  #[builder(setters(vis = "", name = not_in_internal))]
  pub not_in: Option<Arc<[Arc<str>]>>,
  #[builder(setters(vis = "", name = well_known))]
  pub well_known: Option<WellKnownStrings>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<Arc<str>>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl_into_option!(StringValidator);
impl_validator!(StringValidator, String);

impl From<StringValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: StringValidator) -> ProtoOption {
    let mut rules: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      rules.push((CONST_.clone(), OptionValue::String(const_val)));
    }

    validate_lists(validator.in_.as_deref(), validator.not_in.as_deref()).unwrap();

    if validator.len.is_none() {
      insert_option!(validator, rules, min_len);
      insert_option!(validator, rules, max_len);
    } else {
      insert_option!(validator, rules, len);
    }

    if validator.len_bytes.is_none() {
      insert_option!(validator, rules, min_bytes);
      insert_option!(validator, rules, max_bytes);
    } else {
      insert_option!(validator, rules, len_bytes);
    }

    if let Some(pattern) = validator.pattern {
      rules.push((
        PATTERN.clone(),
        OptionValue::String(pattern.as_str().into()),
      ))
    }

    insert_option!(validator, rules, prefix);
    insert_option!(validator, rules, suffix);
    insert_option!(validator, rules, contains);
    insert_option!(validator, rules, not_contains);
    insert_option!(validator, rules, in_);
    insert_option!(validator, rules, not_in);

    if let Some(v) = validator.well_known {
      v.to_option(&mut rules)
    }

    // This is the outer rule grouping, "(buf.validate.field)"
    let mut outer_rules: OptionValueList = vec![];

    outer_rules.push((STRING.clone(), OptionValue::Message(rules.into())));

    // These must be added on the outer grouping, as they are generic rules
    // It's (buf.validate.field).required, NOT (buf.validate.field).string.required
    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, required);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}

reusable_string!(STRING);
reusable_string!(LEN_BYTES);
reusable_string!(MIN_BYTES);
reusable_string!(MAX_BYTES);

/// All of the variants for protovalidate's well known string rules
#[derive(Clone, Debug, Copy)]
pub enum WellKnownStrings {
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

macro_rules! well_known_impl {
  ($name:ident) => {
    paste::paste! {
      #[doc = "Sets a rule for this field to match the [`WellKnown::" $name "`] variant."]
      pub fn [< $name:snake >](self) -> StringValidatorBuilder<SetWellKnown<S>>
        where
          S::WellKnown: IsUnset,
        {
          self.well_known(WellKnownStrings::$name)
        }
    }
  };
}

impl_ignore!(StringValidatorBuilder);

impl<S: State> StringValidatorBuilder<S> {
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

reusable_string!(EMAIL);
reusable_string!(HOSTNAME);
reusable_string!(URI);
reusable_string!(URI_REF);
reusable_string!(ADDRESS);
reusable_string!(UUID);
reusable_string!(TUUID);
reusable_string!(IP_WITH_PREFIXLEN);
reusable_string!(IPV4_WITH_PREFIXLEN);
reusable_string!(IPV6_WITH_PREFIXLEN);
reusable_string!(IP_PREFIX);
reusable_string!(IPV4_PREFIX);
reusable_string!(IPV6_PREFIX);
reusable_string!(HOST_AND_PORT);
reusable_string!(WELL_KNOWN_REGEX);
reusable_string!(KNOWN_REGEX_HTTP_HEADER_NAME, "KNOWN_REGEX_HTTP_HEADER_NAME");
reusable_string!(
  KNOWN_REGEX_HTTP_HEADER_VALUE,
  "KNOWN_REGEX_HTTP_HEADER_VALUE"
);
reusable_string!(STRICT);

impl WellKnownStrings {
  pub(crate) fn to_option(self, option_values: &mut OptionValueList) {
    let name = match self {
      WellKnownStrings::Email => EMAIL.clone(),
      WellKnownStrings::Hostname => HOSTNAME.clone(),
      WellKnownStrings::Ip => IP.clone(),
      WellKnownStrings::Ipv4 => IPV4.clone(),
      WellKnownStrings::Ipv6 => IPV6.clone(),
      WellKnownStrings::Uri => URI.clone(),
      WellKnownStrings::UriRef => URI_REF.clone(),
      WellKnownStrings::Address => ADDRESS.clone(),
      WellKnownStrings::Uuid => UUID.clone(),
      WellKnownStrings::Tuuid => TUUID.clone(),
      WellKnownStrings::IpWithPrefixlen => IP_WITH_PREFIXLEN.clone(),
      WellKnownStrings::Ipv4WithPrefixlen => IPV4_WITH_PREFIXLEN.clone(),
      WellKnownStrings::Ipv6WithPrefixlen => IPV6_WITH_PREFIXLEN.clone(),
      WellKnownStrings::IpPrefix => IP_PREFIX.clone(),
      WellKnownStrings::Ipv4Prefix => IPV4_PREFIX.clone(),
      WellKnownStrings::Ipv6Prefix => IPV6_PREFIX.clone(),
      WellKnownStrings::HostAndPort => HOST_AND_PORT.clone(),
      WellKnownStrings::HeaderNameLoose
      | WellKnownStrings::HeaderNameStrict
      | WellKnownStrings::HeaderValueLoose
      | WellKnownStrings::HeaderValueStrict => WELL_KNOWN_REGEX.clone(),
    };

    let value = match self {
      WellKnownStrings::HeaderNameLoose => {
        option_values.push((STRICT.clone(), OptionValue::Bool(false)));
        OptionValue::Enum(KNOWN_REGEX_HTTP_HEADER_NAME.clone())
      }
      WellKnownStrings::HeaderNameStrict => {
        option_values.push((STRICT.clone(), OptionValue::Bool(true)));
        OptionValue::Enum(KNOWN_REGEX_HTTP_HEADER_NAME.clone())
      }
      WellKnownStrings::HeaderValueLoose => {
        option_values.push((STRICT.clone(), OptionValue::Bool(false)));
        OptionValue::Enum(KNOWN_REGEX_HTTP_HEADER_VALUE.clone())
      }
      WellKnownStrings::HeaderValueStrict => {
        option_values.push((STRICT.clone(), OptionValue::Bool(true)));
        OptionValue::Enum(KNOWN_REGEX_HTTP_HEADER_VALUE.clone())
      }
      _ => OptionValue::Bool(true),
    };

    option_values.push((name, value));
  }
}
