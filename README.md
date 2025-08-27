# 📘 Protoschema

> 📐 Programmatically define protobuf contracts using flexible, modular and reusable elements

Protobuf has many strengths, like its schema-driven composition and language-agnostic reproduction of the objects defined in `.proto` files.
However, defining said files is often a boring and repetitive job, especially if you want to leverage custom options like those that libraries like [protocheck](https://github.com/Rick-Phoenix/protocheck) or [protovalidate-es](https://github.com/bufbuild/protovalidate-es) use to define some extra logic about your protobuf messages.

Wouldn't it be great if we could define protobuf contracts in a flexible, programmatic way, using modular, reusable building blocks?
<br/>
Wouldn't it be nice if we could define some common options, fields and other reusable parts like enum variants, and put them all together to create the complete proto contract structure?
<br/>
And wouldn't it be extra-great if we could also define validation for said contracts using the same syntax, in a very concise way, coupled with the benefits of type safety and autocomplete suggestions?

Well, this crate tries to address precisely all of that. 

# 🛠️ How it works

It all starts with the [`Package`] struct, which emulates a protobuf package. 
From this package, we can create new [`FileBuilder`](crate::files::FileBuilder)s, which in turn can be used to generate new [`MessageBuilder`](crate::messages::MessageBuilder)s and [`EnumBuilder`](crate::enums::EnumBuilder)s. 

There are macros and function to generate every item that you would see in a protobuf file.
Every macro comes with its own examples, so you can head over to the [`macros`] module to inspect the various detailed examples for each use case.

We are also going to cover the single items one by one in here with some elementary examples, with a complete example at the end.
But first, I want to focus on two aspects which are core elements about this crate, and its two main drivers, namely reusable elements and validators.

## 🧩 Reusable elements

Protoschema is designed to be as modular as possible. This means that you can define any sort of item which you might reuse in multiple places, such as one or many options, one or many fields (with options and even imports included in them), one or many enum variants and oneofs. This is how you would do it.

```rust
use protoschema::{reusable_fields, enum_variants, proto_enum, oneof, proto_option, timestamp, uint64, Package, message};

let my_reusable_option = proto_option("something_i_use", "very_very_often");
let my_list_of_options = [ my_reusable_option.clone(), my_reusable_option.clone() ];

let my_reusable_fields = reusable_fields!(
  1 => uint64!("id"),
  // This will automatically add google/protobuf/timestamp.proto to the receiving file
  2 => timestamp!("created_at"),
  3 => timestamp!("updated_at"),
  // This will add the import to the receiving file
  4 => uint64!("with_custom_option").add_option(my_reusable_option.clone()).add_import("my_other_package/v1/file.proto")
);

let my_reusable_variants = enum_variants!(
  0 => "UNSPECIFIED",
  1 => "SOME_COMMON_VARIANT"
);

// And now we can simply reuse these wherever we want. Let's start with a oneof.

let my_oneof = oneof!(
  "my_oneof",
  options = [ my_reusable_option.clone() ],
  include(my_reusable_fields),
);

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

let my_msg = my_file.new_message("MyMessage");

message!(
  my_msg,
  // This is an expression like any other, so any IntoIter<Item = ProtoOption> can work
  options = my_list_of_options.clone(),
  // Including all fields at once
  include(my_reusable_fields),
  5 => uint64!("internal_ref"),

  // Including a reusable oneof
  include_oneof(my_oneof),

  enum "my_nested_enum" {
    // Including a group of reusable variants
    include(my_reusable_variants),
  }
);

let my_other_enum = proto_enum!(
  my_file.new_enum("my_other_enum"),
  // Included blocks are cloned automatically behind the scenes
  include(my_reusable_variants)
);
```

## ✅ Easy validation rules definition

Protobuf as a whole has a great potential for becoming the standard in defining API contracts, especially between different languages, because by leveraging libraries such as [protocheck](https://github.com/Rick-Phoenix/protocheck) or [protovalidate-es](https://github.com/bufbuild/protovalidate-es), it can allow you to define the validation logic for your objects in the same schema where you are defining them. 

Compared to the traditional methods like JSON schema, this comes with the benefit of being able to apply custom [cel](https://cel.dev/) rules which can validate multiple fields in a struct at once, which is not possible with normal JSON schema validation.

However, there are a few limitations that come with this approach.

First, the syntax for doing so is a bit long and repetitive. Doing 

```proto
string my_field = 1 [(buf.validate.field).string = { min_len: 10 }];
map<uint64, string> my_map = 2 [(buf.validate.field).map = { min_pairs: 5, keys: { uint64: { gt: 15 } }, values: { string: { min_len: 10, email: true } } }];
```

For every field is a tad bit too slow for my taste.

But most importantly, writing rules like this means lacking the best features that come from modern day development such as type safety and LSP autocompletion.

And lastly, this is not easy to replicate. If you want to have the same rules for 20, 100, or 500 different fields, you need to manually copy-paste the options everywhere or come up with some custom plugin solution.

These are three issues that this crate tries to fully resolve.

Every field macro such as [`string`] or [`uint64`] comes with an additional last argument, which is a closure that will receive a validator instance such as [`StringValidator`](crate::validators::string::StringValidator), which uses the builder pattern to make definining validation rules not only much quicker, but with the added benefits of type safety and LSP autocompletion, which is something that you just wouldn't get if you were defining them directly in a `.proto` file. 

So the above essentially translates to this (the map field is explained separately below): 

```rust
use protoschema::{string};

let my_string = string!("my_field", |s| s.min_len(10));
```

By typing `s.min_len`, not only I get the specific validation methods for strings being suggested by the lsp, but it also makes it not possible to define the same rule twice, and to add to that, I have also added some safeguards (which would be redundant in case you use [protocheck](https://github.com/Rick-Phoenix/protocheck) as that implements all of them + some extra ones too) that show an error in case you set min_len to be greater than max_len, for example.

And to complement all that, since this is a single, reusable field, it can be reused as many times as you want by using the patterns described above.

As a nice bonus, if a field is a repeated or map field, the validator closure will provide the validators for the field itself and for its items/pairs.

```rust
use protoschema::{map, string};

let my_string = string!(repeated "my_field", |list, string| 
  // First we define rules for the list as a whole, such as the minimum items required    
  list.min_items(5)
  // Then we define rules for the individual items. Once again, here the validator builder will match the type of the field
  .items(
    string.min_len(10)
  )
);

let my_map = map!("my_map", <uint64, string>, |map, keys, values|
  // Map-level rules
  map.min_pairs(5)
  // Rules for keys. Notice they are uint-specific methods
  .keys(
    keys.gt(15)
  )
  // Rules for values
  .values(
    values.min_len(10)
      // Strings have all the supported well known string rules such as email, ip address and so on
      .email()
  )
);
```

## 📦 Define a package and a file

> **Note**: The package path and the .proto suffix are automatically added to file names.
> So in the example below, the full path to the file from the root of the proto project will be `my_pkg/v1/my_file.proto`

> **Tip**: In order to avoid rebuilding the results needlessly, this should ideally be done in a separate crate, from which you will directly use [prost-build](https://crates.io/crates/prost-build) to compile the newly-generated proto files, which you can then import from the consuming applications.

```rust
use protoschema::{Package};

let my_pkg = Package::new("my_pkg.v1");
// .proto is added automatically as a suffix
let my_file = my_pkg.new_file("my_file");
```

## 📩 Define a new message (simple version)

This is how you create the [`MessageBuilder`](crate::messages::MessageBuilder), which is the first argument that you give to the [`message`] macro and also allows you can define nested messages.

For a full, comprehensive example on how to populate a message using the [`message`] macro, check out the [tests](https://github.com/Rick-Phoenix/protoschema/tree/main/test) crate or the [`render_templates`](crate::packages::Package::render_templates) description.

```rust
use protoschema::{Package};

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

// This will be defined at the top level of the file
let my_msg = my_file.new_message("MyMessage");

// This will be defined inside MyMessage
let my_nested_message = my_msg.new_message("MyNestedMsg");
```

## 🔢 Define a new enum

There are two ways to define an enum.

One is to create it as its separate builder, and the other is to define it as part of the [`message`] macro, if the enum is supposed to be defined inside a message.

To define an enum at the top level, you first have to create an [`EnumBuilder`](crate::enums::EnumBuilder) like this:

```rust
use protoschema::{Package};

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

let my_enum = my_file.new_enum("my_enum");

let my_msg = my_file.new_message("MyMessage");
// This will be defined inside MyMessage. This can also be done directly inside the message! macro
let my_nested_enum = my_msg.new_enum("my_nested_enum");
```

Then, you can populate it with the [`proto_enum`] macro, where you can define options, reserved names/numbers, variants, and also include reusable variants defined with the [`enum_variants`] macro.
<br/>

**Note**: You do not need to add the enum name as a prefix to the variants. It will be added automatically.
So if an enum is named "my_enum", and the variant is "UNSPECIFIED", the output will show "MY_ENUM_UNSPECIFIED".

```rust
use protoschema::{Package, enum_variants, proto_enum, proto_option, common::allow_alias};

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

let my_opt = proto_option("cats_are_cute", true);

let my_enum = my_file.new_enum("my_enum");
let reusable_variants = enum_variants!(
  0 => "UNSPECIFIED"
);

let my_enum = proto_enum!(
  my_enum,
  // Common options such as allow_alias have helpers for them
  options = [ my_opt.clone(), allow_alias() ],
  // Including reusable variants as a group
  include(reusable_variants),
  // Options for enum values are defined like this
  1 => "PASSIVE" { [ my_opt.clone() ] },
  1 => "INACTIVE",
  2 => "ACTIVE"
);
```

Alternatively, you can define it directly inside the [`message`] macro, using the same syntax as the [`proto_enum`] macro:

```rust
use protoschema::{Package, message, string};

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

let my_msg = my_file.new_message("MyMessage");

message!(
  my_msg,
  1 => string!("my_field"),

  enum "my_enum" {
    0 => "UNSPECIFIED"
  }
);
```

## 1️⃣ Define a oneof

Just like enums, oneofs can be defined within the [`message`] macro, or on their own, using the [`oneof`] macro.

```rust
use protoschema::{reusable_fields, oneof, proto_option, string, Package, message};

let my_reusable_option = proto_option("something_i_use", "very_very_often");
let my_list_of_options = [ my_reusable_option.clone(), my_reusable_option.clone() ];

let my_reusable_fields = reusable_fields!(
  1 => string!("email"),
  2 => string!("nickname"),
);

// Defining the oneof individually
let my_oneof = oneof!(
  "my_oneof",
  options = [ my_reusable_option.clone() ],
  // Fields can be included as a block
  include(my_reusable_fields),
  // Or individually
  3 => string!("id")
);

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

let my_msg = my_file.new_message("MyMessage");

// Or directly as part of a message, using the same syntax
message!(
  my_msg,
  4 => string!("my_field"),

  oneof "my_oneof" {
    include(my_reusable_fields),
    3 => string!("id")
  }
);
```

## ⚙️ Define services

Macro: [`services`](crate::services!)

```rust
use protoschema::{Package, services, proto_option};

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

let handler_request = my_file.new_message("HandlerRequest");
let handler_response = my_file.new_message("HandlerResponse");

let my_opt = proto_option("true_is_true", true);
let my_list_of_options = [ my_opt.clone(), my_opt.clone() ];

services!(
  my_file,
  // It accepts any IntoIter<Item = ProtoOption>
  MyService {
    options = my_list_of_options.clone(),
    MyHandler(handler_request => handler_response) { [ my_opt.clone() ] },
    MyOtherHandler(handler_request => handler_response)
  };

  MyOtherService {
    MyHandler(handler_request => handler_response),
    MyOtherHandler(handler_request => handler_response)
  };
);
```

## 🔗 Define extensions

The second argument to [`extension`](crate::extension!) is any plain ident that corresponds to a member of the [`ExtensionKind`](crate::extensions::ExtensionKind) enum, such as FieldOptions, MessageOptions, etc.

```rust
use protoschema::{Package, proto_option, extension, string};

let my_pkg = Package::new("my_pkg.v1");
let my_file = my_pkg.new_file("my_file");

let my_opt = proto_option("true_is_true", true);
let my_list_of_options = [ my_opt.clone(), my_opt.clone() ];

extension!(
  my_file,
  MessageOptions {
    1559 => string!("my_extension_field")
  }
);
```

## 📝 How to render the files

After all of your items are defined, you just need to call [`render_templates`](crate::packages::Package::render_templates) with the path to the root of your proto project, and all the files will be written inside of it.

```rust
use protoschema::{Package};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let package = Package::new("mypkg.v1"); 
  let proto_root = Path::new("proto");
  package.render_templates(proto_root)?;
  Ok(())
}
```

## Complete example

Check out the [tests](https://github.com/Rick-Phoenix/protoschema/tree/main/test) crate or the [`render_templates`](crate::packages::Package::render_templates) description for full usage example, with the proto output included.

