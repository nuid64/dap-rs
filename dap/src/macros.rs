/// Generates a Deserialize implementation for the give type name using the FromStr
/// for the give type.
#[macro_export]
macro_rules! fromstr_deser {
  ($e:tt) => {
    impl<'de> Deserialize<'de> for $e {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: Deserializer<'de>,
      {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
      }
    }
  }
}

#[macro_export]
macro_rules! tostr_ser {
  ($e:tt) => {
    impl Serialize for $e {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        serializer.serialize_str(&self.to_string())
      }
    }
  }
}

// Taken shamelessly from https://github.com/serde-rs/serde/issues/1560#issuecomment-506915291
#[macro_export]
macro_rules! named_unit_variant {
  ($variant:ident) => {
    #[allow(non_snake_case)]
    pub mod $variant {
      pub fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        serializer.serialize_str(stringify!($variant))
      }

      //   pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
      //   where
      //     D: serde::Deserializer<'de>,
      //   {
      //     struct V;
      //     impl<'de> serde::de::Visitor<'de> for V {
      //       type Value = ();
      //       fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      //         f.write_str(concat!("\"", stringify!($variant), "\""))
      //       }
      //       fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
      //         if value == stringify!($variant) {
      //           Ok(())
      //         } else {
      //           Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
      //         }
      //       }
      //     }
      //     deserializer.deserialize_str(V)
      //   }
    }
  };
}
