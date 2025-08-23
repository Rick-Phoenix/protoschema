#[macro_export]
macro_rules! oneof {
  (
    $name:expr,
    options = $options_expr:expr,
    $($fields:tt)*
  ) => {
    {
      $crate::oneofs::Oneof::builder()
        .name($name.into())
        .options($options_expr)
        .fields(
          $crate::parse_fields!(
          @included_fields()
          @fields()
          @rest($($fields)*)
          )
        )
        .build()
    }
  };

  (
    $name:expr,
    $($fields:tt)*
  ) => {
    {
      $crate::oneofs::Oneof::builder()
        .name($name.into())
        .fields(
          $crate::parse_fields!(
          @included_fields()
          @fields()
          @rest($($fields)*)
          )
        )
        .build()
    }
  };
}
