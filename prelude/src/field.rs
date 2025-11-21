use crate::*;

#[derive(Debug, Clone)]
pub struct ProtoField {
  pub name: String,
  pub tag: u32,
  pub type_: ProtoType,
  pub options: Vec<ProtoOption>,
  pub validator: Option<ProtoOption>,
}

impl ProtoField {
  pub fn register_type_import_path(&self, imports: &mut HashSet<Arc<str>>) {
    match &self.type_ {
      ProtoType::Single(ty) => ty.register_import(imports),
      ProtoType::Repeated(ty) => ty.register_import(imports),
      ProtoType::Map { keys, values } => {
        keys.register_import(imports);
        values.register_import(imports);
      }
    };
  }
}
