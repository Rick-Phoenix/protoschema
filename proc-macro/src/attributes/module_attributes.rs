use std::{collections::HashSet, fmt::Write, rc::Rc};

use syn::{ItemEnum, ItemStruct, MetaNameValue};

use crate::*;

pub(crate) struct ModuleAttrs {
  pub file: String,
  pub package: String,
}

pub(crate) struct TopLevelItemsTokens {
  pub top_level_messages: TokenStream2,
  pub top_level_enums: TokenStream2,
}

pub(crate) enum ItemKind<'a> {
  Message(&'a mut ItemStruct),
  Enum(&'a mut ItemEnum),
  Oneof(&'a mut ItemEnum),
}

pub(crate) struct ModuleItem<'a> {
  pub kind: ItemKind<'a>,
  pub name: Rc<str>,
}

impl<'a> ModuleItem<'a> {
  pub fn inject_attr(&mut self, attr: Attribute) {
    match &mut self.kind {
      ItemKind::Message(item) => item.attrs.push(attr),
      ItemKind::Enum(item) => item.attrs.push(attr),
      ItemKind::Oneof(item) => item.attrs.push(attr),
    }
  }

  pub fn get_ident(&self) -> &Ident {
    match &self.kind {
      ItemKind::Message(item) => &item.ident,
      ItemKind::Enum(item) => &item.ident,
      ItemKind::Oneof(item) => &item.ident,
    }
  }
}

pub(crate) enum DeriveKind {
  Message,
  Enum,
  Oneof,
}

pub(crate) struct ParentMessage {
  pub ident: Ident,
  pub name: Rc<str>,
}

pub fn process_module_items(
  file_attribute: Attribute,
  items: &'_ mut Vec<Item>,
) -> Result<TopLevelItemsTokens, Error> {
  let mut processed_items: Vec<ModuleItem> = Vec::new();
  let mut nested_items_map: HashMap<Ident, ParentMessage> = HashMap::new();

  for item in items {
    let derive_kind = if let Some(kind) = get_derive_kind(item)? {
      kind
    } else {
      continue;
    };

    match item {
      Item::Struct(s) => {
        let mut name: Option<String> = None;
        let mut nested_items_list: Option<PunctuatedParser<Path>> = None;

        for attr in &s.attrs {
          if attr.path().is_ident("proto") {
            let metas = attr.parse_args::<PunctuatedParser<Meta>>().unwrap().inner;

            for meta in metas {
              match meta {
                Meta::List(list) => {
                  if list.path.is_ident("nested_messages") {
                    nested_items_list = Some(list.parse_args::<PunctuatedParser<Path>>()?);
                  }
                }
                Meta::NameValue(nv) => {
                  if nv.path.is_ident("name") {
                    name = Some(extract_string_lit(&nv.value)?);
                  }
                }
                _ => {}
              }
            }
          }
        }

        if !matches!(derive_kind, DeriveKind::Message) {
          panic!("The Message derive can only be used on structs");
        }

        let name: Rc<str> = if let Some(name_override) = name {
          name_override.into()
        } else {
          let inferred_name = s.ident.to_string();

          let name_attr: Attribute = parse_quote! { #[proto(name = #inferred_name)] };
          s.attrs.push(name_attr);

          inferred_name.into()
        };

        if let Some(nested_items_list) = nested_items_list {
          for nested_item in nested_items_list.inner {
            let nested_item_ident = nested_item.require_ident()?;

            nested_items_map.insert(
              nested_item_ident.clone(),
              ParentMessage {
                ident: s.ident.clone(),
                name: name.clone(),
              },
            );
          }
        }

        processed_items.push(ModuleItem {
          name,
          kind: ItemKind::Message(s),
        })
      }
      Item::Enum(e) => {
        let mut name: Option<String> = None;

        for attr in &e.attrs {
          if attr.path().is_ident("proto") {
            let metas = attr.parse_args::<PunctuatedParser<Meta>>().unwrap().inner;

            for meta in metas {
              if let Meta::NameValue(nv) = meta
                && nv.path.is_ident("name") {
                  name = Some(extract_string_lit(&nv.value)?);
                }
            }
          }
        }

        let name: Rc<str> = if let Some(name_override) = name {
          name_override.into()
        } else {
          let inferred_name = e.ident.to_string();

          let name_attr: Attribute = parse_quote! { #[proto(name = #inferred_name)] };
          e.attrs.push(name_attr);

          inferred_name.into()
        };

        match derive_kind {
          DeriveKind::Enum => processed_items.push(ModuleItem {
            name,
            kind: ItemKind::Enum(e),
          }),
          DeriveKind::Oneof => processed_items.push(ModuleItem {
            name,
            kind: ItemKind::Oneof(e),
          }),
          DeriveKind::Message => panic!("Cannot use the Message derive on an enum"),
        };
      }
      _ => {}
    }
  }

  let mut top_level_messages = TokenStream2::new();
  let mut top_level_enums = TokenStream2::new();

  for item in processed_items.iter_mut() {
    item.inject_attr(file_attribute.clone());

    if let Some(parent_message) = nested_items_map.get(item.get_ident()) {
      let parent_message_ident = &parent_message.ident;

      let mut ancestors = vec![parent_message];
      let mut current_message = parent_message_ident;

      while let Some(parent) = nested_items_map.get(current_message) {
        ancestors.push(parent);
        current_message = &parent.ident;
      }

      let mut full_name = String::new();

      for ancestor in ancestors.iter().rev() {
        let ancestor_name = &ancestor.name;
        write!(full_name, "{ancestor_name}.").unwrap();
      }

      full_name.push_str(&item.name);

      let full_name_attr: Attribute = parse_quote! { #[proto(full_name = #full_name)] };

      item.inject_attr(full_name_attr);

      let parent_message_attr: Attribute =
        parse_quote! { #[proto(parent_message = #parent_message_ident)] };

      item.inject_attr(parent_message_attr);
    } else if !matches!(item.kind, ItemKind::Oneof(_)) {
      let item_ident = item.get_ident();

      match item.kind {
        ItemKind::Message(_) => top_level_messages.extend(quote! { #item_ident::to_message(), }),
        ItemKind::Enum(_) => top_level_enums.extend(quote! { #item_ident::to_enum() }),
        _ => {}
      }
    }
  }

  Ok(TopLevelItemsTokens {
    top_level_messages,
    top_level_enums,
  })
}

impl Parse for ModuleAttrs {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut file: Option<String> = None;
    let mut package: Option<String> = None;

    let args = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

    for arg in args {
      if arg.path.is_ident("file") {
        file = Some(extract_string_lit(&arg.value)?);
      } else if arg.path.is_ident("package") {
        package = Some(extract_string_lit(&arg.value)?);
      }
    }

    let file = file.ok_or(error!(Span::call_site(), "File attribute is missing"))?;
    let package = package.ok_or(error!(Span::call_site(), "Package attribute is missing"))?;

    Ok(ModuleAttrs { file, package })
  }
}

pub fn get_derive_kind(item: &Item) -> Result<Option<DeriveKind>, Error> {
  let attrs = match item {
    Item::Struct(s) => &s.attrs,
    Item::Enum(e) => &e.attrs,
    _ => return Ok(None),
  };

  for attr in attrs {
    if attr.path().is_ident("derive") {
      let derives = attr
        .meta
        .require_list()?
        .parse_args::<PunctuatedParser<Path>>()?
        .inner;

      for path in derives {
        if path.is_ident("Message") {
          return Ok(Some(DeriveKind::Message));
        } else if path.is_ident("Enum") {
          return Ok(Some(DeriveKind::Enum));
        } else if path.is_ident("Oneof") {
          return Ok(Some(DeriveKind::Oneof));
        }
      }

      return Ok(None);
    }
  }

  Ok(None)
}
