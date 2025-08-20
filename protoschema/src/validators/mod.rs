use std::{collections::HashSet, fmt::Debug, hash::Hash};

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
pub mod macros {
  macro_rules! insert_option {
    (
      $validator:ident,
      $values:ident,
      $field:ident,
      $($val_type:tt)*
    ) => {
      $validator
        .$field
        .map(|v| $values.insert(super::get_option_name(stringify!($field)), option_value!(v, $($val_type)*)))
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
pub mod booleans;
pub mod bytes;
pub mod duration;
pub mod enums;
pub mod numeric;
pub mod string;
pub mod timestamp;
