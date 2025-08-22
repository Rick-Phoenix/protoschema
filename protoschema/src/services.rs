use std::marker::PhantomData;

use bon::Builder;

use crate::{
  message::MessageBuilder, schema::Arena, sealed, Empty, IsSet, IsUnset, ProtoOption, Set, Unset,
};

#[derive(Clone, Debug)]
pub struct ServiceBuilder<S: ServiceState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

#[derive(Clone, Debug, Builder)]
#[builder(start_fn = new)]
pub struct ServiceHandler {
  #[builder(start_fn)]
  name: Box<str>,
  #[builder(setters(vis = "", name = options_internal))]
  options: Option<Box<[ProtoOption]>>,
  #[builder(setters(vis = "", name = request_id_internal))]
  request_id: usize,
  #[builder(setters(vis = "", name = response_id_internal))]
  response_id: usize,
}

use service_handler_builder::{
  IsUnset as HandlerIsUnset, SetOptions as HandlerSetOptions, SetRequestId, SetResponseId,
  State as HandlerState,
};

impl<S: HandlerState> ServiceHandlerBuilder<S> {
  pub fn options(self, options: &[ProtoOption]) -> ServiceHandlerBuilder<HandlerSetOptions<S>>
  where
    S::Options: HandlerIsUnset,
  {
    self.options_internal(options.into())
  }

  pub fn request(self, message: &MessageBuilder) -> ServiceHandlerBuilder<SetRequestId<S>>
  where
    S::RequestId: HandlerIsUnset,
  {
    self.request_id_internal(message.get_id())
  }

  pub fn response(self, message: &MessageBuilder) -> ServiceHandlerBuilder<SetResponseId<S>>
  where
    S::ResponseId: HandlerIsUnset,
  {
    self.response_id_internal(message.get_id())
  }
}

#[derive(Clone, Debug, Default)]
pub struct ServiceData {
  pub name: Box<str>,
  pub handlers: Box<[ServiceHandler]>,
  pub file_id: usize,
  pub package: Box<str>,
  pub options: Box<[ProtoOption]>,
}

impl<S: ServiceState> ServiceBuilder<S> {
  pub fn get_file(&self) -> Box<str> {
    let arena = self.arena.borrow();
    let file_id = arena.services[self.id].file_id;
    arena.files[file_id].name.clone()
  }

  pub fn handlers(self, handlers: &[ServiceHandler]) -> ServiceBuilder<SetHandlers<S>>
  where
    S::Handlers: IsUnset,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let service = &mut arena.services[self.id];

      service.handlers = handlers.into()
    }

    ServiceBuilder {
      id: self.id,
      arena: self.arena,
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
      _phantom: PhantomData,
    }
  }

  pub fn get_name(&self) -> Box<str> {
    let arena = self.arena.borrow();

    arena.services[self.id].name.clone()
  }

  pub fn get_package(&self) -> Box<str> {
    self.arena.borrow().name.clone()
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
