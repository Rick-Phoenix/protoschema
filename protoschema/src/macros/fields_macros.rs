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
      @included_fields($($included_fields)* $reusable_fields.clone(),)
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
      @fields($($fields)* ($tag, $field),)
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
      @fields($($fields)* ($tag, $field))
      @rest()
    )
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_field_type {
  ($ty:ident) => {
    $crate::paste! {
      $crate::FieldType::[< $ty:camel >]
    }
  };
}

/// Defines some fields that can be included as a group in multiple messages.
///
/// # Examples
/// ```
/// use protoschema::{reusable_fields, uint64, timestamp, message, Package};
///
/// let my_pkg = Package::new("my_pkg");
/// let my_file = my_pkg.new_file("my_file");
/// let my_msg1 = my_file.new_message("my_msg1");
/// let my_msg2 = my_file.new_message("my_msg2");
///
/// let my_common_fields = reusable_fields!(
///   1 => uint64!("id"),
///   2 => timestamp!("created_at"),
///   3 => timestamp!("updated_at")
/// );
///
/// message!(my_msg1,
///   include(my_common_fields),
///   4 => uint64!("other_field")
/// );
/// message!(my_msg2,
///   include(my_common_fields),
///   4 => uint64!("some_other_field")
/// );
/// ```
#[macro_export]
macro_rules! reusable_fields {
  ($($tag:literal => $field:expr),+ $(,)?) => {
    [ $(($tag, $field)),+ ]
  };
}

