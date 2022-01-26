use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserialize, Serialize,
};

use crate::{ArcBytes, OwnedBytes};

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

impl Serialize for OwnedBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.as_slice())
    }
}

impl<'de> Deserialize<'de> for OwnedBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_byte_buf(BufferVisitor)
            .map(|bytes| match bytes {
                Cow::Borrowed(borrowed) => Self(ArcBytes::owned(borrowed.to_vec())),
                Cow::Owned(vec) => Self(ArcBytes::owned(vec)),
            })
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

impl<'a> From<Bytes> for ArcBytes<'a> {
    fn from(bytes: Bytes) -> Self {
        ArcBytes::owned(bytes.0)
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

/// A `Cow<'a, [u8]>` wrapper that supports serializing efficiently in Serde.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct CowBytes<'a>(pub Cow<'a, [u8]>);

impl<'a> CowBytes<'a> {
    /// Returns the underlying Cow.
    #[allow(clippy::missing_const_for_fn)] // false positive
    #[must_use]
    pub fn into_cow(self) -> Cow<'a, [u8]> {
        self.0
    }

    /// Returns the underlying Vec inside of the Cow, or clones the borrowed bytes into a new Vec..
    #[allow(clippy::missing_const_for_fn)] // false positive
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        match self.0 {
            Cow::Borrowed(bytes) => bytes.to_vec(),
            Cow::Owned(vec) => vec,
        }
    }
}

impl<'a> From<Bytes> for CowBytes<'a> {
    fn from(bytes: Bytes) -> Self {
        CowBytes(Cow::Owned(bytes.0))
    }
}

impl<'a> From<CowBytes<'a>> for Bytes {
    fn from(bytes: CowBytes<'a>) -> Self {
        match bytes.0 {
            Cow::Borrowed(bytes) => Self(bytes.to_vec()),
            Cow::Owned(vec) => Self(vec),
        }
    }
}

impl<'a> From<CowBytes<'a>> for ArcBytes<'a> {
    fn from(bytes: CowBytes<'a>) -> Self {
        ArcBytes::from(bytes.0)
    }
}

impl<'a> From<Vec<u8>> for CowBytes<'a> {
    fn from(buffer: Vec<u8>) -> Self {
        Self(Cow::Owned(buffer))
    }
}

impl<'a> From<&'a [u8]> for CowBytes<'a> {
    fn from(buffer: &'a [u8]) -> Self {
        Self(Cow::Borrowed(buffer))
    }
}

impl<'a> Deref for CowBytes<'a> {
    type Target = Cow<'a, [u8]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for CowBytes<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Serialize for CowBytes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for CowBytes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_byte_buf(BufferVisitor)
            .map(CowBytes)
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
fn serialization_tests() {
    use super::Bytes;

    let simple_buffer = vec![1_u8, 2, 3];
    let simple_buffer = simple_buffer.as_slice();
    let simple_arcbytes = ArcBytes::from(simple_buffer);

    // deserialize_seq
    let u8_sequence_bytes = pot::to_vec(&simple_buffer).unwrap();
    let buffer = pot::from_slice::<ArcBytes<'_>>(&u8_sequence_bytes).unwrap();
    assert_eq!(buffer, simple_buffer);
    assert!(matches!(buffer.buffer, Bytes::Owned(_)));

    // deserialize_borrowed_bytes
    let actual_bytes = pot::to_vec(&simple_arcbytes).unwrap();
    let buffer = pot::from_slice::<ArcBytes<'_>>(&actual_bytes).unwrap();
    assert_eq!(buffer, simple_buffer);
    assert!(matches!(buffer.buffer, Bytes::Borrowed(_)));

    // deserialize_byte_buf
    let json = serde_json::to_string(&simple_arcbytes).unwrap();
    let buffer = serde_json::from_str::<ArcBytes<'_>>(&json).unwrap();
    assert_eq!(buffer, simple_buffer);
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
    let actual_bytes = pot::to_vec(&simple_arcbytes).unwrap();
    let buffer = pot::from_slice::<self::Bytes>(&actual_bytes).unwrap();
    assert_eq!(buffer.as_slice(), simple_buffer);

    // Deserialize `CowBytes`
    let buffer = pot::from_slice::<self::CowBytes<'_>>(&actual_bytes).unwrap();
    assert_eq!(&buffer[..], simple_buffer);
    assert!(matches!(buffer.0, Cow::Borrowed(_)));
    let buffer = pot::from_slice::<self::CowBytes<'_>>(&u8_sequence_bytes).unwrap();
    assert_eq!(&buffer[..], simple_buffer);
    assert!(matches!(buffer.0, Cow::Owned(_)));
}
