use std::collections::HashSet;

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
}

pub(crate) struct ModuleItem<'a> {
  pub kind: ItemKind<'a>,
  pub is_nested: bool,
}

impl<'a> ToTokens for ModuleItem<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let ident = self.get_ident();

    let content = match &self.kind {
      ItemKind::Message(_) => quote! { #ident::to_message() },
      ItemKind::Enum(_) => quote! { #ident::to_enum() },
    };

    tokens.extend(content);
  }
}

impl<'a> ModuleItem<'a> {
  pub fn is_top_level(&self) -> bool {
    !self.is_nested
  }

  pub fn inject_attr(&mut self, attr: Attribute) {
    match &mut self.kind {
      ItemKind::Message(item) => item.attrs.push(attr),
      ItemKind::Enum(item) => item.attrs.push(attr),
    }
  }

  pub fn get_ident(&self) -> &Ident {
    match &self.kind {
      ItemKind::Message(item) => &item.ident,
      ItemKind::Enum(item) => &item.ident,
    }
  }
}

pub fn process_module_items(
  file_attribute: Attribute,
  items: &'_ mut Vec<Item>,
) -> Result<TopLevelItemsTokens, Error> {
  let mut processed_items: Vec<ModuleItem> = Vec::new();
  let mut nested_items: HashMap<Ident, Ident> = HashMap::new();

  for item in items {
    match item {
      Item::Struct(s) => {
        for attr in &s.attrs {
          if attr.path().is_ident("proto") && let Ok(list) = attr.meta.require_list()  {
            let metas = list.parse_args::<PunctuatedParser<Meta>>().unwrap().inner;

            for meta in metas {
              if meta.path().is_ident("nested_messages") {
                let nested_messages_list = meta.require_list().unwrap().parse_args::<PunctuatedParser<Path>>().unwrap().inner;

                for msg in nested_messages_list {
                  let nested_msg_ident = msg.require_ident()?;

                  nested_items.insert(nested_msg_ident.clone(), s.ident.clone());
                }
              }
            }
          }
        }

        processed_items.push(ModuleItem {
          kind: ItemKind::Message(s),
          is_nested: false,
        })
      }
      Item::Enum(e) => processed_items.push(ModuleItem {
        kind: ItemKind::Enum(e),
        is_nested: false,
      }),
      _ => {}
    }
  }

  let mut top_level_messages = TokenStream2::new();
  let mut top_level_enums = TokenStream2::new();

  for item in processed_items.iter_mut() {
    item.inject_attr(file_attribute.clone());

    if let Some(parent_message_ident) = nested_items.get(item.get_ident()) {
      let parent_message_attr: Attribute =
        parse_quote! { #[proto(parent_message = #parent_message_ident)] };

      item.inject_attr(parent_message_attr);
    } else {
      let item_ident = item.get_ident();

      match item.kind {
        ItemKind::Message(_) => top_level_messages.extend(quote! { #item_ident::to_message(), }),
        ItemKind::Enum(_) => top_level_enums.extend(quote! { #item_ident::to_enum() }),
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

pub fn get_derives(meta: &Meta) -> Result<PunctuatedParser<Path>, Error> {
  let list = meta.require_list()?;

  let derives = list.parse_args::<PunctuatedParser<Path>>()?;

  Ok(derives)
}
