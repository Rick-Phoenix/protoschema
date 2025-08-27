fn main() {}

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
  pub mod v2 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v2.rs"));
  }
}

#[cfg(test)]
mod tests {
  use protocheck::types::protovalidate::Violations;

  use crate::myapp::v1::User;

  #[test]
  fn validators_test() {
    let user = User {
      password: "abcde".to_string(),
      repeated_password: "abc".to_string(),
      last_status: 25,
      ..Default::default()
    };

    #[allow(unused_variables)]
    let Violations { violations } = user.validate().unwrap_err();

    // println!("{:#?}", violations);
  }
}
