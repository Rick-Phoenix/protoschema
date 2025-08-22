#[macro_export]
macro_rules! proto_enum {
  ($enum:expr, $($tokens:tt)*) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options(),
      @reserved(),
      @reserved_names(),
      @rest($($tokens)*)
    }
  };
}

#[macro_export]
macro_rules! proto_enum_impl {
  (
    @builder($enum:expr),
    @options(),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:tt)*),
    // Expr must be followed by a comma
    @rest($(,)? options = $options_expr:expr, $($rest:tt)* )
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($options_expr),
      @reserved($($reserved)*),
      @reserved_names($($reserved_names)*),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:tt)*),
    @reserved(),
    @reserved_names($($reserved_names:tt)*),
    @rest($(,)? reserved = [ $($items:tt)* ] $($rest:tt)* )
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($items)*),
      @reserved_names($($reserved_names)*),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:tt)*),
    @reserved($($reserved:tt)*),
    @reserved_names(),
    @rest($(,)? reserved_names = [ $($reserved_names:tt)* ] $($rest:tt)*)
  ) => {
    $crate::proto_enum_impl! {
      @builder($enum),
      @options($($options)*),
      @reserved($($reserved)*),
      @reserved_names(&[ $($reserved_names)* ]),
      @rest($($rest)*)
    }
  };

  (
    @builder($enum:expr),
    @options($($options:expr)?),
    @reserved($($reserved:tt)*),
    @reserved_names($($reserved_names:expr)?),
    @rest($(,)? $($tag:literal => $variant:literal),* $(,)?)
  ) => {
    {
      let mut temp_enum = $enum

      $(
        .options($options.as_slice())
      )?

      $(
        .reserved_names($reserved_names.as_slice())
      )?

      .variants(
        &[ $(($tag, $variant.into())),* ]
      );

      $crate::parse_reserved!{
        @builder(temp_enum)
        @ranges()
        @numbers()
        @rest($($reserved)*)
      }
    }
  };
}
