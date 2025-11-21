use syn::{GenericArgument, PathArguments};

use crate::*;

pub fn extract_option(ty: &Path) -> Option<&Path> {
  let last_segment = ty.segments.last()?;

  if last_segment.ident == "Option" && let PathArguments::AngleBracketed(args) = &last_segment.arguments
    && let GenericArgument::Type(inner_ty) = args.args.first()? && let Type::Path(type_path) = inner_ty {
      return Some(&type_path.path);
    }

  None
}
