//! Simple `Blob` structure which can store binary data and serialize/deserialize to/from base-64 encoded strings

use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use base64;
use serde;

use ::error::ProtocolResult;

/// `Blob` is a simple wrapper around a `Vec<u8>` that serializes to a base-64 string
/// instead of a massive sequence of integers.
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Blob(Vec<u8>);

impl Blob {
    /// Encode the blob to a base-64 string
    pub fn encode_base64(&self) -> String {
        base64::encode(&self.0)
    }

    /// Decode a base-64 encoded string into a binary blob
    pub fn decode_base64(encoded: &str) -> ProtocolResult<Blob> {
        let data = try_throw!(base64::decode(encoded));

        Ok(Blob(data))
    }
}

impl FromStr for Blob {
    type Err = base64::Base64Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Blob(base64::decode(s)?))
    }
}

impl serde::Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let encoded = self.encode_base64();

        serializer.serialize_str(encoded.as_str())
    }
}

impl serde::Deserialize for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer {
        struct BlobVisitor;

        impl serde::de::Visitor for BlobVisitor {
            type Value = Blob;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("string or sequence")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(FromStr::from_str(value).unwrap())
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(Blob::from(value))
            }

            fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(Blob(value))
            }
        }

        deserializer.deserialize(BlobVisitor)
    }
}

impl Deref for Blob {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Blob where T: Into<Vec<u8>> {
    fn from(value: T) -> Blob {
        Blob(value.into())
    }
}

#[cfg(test)]
mod test {
    use super::Blob;

    const DATA: [u8; 5] = [1, 2, 3, 4, 5];

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct BlobFixture {
        my_blob: Blob,
    }

    #[test]
    fn test_blob() {
        let blob = Blob::from(&DATA[..]);

        let encoded = blob.encode_base64();

        let decoded_blob = Blob::decode_base64(encoded.as_str()).unwrap();

        assert_eq!(blob, decoded_blob);
    }

    #[test]
    fn test_blob_serde() {
        use serde_json::{to_string_pretty, from_str};

        let fixture = BlobFixture { my_blob: Blob::from(&DATA[..]) };

        let encoded = to_string_pretty(&fixture).unwrap();

        println!("Encoded {}", encoded);

        let decoded = from_str(encoded.as_str()).unwrap();

        assert_eq!(fixture, decoded);
    }
}