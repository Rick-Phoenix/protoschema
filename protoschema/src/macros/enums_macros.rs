/// Creates a new protobuf enum.
///
/// It receives an expression evaluating to a [`EnumBuilder`](crate::enums::EnumBuilder) instance as the first argument, and attaches to it the variants and options defined in this macro.
/// The syntax for this macro is as follows:
/// The first argument, as described above, must be followed by a comma.
/// After the comma, you can use:
/// `options = $options:expr` where $options should evaluate to IntoIter<Item = [`ProtoOption`](crate::options::ProtoOption)> (must be followed by a comma, even if at last position).
/// `reserved_names = $names:expr` where $names should evaluate to `IntoIter<Item = AsRef<str>>` (must be followed by a comma, even if last)
/// reserved = [ $items ], where $items is a comma separated list of numbers of ranges such as `1..5`. Following the protobuf syntax, these ranges will be inclusive.
/// The variants for this enum, defined as a comma-separated list of `$number:literal => $name:expr`.
/// # Examples
/// ```
/// use protoschema::{Package, enum_variants, proto_enum, proto_option};
///
/// let my_pkg = Package::new("my_pkg");
/// let my_file = my_pkg.new_file("my_file");
/// let reusable_variants = enum_variants!(
///   0 => "UNSPECIFIED"
/// );
/// let my_opt = proto_option("my_opt", true);
///
/// // For enums defined at the top level
/// let my_enum = proto_enum!(
///   my_file.new_enum("my_enum"),
///   // Options, if defined, must be at the very top
///   options = [ my_opt.clone() ],
///   // Must be followed by a comma, even if last
///   reserved_names = [ "ABC" ],
///   // Accepts ranges (inclusive by default) and numbers
///   reserved = [ 100, 205, 300..350 ]
///   
///   // Including reusable variants
///   include(reusable_variants.clone()),
///   1 => "ACTIVE"
/// );
/// ```
#[macro_export]
macro_rules! proto_enum {
  ($enum:expr, $($tokens:tt)*) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options(),
      @reserved(),
      @reserved_names(),
      @variants(),
      @included_variants(),
      @rest($($tokens)*)
    }
  };
}

/// Defines some enum variants that can be included and reused among different enums.
///
/// # Examples
/// ```
/// use protoschema::enum_variants;
///
/// let variants = enum_variants!(
///   0 => "UNSPECIFIED",
///   1 => "ACTIVE"
/// );
///
/// ```
#[macro_export]
macro_rules! enum_variants {
  ($($number:literal => $name:expr),+ $(,)?) => {
    [ $(($number, $name)),* ]
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proto_enum_impl {
  (
    @builder($enum:expr),
    @options($($options:expr)?),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:expr)?),
    @variants($($variants:tt)*),
    @included_variants($($included_variants:expr,)*),
    @rest($(,)?)
  ) => {
    {
      let mut variants = vec! [ $($variants)* ];
      $(variants.extend($included_variants));*;

      let mut temp_enum = $enum

      $(
        .options($options)
      )?

      $(
        .reserved_names($reserved_names)
      )?

      .variants(
        variants
      );

      $crate::parse_reserved!{
        @builder(temp_enum)
        @ranges()
        @numbers()
        @rest($($reserved)*)
      }
    }
  };

  (
    @builder($enum:expr),
    @options($($options:tt)*),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:tt)*),
    @variants($($variants:tt)*),
    @included_variants($($included_variants:tt)*),
    @rest($(,)? include($variants_block:expr) $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @variants($($variants)*),
      @included_variants($($included_variants)* $variants_block,),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options(),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:tt)*),
    @variants($($variants:tt)*),
    @included_variants($($included_variants:tt)*),
    // Expr must be followed by a comma
    @rest($(,)? options = $options_expr:expr, $($rest:tt)* )
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($options_expr),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @variants($($variants)*),
      @included_variants($($included_variants)*),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:tt)*),
    @reserved(),
    @reserved_names($($reserved_names:tt)*),
    @variants($($variants:tt)*),
    @included_variants($($included_variants:tt)*),
    @rest($(,)? reserved = [ $($items:tt)* ] $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($items)*),
      @reserved_names($($reserved_names)*),
      @variants($($variants)*),
      @included_variants($($included_variants)*),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:tt)*),
    @reserved($($reserved:tt)*),
    @reserved_names(),
    @variants($($variants:tt)*),
    @included_variants($($included_variants:tt)*),
    // Expr must be followed by a comma
    @rest($(,)? reserved_names = $reserved_names:expr, $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names($reserved_names),
      @variants($($variants)*),
      @included_variants($($included_variants)*),
      @rest($($rest)*)
    }
  };

  // Variant with trailing comma
  (
    @builder($enum:expr),
    @options($($options:tt)?),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:tt)*),
    @variants($($variants:tt)*),
    @included_variants($($included_variants:tt)*),
    @rest($(,)? $tag:literal => $variant:expr, $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @variants($($variants)* ($tag, $variant),),
      @included_variants($($included_variants)*),
      @rest($($rest)*)
    }
  };

  // Variant without trailing comma
  (
    @builder($enum:expr),
    @options($($options:tt)?),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:tt)*),
    @variants($($variants:tt)*),
    @included_variants($($included_variants:tt)*),
    @rest($(,)? $tag:literal => $variant:expr)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @variants($($variants)* ($tag, $variant)),
      @included_variants($($included_variants)*),
      @rest()
    }
  };
}
