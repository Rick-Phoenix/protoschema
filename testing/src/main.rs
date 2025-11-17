use prelude::{EnumVariant, Message, Oneof, ProtoEnum, ProtoField, ProtoFile, ProtoOption};
use proc_macro_impls::{Enum, Message, Oneof};

pub trait ProtoType {
  fn proto_type() -> &'static str;
}

#[derive(Oneof)]
enum PseudoOneof {
  A(String),
  B(u32),
}

#[derive(Enum)]
enum Bcd {
  AbcDeg,
  B,
  C,
}

#[derive(Message)]
struct Abc {
  #[proto(validate = |v| v.min_len(25))]
  name: String,
}

fn main() {
  let mut file = ProtoFile::new("abc.proto", "myapp.v1");

  let msg = Abc::to_message(&mut file);

  let msg2 = msg.clone();
  let nested = msg.nested_message(msg2);

  let nested_enum = Bcd::to_nested_enum(nested);

  let oneof = PseudoOneof::to_oneof(nested);
}
