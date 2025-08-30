mod enums_macros;
mod extensions_macros;
mod fields_macros;
mod maps_macros;
mod oneofs_macros;
mod options_macros;
mod parse_reserved;
mod services_macros;

/// A macro to define a single [`CelRule`](crate::validators::cel::CelRule).
///
/// # Examples
/// ```
/// use protoschema::{cel_rule, string};
///
/// // As an invidivual rule
/// let rule = cel_rule!(
///   id = "password_not_matching",
///   msg = "the two passwords do not match",
///   expr = "this.password == this.repeated_password"
/// );
///
/// // As part of a validator definition
/// let my_field_with_cel_rule = string!("name", |v| v.cel([
///   cel_rule!(
///     id = "is_geronimo",
///     msg = "is not geronimo",
///     expr = "this == 'geronimo'"
/// ) ]));
/// ```
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

#[doc(hidden)]
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

/// The macro that is used to define most if not all of the data for a given protobuf message.
///
/// It receives a [`MessageBuilder`](crate::messages::MessageBuilder) instance's ident as the first argument, the (optional) options for the message right after, and the rest of the data after that.
/// It consumes the original builder and returns a new one.
/// # Examples
/// ```
/// use protoschema::{Package, message, proto_option, string, reusable_fields, uint64, timestamp};
///
/// let my_pkg = Package::new("mypkg.v1");
/// let my_file = my_pkg.new_file("my_file");
/// let my_opt = proto_option("my_opt", true);
/// let my_list_of_options = [ my_opt.clone(), my_opt.clone() ];
///
/// let my_common_fields = reusable_fields!(
///   1 => uint64!("id"),
///   2 => timestamp!("created_at"),
///   3 => timestamp!("updated_at")
/// );
///
///
/// message!(
///   my_file.new_message("my_msg"),
///   // Options can only be defined at the very top
///   options  = [ my_opt.clone() ], // Or `options = my_list_of_options.clone()`
///   // reserved_names must always be followed by a comma because it's an expression, even if it's the last item in the list
///   reserved_names = [ "abc", "deg" ],
///   // Accepts both numbers and ranges
///   reserved = [ 5, 12, 23..29 ],
///
///   // Single field
///   10 => string!("abc"),
///   // Included reusable fields
///   include(my_common_fields),
/// );
/// ```
///
/// It's also possible to define enums and oneofs inside of this macro. They follow the same syntax as their respective macros, namely [`proto_enum`](crate::proto_enum) and [`oneof`](crate::oneof)
///
/// ```
/// use protoschema::{message, Package, proto_option, string};
///
/// let my_pkg = Package::new("my_pkg");
/// let my_file = my_pkg.new_file("my_file");
/// let my_option = proto_option("my_option", true);
/// let my_list_of_options = [ my_option.clone(), my_option.clone() ];
///
/// message!(
///   my_file.new_message("my_msg"),
///   1 => string!("my_field"),
///
///   enum "MyEnum" {
///     options = [ my_option ],
///     reserved_names = [ "ABCDE" ],
///     0 => "UNSPECIFIED"
///   }
///
///   oneof "MyOneOf" {
///     // Optionally, we can mark this oneof as required by adding the 'required' keyword
///     // as the first element inside the brackets.
///     required,
///     options = my_list_of_options,
///     2 => string!("abc"),
///     3 => string!("deg"),
///   }
/// );
/// ```
#[macro_export]
macro_rules! message {
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

#[doc(hidden)]
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
        .add_oneofs(oneofs_list)
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

  // Handle included oneof
  (
    @builder($builder:expr)
    @fields($($fields:tt)*)
    @fields_blocks($($fields_blocks:tt)*)
    @oneofs($($oneofs:tt)*)
    @enums($($enums:tt)*)
    @reserved($($reserved:tt)*)
    @reserved_names($($reserved_names:tt)*)
    @cel($($cel:tt)*)
    @input($(,)? include_oneof($oneof:expr) $($rest:tt)*)
  ) => {
    $crate::_internal_message_body! {
      @builder($builder)
      @fields($($fields)*)
      @fields_blocks($($fields_blocks)*)
      @oneofs($($oneofs)* $oneof.clone(),)
      @enums($($enums)*)
      @reserved($($reserved)*)
      @reserved_names($($reserved_names)*)
      @cel($($cel)*)
      @input($($rest)*)
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
      @fields_blocks($($fields_blocks)* $block.clone(),)
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
      @fields($($fields)* ($tag, $field),)
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
      @fields($($fields)* ($tag, $field))
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
