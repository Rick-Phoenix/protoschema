use std::{marker::PhantomData, sync::Arc};

use bon::Builder;

use crate::{
  field_type::{get_shortest_item_name, ImportedItemPath},
  message::MessageBuilder,
  package::Arena,
  sealed, Empty, IsSet, IsUnset, ProtoOption, Set, Unset,
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
  #[builder(setters(vis = "", name = options_internal))]
  #[builder(default)]
  pub(crate) options: Box<[ProtoOption]>,
  #[builder(setters(vis = "", name = request_internal))]
  pub(crate) request: Arc<ImportedItemPath>,
  #[builder(setters(vis = "", name = response_internal))]
  pub(crate) response: Arc<ImportedItemPath>,
}

impl ServiceHandler {
  pub fn render_request(&self, current_file: &str, current_package: &str) -> Arc<str> {
    get_shortest_item_name(&self.request, current_file, current_package)
  }

  pub fn render_response(&self, current_file: &str, current_package: &str) -> Arc<str> {
    get_shortest_item_name(&self.response, current_file, current_package)
  }
}

use service_handler_builder::{
  IsUnset as HandlerIsUnset, SetOptions as HandlerSetOptions, SetRequest, SetResponse,
  State as HandlerState,
};

impl<S: HandlerState> ServiceHandlerBuilder<S> {
  pub fn options<I>(self, options: I) -> ServiceHandlerBuilder<HandlerSetOptions<S>>
  where
    S::Options: HandlerIsUnset,
    I: IntoIterator<Item = ProtoOption>,
  {
    self.options_internal(options.into_iter().collect())
  }

  pub fn request(self, message: &MessageBuilder) -> ServiceHandlerBuilder<SetRequest<S>>
  where
    S::Request: HandlerIsUnset,
  {
    self.request_internal(message.get_import_path())
  }

  pub fn response(self, message: &MessageBuilder) -> ServiceHandlerBuilder<SetResponse<S>>
  where
    S::Response: HandlerIsUnset,
  {
    self.response_internal(message.get_import_path())
  }
}

#[derive(Clone, Debug, Default)]
pub struct ServiceData {
  pub name: Box<str>,
  pub handlers: Box<[ServiceHandler]>,
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

  pub fn handlers<I>(self, handlers: I) -> ServiceBuilder<SetHandlers<S>>
  where
    S::Handlers: IsUnset,
    I: IntoIterator<Item = ServiceHandler>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let file = &mut arena.files[self.file_id];

      arena.services[self.id].handlers = handlers
        .into_iter()
        .inspect(|h| {
          file.conditionally_add_import(&h.request.file);
          file.conditionally_add_import(&h.response.file);
        })
        .collect();
    }

    ServiceBuilder {
      id: self.id,
      arena: self.arena,
      file_id: self.file_id,
      _phantom: PhantomData,
    }
  }

  pub fn options<I>(self, options: I) -> ServiceBuilder<SetOptions<S>>
  where
    S::Options: IsUnset,
    I: IntoIterator<Item = ProtoOption>,
  {
    {
      let mut arena = self.arena.borrow_mut();
      let service = &mut arena.services[self.id];

      service.options = options.into_iter().collect()
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
