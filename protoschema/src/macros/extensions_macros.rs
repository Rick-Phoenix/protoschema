#[macro_export]
macro_rules! extension {
  ($file:ident, $extendee:literal { $($tag:literal => $field:expr),* $(,)? }) => {
    $file.add_extension($crate::extensions::Extension {
      target: stringify!($extendee).into(),
      fields: vec![
        $($field.tag($tag).build())*
      ].into_boxed_slice(),
      imports: vec![]
    })
  };
}
