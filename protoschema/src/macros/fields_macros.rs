#[doc(hidden)]
#[macro_export]
macro_rules! parse_fields {
  (
    @included_fields($($included_fields:expr,)*)
    @fields($($fields:tt)*)
    @rest($(,)?)
  ) => {
    {
      let mut fields = vec! [ $($fields)* ];
      $(fields.extend($included_fields));*;
      fields
    }
  };

  (
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @rest($(,)? include($reusable_fields:expr) $($rest:tt)*)
  ) => {
    $crate::parse_fields!(
      @included_fields($($included_fields)* $reusable_fields,)
      @fields($($fields)*)
      @rest($($rest)*)
    )
  };

  (
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @rest($(,)? $tag:literal => $field:expr, $($rest:tt)* )
  ) => {
    $crate::parse_fields!(
      @included_fields($($included_fields)*)
      @fields($($fields)* $field.tag($tag),)
      @rest($($rest)*)
    )
  };

  (
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @rest($(,)? $tag:literal => $field:expr )
  ) => {
    $crate::parse_fields!(
      @included_fields($($included_fields)*)
      @fields($($fields)* $field.tag($tag))
      @rest()
    )
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_field_type {
  ($ty:ident) => {
    $crate::FieldType::from($ty.get_type())
  };

  ($ty:expr) => {
    $crate::FieldType::from($ty)
  };
}

#[macro_export]
macro_rules! reusable_fields {
  ($($tag:literal => $field:expr),+ $(,)?) => {
    [ $($field.tag($tag)),+ ]
  };
}

#[macro_export]
macro_rules! repeated_field {
  ($name:expr, $field_type:expr, $proto_type:ident $(, $validator:expr)?) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .repeated()
      .field_type($field_type)
      $(
        .add_option($crate::validators::repeated::[< build_repeated_ $proto_type _validator_option >]($validator))
        .add_import("buf/validate/validate.proto")
      )?
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! field {
  ($($optional:ident)? $name:expr, $field_type:expr, $proto_type:ident, $module_name:ident $(, $validator:expr)? ) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($field_type)
      $(.$optional())?
      $(
        .add_option($crate::validators::$module_name::[< build_ $proto_type _validator_option >]($validator))
        .add_import("buf/validate/validate.proto")
      )?
    }
  };
}

#[doc(hidden)]
macro_rules! field_impl {
  ($proto_type:ident, $module_name:ident $(, $import_path:expr)?) => {
    #[macro_export]
    macro_rules! $proto_type {
      (repeated $name:expr, $validator:expr) => {
        $crate::repeated_field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $validator
        )
        $(
          .add_import($import_path)
        )?
      };

      (repeated $name:expr) => {
        $crate::repeated_field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type
        )
        $(
          .add_import($import_path)
        )?
      };

      (optional $name:expr, $validator:expr) => {
        $crate::field!(
          optional
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name,
          $validator
        )
        $(
          .add_import($import_path)
        )?
      };

      (optional $name:expr) => {
        $crate::field!(
          optional
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name
        )
        $(
          .add_import($import_path)
        )?
      };

      ($name:expr, $validator:expr) => {
        $crate::field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name,
          $validator
        )
        $(
          .add_import($import_path)
        )?
      };

      ($name:expr) => {
        $crate::field!(
          $name,
          $crate::parse_field_type!(stringify!($proto_type)),
          $proto_type,
          $module_name
        )
        $(
          .add_import($import_path)
        )?
      };
    }
  };
}

field_impl!(string, string);
field_impl!(any, any, "google/protobuf/any.proto");
field_impl!(duration, duration, "google/protobuf/duration.proto");
field_impl!(timestamp, timestamp, "google/protobuf/timestamp.proto");
field_impl!(bytes, bytes);
field_impl!(bool, booleans);
field_impl!(int64, numeric);
field_impl!(int32, numeric);
field_impl!(sint64, numeric);
field_impl!(sint32, numeric);
field_impl!(sfixed64, numeric);
field_impl!(sfixed32, numeric);
field_impl!(uint64, numeric);
field_impl!(uint32, numeric);
field_impl!(fixed64, numeric);
field_impl!(fixed32, numeric);
field_impl!(double, numeric);
field_impl!(float, numeric);

#[macro_export]
macro_rules! enum_field {
  (repeated $enum_ident:expr, $name:expr $(, $validator:expr)?) => {
    $crate::repeated_field!(
      $name,
      $enum_ident.get_type(),
      enum
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };

  (optional $enum_ident:expr, $name:expr $(, $validator:expr)? ) => {
    $crate::field!(
      optional
      $name,
      $enum_ident.get_type(),
      enum
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };

  ($enum_ident:expr, $name:expr $(, $validator:expr)?) => {
    $crate::field!(
      $name,
      $enum_ident.get_type(),
      enum,
      enums
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };
}

#[macro_export]
macro_rules! msg_field {
  (repeated $msg_ident:expr, $name:expr $(, $validator:expr)?) => {
    $crate::repeated_field!(
      $name,
      $msg_ident.get_type(),
      message
      $(, $validator)?
    )
    .add_import(&$msg_ident.get_file())
  };

  ($msg_ident:expr, $name:expr $(, $validator:expr)?) => {
    $crate::field!(
      $name,
      $msg_ident.get_type(),
      message,
      message
      $(, $validator)?
    )
    .add_import(&$msg_ident.get_file())
  };
}
