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
        .map(|v| $values.insert(stringify!($field).into(), option_value!(v, $($val_type)*)))
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
    ($val:ident, [i32]) => {
      OptionValue::List(
        $val
          .iter()
          .map(|i| OptionValue::Int(*i as i64))
          .collect::<Vec<OptionValue>>()
          .into_boxed_slice(),
      )
    };
    ($val:ident, [$val_type:ident]) => {
      paste::paste! {
        OptionValue::List(
          $val
            .iter()
            .map(|i| OptionValue::[< $val_type:camel >](*i))
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
        OptionValue::[< $val_type:camel >]($val)
      }
    };
  }
}

pub mod any;
pub mod booleans;
pub mod bytes;
pub mod duration;
pub mod enums;
pub mod strings;
pub mod timestamp;
