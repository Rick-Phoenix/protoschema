#[macro_export]
macro_rules! extension {
  ($file:ident, $extendee:literal { import $import:expr, $($fields:tt)* }) => {
    $file.add_extension($crate::extensions::Extension::builder()
      .target(stringify!($extendee).into())
      .import_path($import.into())
      .fields(
        $crate::parse_fields!(
          @included_fields()
          @fields()
          @rest($($fields)*)
        )
      )
      .build()
      )
  };

  ($file:ident, $extendee:ident { $($fields:tt)* }) => {
    $file.add_extension($crate::extensions::Extension::builder()
      .target($extendee.get_full_name())
      .import_path($extendee.get_file())
      .fields(
        $crate::parse_fields!(
          @included_fields()
          @fields()
          @rest($($fields)*)
        )
      )
      .build()
      )
  };
}
