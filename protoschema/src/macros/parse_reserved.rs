#[macro_export]
macro_rules! parse_reserved {
  (
    @builder($builder:expr)
    @ranges()
    @numbers()
    @rest($(,)?)
  ) => {
     $builder
  };

  (
    @builder($builder:expr)
    @ranges($($start:literal..$end:literal),* $(,)?)
    @numbers()
    @rest($(,)?)
  ) => {
     $builder
      .reserved_ranges(&[$(::std::ops::Range { start: $start, end: $end }),*])
  };

  (
    @builder($builder:expr)
    @ranges()
    @numbers($($number:literal),* $(,)?)
    @rest($(,)?)
  ) => {
     $builder
      .reserved_numbers(&[$($number),*])
  };

  (
    @builder($builder:expr)
    @ranges($($start:literal..$end:literal),* $(,)?)
    @numbers($($number:literal),* $(,)?)
    @rest($(,)?)
  ) => {
     $builder
      .reserved_ranges(&[$(::std::ops::Range { start: $start, end: $end }),*])
      .reserved_numbers(&[$($number),*])
  };

  (
    @builder($builder:expr)
    @ranges($($ranges:tt)*)
    @numbers($($numbers:tt)*)
    @rest($(,)? $start:literal..$end:literal $($rest:tt)* )
  ) => {
    $crate::parse_reserved!{
      @builder($builder)
      @ranges($($ranges)* $start..$end,)
      @numbers($($numbers)*)
      @rest($($rest)*)
    }
  };

  (
    @builder($builder:expr)
    @ranges($($ranges:tt)*)
    @numbers($($numbers:tt)*)
    @rest($(,)? $number:literal $($rest:tt)* )
  ) => {
    $crate::parse_reserved!{
      @builder($builder)
      @ranges($($ranges)*)
      @numbers($($numbers)* $number,)
      @rest($($rest)*)
    }
  };
}
