macro_rules! reusable_string {
  ($name:ident) => {
    $crate::paste! {
      pub(crate) static $name: std::sync::LazyLock<std::sync::Arc<str>> =
      std::sync::LazyLock::new(|| stringify!([< $name:lower >]).into());
    }
  };

  ($name:ident, $string:literal) => {
    pub(crate) static $name: std::sync::LazyLock<std::sync::Arc<str>> =
      std::sync::LazyLock::new(|| $string.into());
  };
}

macro_rules! impl_validator {
  ($validator:ident, $rust_type:ty, with_lifetime) => {
    $crate::paste! {
      impl<'a, S: [< $validator:snake _builder >]::IsComplete> From<[< $validator Builder >]<'a, S>> for ProtoOption {
        #[track_caller]
        fn from(value: [< $validator Builder >]<'a, S>) -> ProtoOption {
          let validator = value.build();

          validator.into()
        }
      }

      impl ProtoValidator<$rust_type> for ValidatorMap {
        type Builder<'a> = [< $validator Builder >]<'a>;

        fn builder() -> Self::Builder<'static> {
          $validator::builder()
        }
      }
    }
  };

  ($validator:ident, $rust_type:ty) => {
    $crate::paste! {
      impl<S: [< $validator:snake _builder >]::IsComplete> From<[< $validator Builder >]<S>> for ProtoOption {
        #[track_caller]
        fn from(value: [< $validator Builder >]<S>) -> ProtoOption {
          let validator = value.build();

          validator.into()
        }
      }

      impl ProtoValidator<$rust_type> for ValidatorMap {
        type Builder<'a> = [< $validator Builder >];

        fn builder() -> Self::Builder<'static> {
          $validator::builder()
        }
      }
    }
  };
}
