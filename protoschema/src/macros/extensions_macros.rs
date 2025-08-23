#[macro_export]
macro_rules! extension {
  ($file:ident, $extendee:ident { $($fields:tt)* }) => {
    $file.add_extension($crate::extensions::Extension::builder()
      .import_path($extendee.get_import_path())
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
