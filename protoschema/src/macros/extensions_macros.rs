#[macro_export]
macro_rules! extension {
  ($file:ident, $extendee:literal { import $import:expr, $($tag:literal => $field:expr),* $(,)? }) => {
    $file.add_extension($crate::extensions::Extension {
      target: stringify!($extendee).into(),
      fields: vec![
        $($field.tag($tag).build())*
      ].into_boxed_slice(),
      import_path: $import.into()
    })
  };

  ($file:ident, $extendee:ident { $($tag:literal => $field:expr),* $(,)? }) => {
    $file.add_extension($crate::extensions::Extension {
      target: $extendee.get_full_name(),
      fields: vec![
        $($field.tag($tag).build())*
      ].into_boxed_slice(),
      import_path: $extendee.get_file()
    })
  };
}
