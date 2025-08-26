/// Evaluates to a [`Oneof`](crate::oneofs::Oneof) instance.
///
/// The first argument is the name of the oneof, followed by a comma, optionally followed by this oneof's options defined as `options = $options` where $options should evaluate to IntoIter<Item = [`ProtoOption`](crate::options::ProtoOption)>.
/// After that, the fields for this oneof can be defined as a comma separated list of `$number:literal => $field:expr`, with $field evaluating to a [`FieldBuilder`](crate::fields::FieldBuilder).
/// # Examples
/// ```
/// use protoschema::{oneof, proto_option, string, reusable_fields, uint64};
///
/// let my_common_fields = reusable_fields!(
///   1 => uint64!("id"),
///   2 => string!("username"),
///   3 => string!("email")
/// );
///
/// let my_opt = proto_option("my_opt", true);
/// let my_list_of_options = [ my_opt.clone(), my_opt.clone() ];
///
/// let oneof = oneof!(
///   "my_oneof",
///   options = [ my_opt ], // Or `options = my_list_of_options.clone()`
///   // Add a normal field
///   1 => string!("abc"),
///   // Include some reusable fields
///   include(my_common_fields)
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
