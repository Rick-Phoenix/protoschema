use std::marker::PhantomData;

use crate::{
  message::MessageBuilder, schema::Arena, sealed, Empty, IsSet, IsUnset, ProtoOption, Set, Unset,
};

#[derive(Clone, Debug)]
pub struct ServiceBuilder<S: ServiceState = Empty> {
  pub(crate) id: usize,
  pub(crate) arena: Arena,
  pub(crate) _phantom: PhantomData<fn() -> S>,
}

#[derive(Clone, Debug)]
pub struct ServiceHandler {
  name: Box<str>,
  request_id: usize,
  response_id: usize,
}

impl ServiceHandler {
  pub fn new(name: &str, request: &MessageBuilder, response: &MessageBuilder) -> Self {
    ServiceHandler {
      name: name.into(),
      request_id: request.get_id(),
      response_id: response.get_id(),
    }
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

  pub fn handlers(self, handlers: &[ServiceHandler]) -> ServiceBuilder<SetOptions<S>>
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
