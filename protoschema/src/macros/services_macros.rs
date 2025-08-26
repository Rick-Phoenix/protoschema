#[doc(hidden)]
#[macro_export]
macro_rules! handler {
  ($handler:ident($request:expr => $response:expr) $($options:expr)?) => {
    $crate::services::ServiceHandler::new(stringify!($handler).into())
      .request(&$request)
      .response(&$response)
      $(.options($options))?
      .build()
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! service {
  ($file:ident, $name:ident { options = $service_options:expr, $($handler_name:ident($request:ident => $response:ident) $({ $handler_options:expr })?),+ $(,)? } $(;)?) => {
    $file
      .new_service(stringify!($name))
      .handlers([
        $($crate::handler!($handler_name($request => $response) $($handler_options)?)),*
      ])
      .options($service_options)
  };

  ($file:ident, $name:ident { $($handler_name:ident($request:ident => $response:ident) $({ $handler_options:expr })?),+ $(,)? } $(;)?) => {
    $file
      .new_service(stringify!($name))
      .handlers([
        $($crate::handler!($handler_name($request => $response) $($handler_options)?)),*
      ])
  };
}

/// Creates a list of new services and adds them to a [`FileBuilder`](crate::files::FileBuilder).
///
/// The first argument is the ident of the FileBuilder where these services will be added.
/// After that, the syntax is very similar to the protobuf syntax, and it consists of an ident for the service's name, followed by a block inside curly brackets where the service's options can optionally be defined at the top, followed by the handlers, which are defined like in protobuf, where the idents between parentheses should refer to the [`MessageBuilder`](crate::messages::MessageBuilder) instance of the message being received/returned from a handler.
///
/// # Examples
/// ```rust
/// use protoschema::{services, proto_option, Package};
///
/// let my_pkg = Package::new("my_pkg");
/// let my_file = my_pkg.new_file("my_file");
///
/// let my_opt = proto_option("my_opt", true);
/// let my_list_of_options = [ my_opt.clone(), my_opt.clone() ];
///
/// let my_request = my_file.new_message("MyRequest");
/// let my_response = my_file.new_message("MyResponse");
///
/// services!(
///   my_file,
///   MyService {
///     // Options can only be defined at the top of a service's block
///     options = [ my_opt.clone() ], // Or `options = my_list_of_options.clone()`
///     GetUser(my_request => my_response) { [ my_opt.clone() ] },
///     GetData(my_request => my_response),
///   };
///
///   MyOtherService {
///     GetSomething(my_request => my_response),
///   };
/// );
/// ```
#[macro_export]
macro_rules! services {
  ($file:ident, $($service_name:ident { $($service:tt)* });+ $(;)?) => {
    {
      $(
        $crate::service!($file, $service_name { $($service)* })
      );*
    }
  };
}
