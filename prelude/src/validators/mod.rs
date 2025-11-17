mod common_strings;
use std::{collections::HashSet, fmt::Debug, hash::Hash, sync::Arc};

use common_strings::*;
use proto_types::protovalidate::Ignore;

type OptionValueList = Vec<(Arc<str>, OptionValue)>;

macro_rules! impl_ignore {
  (no_lifetime, $builder:ident) => {
    $crate::paste! {
      impl < S: [< $builder:snake >]::State> $builder< S>
      where
        S::Ignore: [< $builder:snake >]::IsUnset,
      {
        /// Rules defined for this field will be ignored if the field is set to its protobuf zero value.
        /// No-op for fields that track presence such as optional fields, or messages in proto3.
        pub fn ignore_if_zero_value(self) -> $builder< [< $builder:snake >]::SetIgnore<S>> {
          self.ignore(Ignore::IfZeroValue)
        }

        /// Rules set for this field will always be ignored.
        pub fn ignore_always(self) -> $builder< [< $builder:snake >]::SetIgnore<S>> {
          self.ignore(Ignore::Always)
        }
      }
    }
  };

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

#[macro_use]
mod macros {
  macro_rules! insert_cel_rule {
    ($validator:ident, $values:ident) => {
      if let Some(cel_rules) = $validator.cel {
        let rule_values: Vec<OptionValue> =
          cel_rules.iter().cloned().map(OptionValue::from).collect();
        $values.push((CEL.clone(), OptionValue::List(rule_values.into())));
      }
    };
  }

  macro_rules! insert_option2 {
    (
    $validator:ident,
    $values:ident,
    $field:ident
  ) => {
      $crate::paste! {
        if let Some(value) = $validator.$field {
          $values.push(([< $field:snake:upper >].clone(), value.into()))
        }
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
      $crate::paste! {
        if let Some(value) = $validator.$field {
          $values.push(([< $field:snake:upper >].clone(), option_value!(value, $($val_type)*)))
        }
      }
    };
  }

  macro_rules! option_value {
    ($val:ident, [string]) => {
      OptionValue::List(
        $val
          .iter()
          .map(|&i| OptionValue::String(i.into()))
          .collect::<Vec<OptionValue>>()
          .into(),
      )
    };
    ($val:ident, [$val_type:ident]) => {
      OptionValue::List(
        $val
          .iter()
          .map(|i| OptionValue::from(*i))
          .collect::<Vec<OptionValue>>()
          .into(),
      )
    };
    ($val:ident, string) => {
      OptionValue::String($val.into())
    };
    ($val:ident, $val_type:ident) => {
      OptionValue::from($val)
    };
  }
}

mod any;
mod bool;
mod bytes;
mod cel;
mod map;
mod numeric;
mod string;

pub use any::*;
pub use bool::*;
pub use bytes::*;
pub use cel::*;
pub use map::*;
pub use numeric::*;
pub use string::*;

use crate::OptionValue;
