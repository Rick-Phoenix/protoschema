/// Creates a new protobuf enum.
///
/// It receives an expression evaluating to a [`EnumBuilder`](crate::enums::EnumBuilder) instance as the first argument, and attaches to it the variants and options defined in this macro.
/// It consumes the original builder and returns a new one.
/// The syntax for this macro is as follows:
/// The first argument, as described above, must be followed by a comma.
/// After the comma, you can use:
/// `options = $options:expr` where $options should evaluate to IntoIter<Item = [`ProtoOption`](crate::options::ProtoOption)> (must be followed by a comma, even if at last position).
/// `reserved_names = $names:expr` where $names should evaluate to `IntoIter<Item = AsRef<str>>` (must be followed by a comma, even if last)
/// reserved = [ $items ], where $items is a comma separated list of numbers of ranges such as `1..5`. Following the protobuf syntax, these ranges will be inclusive.
/// The variants for this enum, defined as a comma-separated list of `$number:literal => $name:expr` with the optional options (that's a mouthful) being defined as an expression in curly brackets.
/// <br/>
/// **Note**: You do not need to add the enum name as a prefix to the variants. It will be added automatically.
/// So if an enum is named "my_enum", and the variant is "UNSPECIFIED", the output will show "MY_ENUM_UNSPECIFIED".
/// # Examples
/// ```
/// use protoschema::{Package, enum_variants, proto_enum, proto_option};
///
/// let my_pkg = Package::new("my_pkg");
/// let my_file = my_pkg.new_file("my_file");
///
/// let my_opt = proto_option("my_opt", true);
/// let my_list_of_opts = [ my_opt.clone(), my_opt.clone() ];
///
/// let reusable_variants = enum_variants!(
///   0 => "UNSPECIFIED"
/// );
///
/// // For enums defined at the top level
/// let my_enum = proto_enum!(
///   my_file.new_enum("my_enum"),
///   // Options, if defined, must be at the very top
///   options = [ my_opt.clone() ], // Or `options = my_list_of_opts.clone()`
///   // Must be followed by a comma, even if last
///   reserved_names = [ "ABC" ],
///   // Accepts ranges (inclusive by default) and numbers
///   reserved = [ 100, 205, 300..350 ]
///   
///   // Include reusable variants
///   include(reusable_variants),
///   // Define normal variants
///   1 => "ACTIVE" { my_list_of_opts.clone() },
///   2 => "CONNECTED" { [ my_opt.clone() ] },
///   3 => "DISCONNECTED"
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
/// You can optionally add a list of imports to the macro (which must be an `IntoIter<Item = Into<Arc<str>>>`) which will be added to the receiving file whenever this block of variants is reused.
///
/// **Note:** The prefix with the enum name will be added when these fields are included in an enum.
/// So if an enum is named "my_enum", and the variant is "UNSPECIFIED", the output will show "MY_ENUM_UNSPECIFIED".
///
/// # Examples
/// ```
/// use protoschema::{enum_variants, proto_option};
///
/// let my_opt = proto_option("my_opt", true);
/// let my_list_of_opts = [ my_opt.clone(), my_opt.clone() ];
///
/// let variants = enum_variants!(
///   imports = ["my_pkg/reusable/import.proto"],
///   0 => "UNSPECIFIED" { my_list_of_opts.clone() },
///   1 => "ACTIVE" { [ my_opt.clone() ] },
///   2 => "DISCONNECTED"
/// );
///
/// ```
#[macro_export]
macro_rules! enum_variants {
  ($(imports = $imports:expr,)? $($number:literal => $name:literal $({ $options:expr })?),+ $(,)? ) => {
    $crate::enums::ReusableVariants::builder()
      .variants(
        Box::new([ $(($number, $crate::enums::EnumVariant::builder()
        .name($name)
        $(.options($options))?
        .build()
        )),* ])
      )
      $(.imports($imports))?
      .build()
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
      let included_variants: Vec<$crate::enums::ReusableVariants> = vec! [ $($included_variants),* ];

      let mut temp_enum1 = $enum;

      if !included_variants.is_empty() {
        for group in included_variants.into_iter() {
          variants.extend(group.variants);
          temp_enum1 = temp_enum1.add_imports(group.imports);
        }
      }

      let temp_enum2 = temp_enum1

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
        @builder(temp_enum2)
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
      @included_variants($($included_variants)* $variants_block.clone(),),
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
    @rest($(,)? $tag:literal => $variant:literal $({ $enum_value_options:expr })?, $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @variants($($variants)*
        ($tag, $crate::enums::EnumVariant::builder().name($variant)
        $(.options($enum_value_options))?
        .build()
        ),),
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
    @rest($(,)? $tag:literal => $variant:literal $({ $enum_value_options:expr })?)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @variants($($variants)* ($tag, $crate::enums::EnumVariant::builder()
        .name($variant)
        $(.options($enum_value_options))?
        .build()
      )),
      @included_variants($($included_variants)*),
      @rest()
    }
  };
}
