/// Evaluates to a [`Oneof`](crate::oneofs::Oneof) instance.
///
/// The first argument is the name of the oneof, followed by a comma, optionally followed by this oneof's options defined as `options = $options` where $options should evaluate to IntoIter<Item = [`ProtoOption`](crate::options::ProtoOption)>, followed by the fields for this oneof, which are defined as a comma separated list of `$number:literal => $field:expr`, with $field evaluating to a [`FieldBuilder`](crate::fields::FieldBuilder) or include($reusable_fields:expr), where $reusable_fields is the expansion of a call to [`reusable_fields`](crate::reusable_fields).
/// If you want to add the "required" rule to this oneof (meaning at least one of the choices will need to be set to pass validation), you must place the "required" keyword right after the name of the oneof.
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
///   required,
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
    required,
    $($tokens:tt)*
  ) => {
    $crate::oneof_impl!(
      @name($name)
      @options()
      @included_fields()
      @fields()
      @imports()
      @rest($($tokens)*)
    )
    .required()
    .build()
  };

  (
    $name:expr,
    $($tokens:tt)*
  ) => {
    $crate::oneof_impl!(
      @name($name)
      @options()
      @included_fields()
      @fields()
      @imports()
      @rest($($tokens)*)
    )
    .build()
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! oneof_impl {
  (
    @name($name:expr)
    @options($($options:expr)?)
    @included_fields($($included_fields:expr,)*)
    @fields($($fields:tt)*)
    @imports($($imports:expr)?)
    @rest($(,)?)
  ) => {
    $crate::oneofs::Oneof::builder()
    .name($name.into())
    $(.add_options($options))?
    .fields(
      {
        let mut fields = vec! [ $($fields)* ];
        $(fields.extend($included_fields));*;
        fields
      }
    )
    $(.add_imports($imports))?
  };

  (
    @name($name:expr)
    @options()
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @imports($($imports:tt)?)
    @rest($(,)? options = $options:expr, $($rest:tt)*)
  ) => {
    $crate::oneof_impl!(
      @name($name)
      @options($options)
      @included_fields($($included_fields)*)
      @fields($($fields)*)
      @imports($($imports)*)
      @rest($($rest)*)
    )
  };

  (
    @name($name:expr)
    @options($($options:tt)?)
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @imports()
    @rest($(,)? imports = $imports:expr, $($rest:tt)*)
  ) => {
    $crate::oneof_impl!(
      @name($name)
      @options($($options)?)
      @included_fields($($included_fields)*)
      @fields($($fields)*)
      @imports($imports)
      @rest($($rest)*)
    )
  };

  (
    @name($name:expr)
    @options($($options:tt)?)
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @imports($($imports:tt)?)
    @rest($(,)? include($new_included_fields:expr) $($rest:tt)*)
  ) => {
    $crate::oneof_impl!(
      @name($name)
      @options($($options)?)
      @included_fields($($included_fields)* $new_included_fields.clone(),)
      @fields($($fields)*)
      @imports($($imports)*)
      @rest($($rest)*)
    )
  };

  (
    @name($name:expr)
    @options($($options:tt)?)
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @imports($($imports:tt)?)
    @rest($(,)? $tag:literal => $field:expr, $($rest:tt)*)
  ) => {
    $crate::oneof_impl!(
      @name($name)
      @options($($options)?)
      @included_fields($($included_fields)*)
      @fields($($fields)* ($tag, $field),)
      @imports($($imports)*)
      @rest($($rest)*)
    )
  };

  (
    @name($name:expr)
    @options($($options:tt)?)
    @included_fields($($included_fields:tt)*)
    @fields($($fields:tt)*)
    @imports($($imports:tt)?)
    @rest($(,)? $tag:literal => $field:expr)
  ) => {
    $crate::oneof_impl!(
      @name($name)
      @options($($options)?)
      @included_fields($($included_fields)*)
      @fields($($fields)* ($tag, $field))
      @imports($($imports)*)
      @rest()
    )
  };
}
