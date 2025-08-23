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
      $crate::FieldType::Enum($enum_ident.get_full_name().into()),
      enum
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };

  (optional $enum_ident:expr, $name:expr $(, $validator:expr)? ) => {
    $crate::field!(
      optional
      $name,
      $crate::FieldType::Enum($enum_ident.get_full_name().into()),
      enum
      $(, $validator)?
    )
    .add_import(&$enum_ident.get_file())
  };

  ($enum_ident:expr, $name:expr $(, $validator:expr)?) => {
    $crate::field!(
      $name,
      $crate::FieldType::Enum($enum_ident.get_full_name().into()),
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
      $crate::FieldType::Message($msg_ident.get_full_name().into()),
      message
      $(, $validator)?
    )
    .add_import(&$msg_ident.get_file())
  };

  ($msg_ident:expr, $name:expr $(, $validator:expr)?) => {
    $crate::field!(
      $name,
      $crate::FieldType::Message($msg_ident.get_full_name().into()),
      message,
      message
      $(, $validator)?
    )
    .add_import(&$msg_ident.get_file())
  };
}
