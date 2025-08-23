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

#[macro_export]
macro_rules! service {
  ($file:ident, $name:ident { options = $service_options:expr; $($handler_name:ident($request:ident => $response:ident) $([ $($handler_options:tt)+ ])?);+ $(;)? } $(;)?) => {
    $file
      .new_service(stringify!($name).into())
      .handlers([
        $($crate::handler!($handler_name($request => $response) $([ $($handler_options)+ ])?)),*
      ])
      .options($service_options)
  };
}

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
