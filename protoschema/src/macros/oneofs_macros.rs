/// Evaluates to a [`Oneof`](crate::oneofs::Oneof) instance.
/// The first argument is the name of the oneof, followed by a comma, optionally followed by this oneof's options defined as `options = $options` where $options should evaluate to IntoIter<Item = [`ProtoOption`](crate::options::ProtoOption)>.
/// After that, the fields for this oneof can be defined as a comma separated list of `$number:literal => $field:expr`, with $field evaluating to a [`FieldBuilder`](crate::fields::FieldBuilder).
/// # Examples
/// ```
/// use protoschema::{oneof, proto_option, string};
///
/// let my_opt = proto_option("my_opt", true);
/// let oneof = oneof!(
///   "my_oneof",
///   options = [ my_opt ],
///   1 => string!("abc")
/// );
/// ```
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
