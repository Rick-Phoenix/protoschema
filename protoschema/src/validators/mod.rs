use std::{collections::HashSet, fmt::Debug, hash::Hash};

use proto_types::protovalidate::Ignore;

use crate::OptionValue;

type OptionValueList = Vec<(Box<str>, OptionValue)>;

macro_rules! impl_ignore {
  ($builder:ident) => {
    $crate::paste! {
      impl <'a, S: [< $builder:snake >]::State> $builder<'a, S>
      where
        S::Ignore: [< $builder:snake >]::IsUnset,
      {
        /// Rules defined for this field will be ignored if the field is set to its protobuf zero value.
        /// No-op for fields that track presence such as optional fields, or messages in proto3.
        pub fn ignore_if_zero_value(self) -> $builder<'a, [< $builder:snake >]::SetIgnore<S>> {
          self.ignore(Ignore::IfZeroValue)
        }

        /// Rules set for this field will always be ignored.
        pub fn ignore_always(self) -> $builder<'a, [< $builder:snake >]::SetIgnore<S>> {
          self.ignore(Ignore::Always)
        }
      }
    }
  };
}

#[track_caller]
fn validate_comparables<T>(lt: Option<T>, lte: Option<T>, gt: Option<T>, gte: Option<T>)
where
  T: Copy + PartialEq + PartialOrd,
{
  if lt.is_some() && lte.is_some() {
    panic!("Cannot use lt and lte together")
  }

  if gt.is_some() && gte.is_some() {
    panic!("Cannot use gt and gte together")
  }

  if let Some(lt_val) = lt {
    if let Some(gt_val) = gt && lt_val <= gt_val {
      panic!("Lt cannot be smaller than or equal to gt")
    }

    if let Some(gte_val) = gte && lt_val < gte_val {
      panic!("Lt cannot be smaller than gte")
    }
  }

  if let Some(lte_val) = lte {
    if let Some(gt_val) = gt && lte_val < gt_val {
      panic!("Lte cannot be smaller than to gt")
    }

    if let Some(gte_val) = gte && lte_val < gte_val {
      panic!("Lte cannot be smaller than gte")
    }
  }
}

#[track_caller]
fn validate_lists<'a, T>(
  in_list: Option<&'a [T]>,
  not_in_list: Option<&'a [T]>,
) -> Result<(), Vec<T>>
where
  T: Eq + Hash + Debug + Clone,
{
  let in_list = in_list.unwrap_or(&[]);
  let not_in_list = not_in_list.unwrap_or(&[]);

  if in_list.is_empty() || not_in_list.is_empty() {
    return Ok(());
  }

  let in_list_set: HashSet<&T> = in_list.iter().collect();
  let not_in_list_set: HashSet<&T> = not_in_list.iter().collect();

  let mut invalid_vals: Vec<T> = Vec::new();

  in_list_set
    .intersection(&not_in_list_set)
    .for_each(|&v| invalid_vals.push(v.to_owned()));

  if !invalid_vals.is_empty() {
    Err(invalid_vals)
  } else {
    Ok(())
  }
}

fn get_option_name(raw_name: &str) -> Box<str> {
  if raw_name == "const_" {
    "const".into()
  } else if raw_name == "in_" {
    "in".into()
  } else {
    raw_name.into()
  }
}

#[macro_use]
mod macros {
  macro_rules! insert_cel_rule {
    ($validator:ident, $values:ident) => {
      if let Some(cel_rules) = $validator.cel {
        let rule_values: Vec<OptionValue> =
          cel_rules.iter().cloned().map(OptionValue::from).collect();
        $values.push((
          "cel".into(),
          OptionValue::List(rule_values.into_boxed_slice()),
        ));
      }
    };
  }

  macro_rules! insert_option {
    (
      $validator:ident,
      $values:ident,
      $field:ident,
      $($val_type:tt)*
    ) => {
      $validator
        .$field
        .map(|v| $values.push((super::get_option_name(stringify!($field)), option_value!(v, $($val_type)*))))
    };
  }

  macro_rules! option_value {
    ($val:ident, [string]) => {
      OptionValue::List(
        $val
          .iter()
          .map(|&i| OptionValue::String(i.into()))
          .collect::<Vec<OptionValue>>()
          .into_boxed_slice(),
      )
    };
    ($val:ident, [$val_type:ident]) => {
      paste::paste! {
        OptionValue::List(
          $val
            .iter()
            .map(|i| OptionValue::from(*i))
            .collect::<Vec<OptionValue>>()
            .into_boxed_slice()
        )
      }
    };
    ($val:ident, string) => {
      OptionValue::String($val.into())
    };
    ($val:ident, $val_type:ident) => {
      paste::paste! {
        OptionValue::from($val)
      }
    };
  }
}

pub mod any;
pub mod bool;
pub mod bytes;
pub mod cel;
pub mod duration;
pub mod enums;
pub mod map;
pub mod message;
pub mod numeric;
pub mod repeated;
pub mod string;
pub mod timestamp;
