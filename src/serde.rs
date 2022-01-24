use std::fmt;

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

impl<'de> Deserialize<'de> for ArcBytes<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(BufferVisitor)
    }
}

struct BufferVisitor;

impl<'de> Visitor<'de> for BufferVisitor {
    type Value = ArcBytes<'de>;

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

        Ok(ArcBytes::from(bytes))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(ArcBytes::from(v.to_vec()))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(ArcBytes::from(v))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(ArcBytes::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(ArcBytes::from(v.as_bytes().to_vec()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(ArcBytes::from(v))
    }
}