#[doc(hidden)]
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
        .add_import($crate::common::VALIDATE_PROTO_FILE.clone())
      )?
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! optional_field {
  ($name:expr, $field_type:expr, $proto_type:ident, $module_name:ident $(, $validator:expr)? ) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($field_type)
      .optional()
      $(
        .add_option($crate::validators::$module_name::[< build_ $proto_type _validator_option >]($validator))
        .add_import($crate::common::VALIDATE_PROTO_FILE.clone())
      )?
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! field {
  ($name:expr, $field_type:expr, $proto_type:ident, $module_name:ident $(, $validator:expr)? ) => {
    $crate::paste! {
      $crate::fields::Field::builder()
      .name($name.into())
      .field_type($field_type)
      $(
        .add_option($crate::validators::$module_name::[< build_ $proto_type _validator_option >]($validator))
        .add_import($crate::common::VALIDATE_PROTO_FILE.clone())
      )?
    }
  };
}

#[doc(hidden)]
macro_rules! field_impl {
  ($proto_type:ident, $module_name:ident $(, $import_path:expr)?) => {
    #[doc = concat!("Evaluates to a protobuf ")]
    #[doc = concat!(stringify!($proto_type))]
    #[doc = concat!("[`FieldBuilder`](crate::fields::FieldBuilder) instance.")]
    #[doc = concat!("")]
    #[doc = concat!("The first argument is the name of the field, which can be a literal or an expression, optionally preceded by 'optional' or 'repeated'.")]
    #[doc = concat!("")]
    #[doc = concat!("The second, optional argument is a closure where validation rules can be defined.")]
    #[doc = concat!("")]
    #[doc = concat!("If the field is marked as repeated, the closure will receive two arguments, one being the [`RepeatedValidator`](crate::validators::repeated::RepeatedValidator) builder, and the other being the validator builder for the field. Otherwise, the only argument will be the latter.")]
    #[macro_export]
    macro_rules! $proto_type {
      (repeated $name:expr, $validator:expr) => {
        $crate::repeated_field!(
          $name,
          $crate::parse_field_type!($proto_type),
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
          $crate::parse_field_type!($proto_type),
          $proto_type
        )
        $(
          .add_import($import_path)
        )?
      };

      (optional $name:expr, $validator:expr) => {
        $crate::optional_field!(
          $name,
          $crate::parse_field_type!($proto_type),
          $proto_type,
          $module_name,
          $validator
        )
        $(
          .add_import($import_path)
        )?
      };

      (optional $name:expr) => {
        $crate::optional_field!(
          $name,
          $crate::parse_field_type!($proto_type),
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
          $crate::parse_field_type!($proto_type),
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
          $crate::parse_field_type!($proto_type),
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
field_impl!(bool, bool);
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

/// Evaluates to an enum [`FieldBuilder`](crate::fields::FieldBuilder) instance.
///
/// The first argument is an expression or ident evaluating to an [`EnumBuilder`](crate::enums::EnumBuilder) instance, optionally preceded by 'optional' or 'repeated'.
/// The second argument is the name of the field, which can be a literal or an expression.
/// The third, optional argument is a closure where validation rules can be defined.
/// If the field is marked as repeated, the closure will receive two arguments, one being the [`RepeatedValidator`](crate::validators::repeated::RepeatedValidator) builder, and the other being the [`EnumValidator`](crate::validators::enums::EnumValidator) builder. Otherwise, the only argument will be the latter.
/// # Examples
/// ```
/// use protoschema::{Package, enum_field};
///
/// let pkg = Package::new("my_pkg");
/// let file = pkg.new_file("my_file");
/// let my_enum = file.new_enum("my_enum");
/// let my_field1 = enum_field!(
///   repeated my_enum, "my_field1"
/// );
/// ```
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
    $crate::optional_field!(
      $name,
      $enum_ident.get_type(),
      enum,
      enums
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

/// Evaluates to a message [`FieldBuilder`](crate::fields::FieldBuilder) instance.
///
/// The first argument is an expression or ident evaluating to a [`MessageBuilder`](crate::messages::MessageBuilder) instance, optionally preceded by 'optional' or 'repeated'.
/// The second argument is the name of the field, which can be a literal or an expression.
/// The third, optional argument is a closure where validation rules can be defined.
/// If the field is marked as repeated, the closure will receive two arguments, one being the [`RepeatedValidator`](crate::validators::repeated::RepeatedValidator) builder, and the other being the [`MessageValidator`](crate::validators::message::MessageValidator) builder. Otherwise, the only argument will be the latter.
/// /// # Examples
/// ```
/// use protoschema::{Package, msg_field};
///
/// let pkg = Package::new("my_pkg");
/// let file = pkg.new_file("my_file");
/// let my_msg = file.new_message("my_msg");
/// let my_field1 = msg_field!(
///   repeated my_msg, "my_field1"
/// );
/// ```
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

macro_rules! impl_well_known_type {
  ($name:ident, $full_name:literal, $import_path:literal) => {
    $crate::paste! {
      #[doc = "Expands to a [`FieldBuilder`](crate::fields::FieldBuilder) instance for a " $full_name " field."]
      #[macro_export]
      macro_rules! $name {
        ($field_name:expr, $validator:expr) => {
          $crate::fields::Field::builder()
            .name($field_name.into())
            .field_type($crate::FieldType::[< $name:camel >])
            .add_import($import_path)
            .add_option($crate::validators::message::build_message_validator_option)
            .add_import($crate::common::VALIDATE_PROTO_FILE.clone())
        };

        ($field_name:expr) => {
          $crate::fields::Field::builder()
            .name($field_name.into())
            .field_type($crate::FieldType::[< $name:camel >])
            .add_import($import_path)
        };
      }
    }
  };
}

impl_well_known_type!(
  field_mask,
  "google.protobuf.FieldMask",
  "google/protobuf/field_mask.proto"
);
impl_well_known_type!(
  empty,
  "google.protobuf.Empty",
  "google/protobuf/empty.proto"
);
impl_well_known_type!(
  proto_struct,
  "google.protobuf.Struct",
  "google/protobuf/descriptor.proto"
);
impl_well_known_type!(money, "google.type.Money", "google/type/money.proto");
impl_well_known_type!(
  interval,
  "google.type.Interval",
  "google/type/interval.proto"
);
impl_well_known_type!(color, "google.type.Color", "google/type/color.proto");
impl_well_known_type!(date, "google.type.Date", "google/type/date.proto");
impl_well_known_type!(
  datetime,
  "google.type.DateTime",
  "google/type/datetime.proto"
);
impl_well_known_type!(
  day_of_week,
  "google.type.DayOfWeek",
  "google/type/dayofweek.proto"
);
impl_well_known_type!(decimal, "google.type.Decimal", "google/type/decimal.proto");
impl_well_known_type!(expr, "google.type.Expr", "google/type/expr.proto");
impl_well_known_type!(
  fraction,
  "google.type.Fraction",
  "google/type/fraction.proto"
);
impl_well_known_type!(lat_lng, "google.type.LatLng", "google/type/latlng.proto");
impl_well_known_type!(
  localized_text,
  "google.type.LocalizedText",
  "google/type/localized_text.proto"
);
impl_well_known_type!(month, "google.type.Month", "google/type/month.proto");
impl_well_known_type!(
  phone_number,
  "google.type.PhoneNumber",
  "google/type/phone_number.proto"
);
impl_well_known_type!(
  postal_address,
  "google.type.PostalAddress",
  "google/type/postal_address.proto"
);
impl_well_known_type!(
  quaternion,
  "google.type.Quaternion",
  "google/type/quaternion.proto"
);
impl_well_known_type!(
  time_of_day,
  "google.type.TimeOfDay",
  "google/type/timeofday.proto"
);
impl_well_known_type!(
  http_request,
  "google.rpc.HttpRequest",
  "google/rpc/http.proto"
);
impl_well_known_type!(
  http_response,
  "google.rpc.HttpResponse",
  "google/rpc/http.proto"
);
impl_well_known_type!(
  http_header,
  "google.rpc.HttpHeader",
  "google/rpc/http.proto"
);
impl_well_known_type!(status, "google.rpc.Status", "google/rpc/status.proto");
impl_well_known_type!(code, "google.rpc.Code", "google/rpc/code.proto");
impl_well_known_type!(
  error_info,
  "google.rpc.ErrorInfo",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  retry_info,
  "google.rpc.RetryInfo",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  debug_info,
  "google.rpc.DebugInfo",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  quota_failure,
  "google.rpc.QuotaFailure",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  quota_failure_violation,
  "google.rpc.QuotaFailure.Violation",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  precondition_failure,
  "google.rpc.PreconditionFailure",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  precondition_failure_violation,
  "google.rpc.PreconditionFailure.Violation",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  bad_request,
  "google.rpc.BadRequest",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  bad_request_violation,
  "google.rpc.BadRequest.Violation",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  request_info,
  "google.rpc.RequestInfo",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(
  resource_info,
  "google.rpc.ResourceInfo",
  "google/rpc/error_details.proto"
);
impl_well_known_type!(help, "google.rpc.Help", "google/rpc/error_details.proto");
impl_well_known_type!(
  localized_message,
  "google.rpc.LocalizedMessage",
  "google/rpc/error_details.proto"
);
