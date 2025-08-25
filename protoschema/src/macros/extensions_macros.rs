/// A macro that creates an [`Extension`](crate::extensions::Extension) and adds it to a [`FileBuilder`](crate::files::FileBuilder).
/// The first argument is the ident for the [`FileBuilder`](crate::files::FileBuilder) where this extension will go.
/// The second argument is the ident of the [`MessageBuilder`](crate::message::MessageBuilder) representing the message being extended.
/// The fields for the extension are defined as a comma separated list of `$field_number:literal => $field:expr` surrounded by curly brackets, where $field evalutes to a [`FieldBuilder`](crate::fields::FieldBuilder) instance.
/// # Examples
/// ```
/// let package = Package::new("mypkg");
/// let file = package.new_file("myfile");
/// let mymsg = file.new_message("mymsg");
///
/// extension!(
///   file,
///   mymsg {
///     150 => string!("abc")
///   }
/// );
/// ```
#[macro_export]
macro_rules! extension {
  ($file:ident, $extendee:ident { $($fields:tt)* }) => {
    $file.add_extension($crate::extensions::Extension::builder()
      .import_path($extendee.get_import_path())
      .fields(
        $crate::parse_fields!(
          @included_fields()
          @fields()
          @rest($($fields)*)
        )
      )
      .build()
      )
  };
}
