use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserialize, Serialize,
};

use crate::ArcBytes;

impl<'a> Serialize for ArcBytes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_slice())
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for ArcBytes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_bytes(BufferVisitor)
            .map(Self::from)
    }
}

/// A `Vec<u8>` wrapper that supports serializing efficiently in Serde.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    /// Returns the underlying Vec.
    #[allow(clippy::missing_const_for_fn)] // false positive
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }
}

impl<'a> From<&'a [u8]> for Bytes {
    fn from(buffer: &'a [u8]) -> Self {
        Self(buffer.to_vec())
    }
}

impl<'a> From<ArcBytes<'a>> for Bytes {
    fn from(buffer: ArcBytes<'a>) -> Self {
        Self(buffer.into_vec())
    }
}

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.as_slice())
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_byte_buf(BufferVisitor)
            .map(|bytes| match bytes {
                Cow::Borrowed(borrowed) => Self(borrowed.to_vec()),
                Cow::Owned(vec) => Self(vec),
            })
    }
}

struct BufferVisitor;

impl<'de> Visitor<'de> for BufferVisitor {
    type Value = Cow<'de, [u8]>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bytes")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut bytes = if let Some(len) = visitor.size_hint() {
            Vec::with_capacity(len)
        } else {
            Vec::default()
        };

        while let Some(b) = visitor.next_element()? {
            bytes.push(b);
        }

        Ok(Cow::Owned(bytes))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Cow::Owned(v.to_vec()))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Cow::Borrowed(v))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Cow::Owned(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Cow::Owned(v.as_bytes().to_vec()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Cow::Borrowed(v.as_bytes()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Cow::Owned(v.into_bytes()))
    }
}

#[test]
fn tests() {
    use super::Bytes;

    // deserialize_seq
    let u8_sequence_bytes = pot::to_vec(&vec![1_u8, 2, 3]).unwrap();
    let buffer = pot::from_slice::<ArcBytes<'_>>(&u8_sequence_bytes).unwrap();
    assert_eq!(buffer, &[1_u8, 2, 3]);
    assert!(matches!(buffer.buffer, Bytes::Owned(_)));

    // deserialize_borrowed_bytes
    let actual_bytes = pot::to_vec(&ArcBytes::from(vec![1_u8, 2, 3])).unwrap();
    let buffer = pot::from_slice::<ArcBytes<'_>>(&actual_bytes).unwrap();
    assert_eq!(buffer, &[1_u8, 2, 3]);
    assert!(matches!(buffer.buffer, Bytes::Borrowed(_)));

    // deserialize_byte_buf
    let json = serde_json::to_string(&ArcBytes::from(vec![1_u8, 2, 3])).unwrap();
    let buffer = serde_json::from_str::<ArcBytes<'_>>(&json).unwrap();
    assert_eq!(buffer, &[1_u8, 2, 3]);
    assert!(matches!(buffer.buffer, Bytes::Owned(_)));

    // deserialize_str
    let str_bytes = pot::to_vec(&"hello").unwrap();
    let buffer = pot::from_slice::<ArcBytes<'_>>(&str_bytes).unwrap();
    assert_eq!(buffer, b"hello");
    assert!(matches!(buffer.buffer, Bytes::Borrowed(_)));

    // deserialize_string
    let buffer = serde_json::from_str::<ArcBytes<'_>>(r#""hello\u0020world""#).unwrap();
    assert_eq!(buffer, b"hello world");
    assert!(matches!(buffer.buffer, Bytes::Owned(_)));

    // Deserialize `Bytes`
    let actual_bytes = pot::to_vec(&ArcBytes::from(vec![1_u8, 2, 3])).unwrap();
    let buffer = pot::from_slice::<self::Bytes>(&actual_bytes).unwrap();
    assert_eq!(buffer.as_slice(), &[1_u8, 2, 3]);
}
