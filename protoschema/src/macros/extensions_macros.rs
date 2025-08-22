#[macro_export]
macro_rules! extension {
  ($file:ident, $extendee:literal { import $import:expr, $($tag:literal => $field:expr),* $(,)? }) => {
    $file.add_extension($crate::extensions::Extension::builder()
      .target(stringify!($extendee).into())
      .import_path($import.into())
      .fields(
        [ $($field.tag($tag).build())* ]
      )
      .build()
      )
  };

  ($file:ident, $extendee:ident { $($tag:literal => $field:expr),* $(,)? }) => {
    $file.add_extension($crate::extensions::Extension::builder()
      .target($extendee.get_full_name())
      .import_path($extendee.get_file())
      .fields(
        [ $($field.tag($tag).build())* ]
      )
      .build()
      )
  };
}
