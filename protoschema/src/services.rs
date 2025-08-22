use std::{marker::PhantomData, sync::Arc};

use bon::Builder;

use crate::{
  field_type::strip_common_prefix, message::MessageBuilder, schema::Arena, sealed, Empty, IsSet,
  IsUnset, ProtoOption, Set, Unset,
};

#[derive(Clone, Debug)]
pub struct ServiceBuilder<S: ServiceState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) file_id: usize,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

#[derive(Clone, Debug, Builder)]
#[builder(start_fn = new)]
pub struct ServiceHandler {
  #[builder(start_fn)]
  pub(crate) name: Box<str>,
  #[builder(field)]
  pub(crate) imports: Vec<Arc<str>>,
  #[builder(setters(vis = "", name = options_internal))]
  #[builder(default)]
  pub(crate) options: Box<[ProtoOption]>,
  #[builder(setters(vis = "", name = request_internal))]
  pub(crate) request: Arc<str>,
  #[builder(setters(vis = "", name = response_internal))]
  pub(crate) response: Arc<str>,
}

impl ServiceHandler {
  pub fn render_request(&self, package: &str) -> Box<str> {
    strip_common_prefix(&self.request, &format!("{}.", package)).into()
  }

  pub fn render_response(&self, package: &str) -> Box<str> {
    strip_common_prefix(&self.response, &format!("{}.", package)).into()
  }
}

use service_handler_builder::{
  IsUnset as HandlerIsUnset, SetOptions as HandlerSetOptions, SetRequest, SetResponse,
  State as HandlerState,
};

impl<S: HandlerState> ServiceHandlerBuilder<S> {
  pub fn options(self, options: &[ProtoOption]) -> ServiceHandlerBuilder<HandlerSetOptions<S>>
  where
    S::Options: HandlerIsUnset,
  {
    self.options_internal(options.into())
  }

  pub fn add_import(mut self, import: &str) -> ServiceHandlerBuilder<S> {
    self.imports.push(import.into());
    self
  }

  pub fn request(self, message: &MessageBuilder) -> ServiceHandlerBuilder<SetRequest<S>>
  where
    S::Request: HandlerIsUnset,
  {
    self
      .add_import(&message.get_file())
      .request_internal(message.get_full_name())
  }

  pub fn response(self, message: &MessageBuilder) -> ServiceHandlerBuilder<SetResponse<S>>
  where
    S::Response: HandlerIsUnset,
  {
    self
      .add_import(&message.get_file())
      .response_internal(message.get_full_name())
  }
}

#[derive(Clone, Debug, Default)]
pub struct ServiceData {
  pub imports: Vec<Box<str>>,
  pub name: Box<str>,
  pub handlers: Box<[ServiceHandler]>,
  pub package: Arc<str>,
  pub options: Box<[ProtoOption]>,
}

impl<S: ServiceState> ServiceBuilder<S> {
  pub fn get_data(self) -> ServiceData {
    self.arena.borrow().services[self.id].clone()
  }

  pub fn get_file(&self) -> Arc<str> {
    let arena = self.arena.borrow();
    arena.files[self.file_id].name.clone()
  }

  pub fn get_name(&self) -> Box<str> {
    let arena = self.arena.borrow();

    arena.services[self.id].name.clone()
  }

  pub fn get_package(&self) -> Arc<str> {
    self.arena.borrow().name.clone()
  }

  pub fn handlers(self, handlers: &[ServiceHandler]) -> ServiceBuilder<SetHandlers<S>>
  where
    S::Handlers: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();

      for handler in handlers {
        handler.imports.iter().for_each(|i| {
          arena.files[self.file_id].imports.insert(i.clone());
        });
      }

      arena.services[self.id].handlers = handlers.into()
    }

    ServiceBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  pub fn options(self, options: &[ProtoOption]) -> ServiceBuilder<SetOptions<S>>
  where
    S::Options: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let service = &mut arena.services[self.id];

      service.options = options.into()
    }

    ServiceBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }
}

pub trait ServiceState: Sized {
  type Handlers;
  type Options;
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}

#[allow(non_camel_case_types)]
mod members {
  pub struct handlers;
  pub struct options;
}

pub trait IsComplete: ServiceState {
  #[doc(hidden)]
  const SEALED: sealed::Sealed;
}
#[doc(hidden)]
impl<S: ServiceState> IsComplete for S
where
  S::Handlers: IsSet,
  S::Options: IsSet,
{
  const SEALED: sealed::Sealed = sealed::Sealed;
}

pub struct SetHandlers<S: ServiceState = Empty>(PhantomData<fn() -> S>);
pub struct SetOptions<S: ServiceState = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl ServiceState for Empty {
  type Handlers = Unset<members::handlers>;
  type Options = Unset<members::options>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: ServiceState> ServiceState for SetHandlers<S> {
  type Handlers = Set<members::handlers>;
  type Options = S::Options;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: ServiceState> ServiceState for SetOptions<S> {
  type Handlers = S::Handlers;
  type Options = Set<members::options>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
