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
