mod enums_macros;
mod extensions_macros;
mod fields_macros;
mod maps_macros;
mod oneofs_macros;
mod parse_reserved;
mod services_macros;

#[macro_export]
macro_rules! cel_rule {
  (id = $id:expr, msg = $msg:expr, expr = $expr:expr) => {
    $crate::validators::cel::CelRule {
      id: $id.into(),
      message: $msg.into(),
      expression: $expr.into(),
    }
  };
}

#[macro_export]
macro_rules! cel_rules {
  (
    @rules($($rules:tt)*)
    @rest($(,)?)
  ) => {
    [ $($rules)* ]
  };

  (
    @rules($($rules:tt)*)
    @rest({ $($rule_tokens:tt)* } $($rest:tt)*)
  ) => {
    $crate::cel_rules!(
      @rules($($rules)* $crate::cel_rule!($($rule_tokens)*),)
      @rest($($rest)*)
    )
  };
}

#[macro_export]
macro_rules! message_body {
  ($msg_builder:expr, options = $options:expr, $($tokens:tt)*) => {
    $crate::_internal_message_body! {
      @builder($msg_builder)
      @fields()
      @fields_blocks()
      @oneofs()
      @enums()
      @reserved()
      @reserved_names()
      @cel()
      @input($($tokens)*)
    }.add_options($options)
  };

  ($msg_builder:expr, $($tokens:tt)*) => {
    $crate::_internal_message_body! {
      @builder($msg_builder)
      @fields()
      @fields_blocks()
      @oneofs()
      @enums()
      @reserved()
      @reserved_names()
      @cel()
      @input($($tokens)*)
    }
  };
}

#[macro_export]
macro_rules! _internal_message_body {
  // No tokens remaining, process items
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:expr,)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($names:expr)?)
    @cel($($cel_rules:expr)?)
    @input($(,)?)
  ) => {
    {
      { $($enums)* };

      let mut fields_list = vec! [ $($fields)* ];
      $(fields_list.extend($fields_blocks));*;

      let oneofs_list = [ $($oneofs)* ];

      let mut new_msg = $builder
        .fields(fields_list)
        .oneofs(oneofs_list)
      $(
        .cel_rules($cel_rules)
      )?

      $(
        .reserved_names($names)
      )?;

      $crate::parse_reserved! {
        @builder(new_msg)
        @ranges()
        @numbers()
        @rest($($reserved)*)
      }
    }
  };

  // Handle field blocks
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @cel($($cel:tt)*)
    @input($(,)? include($block:expr) $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @fields_blocks($($fields_blocks)* $block,)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @cel($($cel)*)
      @input($($rest)*)
    }
  };

  // Cel rules
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @cel()
    @input($(,)? cel = [ $($items:tt)* ] $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @fields_blocks($($fields_blocks)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @cel($crate::cel_rules!(
        @rules()
        @rest($($items)*)
      ))
      @input($($rest)*)
    }
  };

  // Reserved numbers
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved()
    @reserved_names($($reserved_names:tt)*)
    @cel($($cel:tt)*)
    @input($(,)? reserved = [ $($items:tt)* ] $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @fields_blocks($($fields_blocks)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($items)*)
      @reserved_names($($reserved_names)*)
      @cel($($cel)*)
      @input($($rest)*)
    }
  };

  // Reserved names
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names()
    @cel($($cel:tt)*)
    // Expr must be followed by a comma
    @input($(,)? reserved_names = $reserved_names:expr, $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @fields_blocks($($fields_blocks)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($reserved_names)
      @cel($($cel)*)
      @input($($rest)*)
    }
  };

  // Handle enum
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @cel($($cel:tt)*)
    @input($(,)? enum $name:literal { $($tokens:tt)* } $($rest:tt)* )
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @fields_blocks($($fields_blocks)*)
      @oneofs($($oneofs)*)
      @enums($crate::proto_enum!($builder.new_enum($name), $($tokens)*); $($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @cel($($cel)*)
      @input($($rest)*)
    }
  };

  // Handle oneof
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @cel($($cel:tt)*)
    @input($(,)? oneof $name:literal { $($oneof_body:tt)* } $($rest:tt)* )
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @fields_blocks($($fields_blocks)*)
      @oneofs(
        $($oneofs)*
        $crate::oneof!($name, $($oneof_body)*),
      )
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @cel($($cel)*)
      @input($($rest)*)
    }
  };

  // Process normal field with trailing comma
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @cel($($cel:tt)*)
    @input($(,)? $tag:literal => $field:expr, $($rest:tt)* )
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* $field.tag($tag),)
      @fields_blocks($($fields_blocks)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @cel($($cel)*)
      @input($($rest)*)
    }
  };

  // Process normal field (cannot have tt following an expr without a comma)
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @cel($($cel:tt)*)
    @input($(,)? $tag:literal => $field:expr)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)* $field.tag($tag))
      @fields_blocks($($fields_blocks)*)
      @oneofs($($oneofs)*)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @cel($($cel)*)
      @input()
    }
  };
}
