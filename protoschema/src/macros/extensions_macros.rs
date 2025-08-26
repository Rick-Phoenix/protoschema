/// A macro that creates an [`Extension`](crate::extensions::Extension) and adds it to a [`FileBuilder`](crate::files::FileBuilder).
///
/// The first argument is the ident for the [`FileBuilder`](crate::files::FileBuilder) where this extension will go.
/// The second argument is an ident that will be matched with a variant of [`ExtensionKind`](crate::extensions::ExtensionKind) enum (i.e. MessageOptions, FileOptions, etc).
/// The fields for the extension are defined as a comma separated list of `$field_number:literal => $field:expr` surrounded by curly brackets, where $field evalutes to a [`FieldBuilder`](crate::fields::FieldBuilder) instance.
/// Options for the fields themselves can be defined by calling [`add_option`](crate::fields::FieldBuilder::add_option) on the field builder.
/// # Examples
/// ```
/// use protoschema::{Package, extension, string, reusable_fields};
///
/// let package = Package::new("mypkg");
/// let file = package.new_file("myfile");
///
/// let shared_fields = reusable_fields!(
///   151 => string!("metadata_1"),
///   152 => string!("metadata_2"),
/// );
///
/// extension!(
///   file,
///   MessageOptions {
///     // Add a normal field
///     150 => string!("abc"),
///     // Include reusable fields
///     include(shared_fields)
///   }
/// );
/// ```
#[macro_export]
macro_rules! extension {
  ($file:ident, $extendee:ident { $($fields:tt)* }) => {
    $crate::paste! {
      $file.add_extension($crate::extensions::Extension::builder()
      .fields(
        $crate::parse_fields!(
          @included_fields()
          @fields()
          @rest($($fields)*)
        )
      )
      .kind($crate::extensions::ExtensionKind::[< $extendee >])
      .build()
      )

    }
  };
}
