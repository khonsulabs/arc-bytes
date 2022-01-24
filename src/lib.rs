#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms,
)]
#![cfg_attr(doc, deny(rustdoc::all))]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
)]

use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{Debug, Write},
    io::{self, ErrorKind, Read},
    ops::{Bound, Deref, RangeBounds},
    sync::Arc,
};

/// An immutable buffer of bytes that can be cloned, sliced, and read into
/// multiple parts using a single refernce to the underlying buffer.
///
/// The read operations do not mutate the buffer when shortening the `self`
/// instance. Instead, the position is tracked within the original source buffer.
#[derive(Clone)]
pub struct ArcBytes<'a> {
    buffer: Option<Arc<Cow<'a, [u8]>>>,
    end: usize,
    position: usize,
}

impl<'a> Default for ArcBytes<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn default_is_new() {
    assert_eq!(ArcBytes::new(), ArcBytes::default());
}

impl<'a> Debug for ArcBytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut slice = self.as_slice();
        write!(f, "ArcBytes {{ length: {}, bytes: [", slice.len())?;
        while !slice.is_empty() {
            let (chunk, remaining) = slice.split_at(4.min(slice.len()));
            slice = remaining;
            for byte in chunk {
                write!(f, "{:02x}", byte)?;
            }
            if !slice.is_empty() {
                f.write_char(' ')?;
            }
        }
        f.write_str("] }")
    }
}

#[test]
fn debug_fmt() {
    let test = ArcBytes::borrowed(b"\x01\x23\x45\x67\x89");
    assert_eq!(
        format!("{:?}", test),
        "ArcBytes { length: 5, bytes: [01234567 89] }"
    );
}

// TODO add string conversions

impl<'a> Eq for ArcBytes<'a> {}

impl<'a, 'b> PartialEq<ArcBytes<'b>> for ArcBytes<'a> {
    fn eq(&self, other: &ArcBytes<'b>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> PartialEq<[u8]> for ArcBytes<'a> {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice().cmp(other) == Ordering::Equal
    }
}

impl<'a, 'b, const N: usize> PartialEq<&'b [u8; N]> for ArcBytes<'a> {
    fn eq(&self, other: &&'b [u8; N]) -> bool {
        self.as_slice().cmp(*other) == Ordering::Equal
    }
}

impl<'a> Ord for ArcBytes<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.buffer.as_ref(), other.buffer.as_ref()) {
            (Some(a), Some(b))
                if Arc::ptr_eq(a, b)
                    && self.position == other.position
                    && self.end == other.end =>
            {
                Ordering::Equal
            }
            _ => (&**self).cmp(&**other),
        }
    }
}

#[test]
fn ord_tests() {
    // Test simple comparisons with two separate allocated arcs
    assert_eq!(ArcBytes::borrowed(b"eq"), ArcBytes::borrowed(b"eq"));
    assert!(ArcBytes::borrowed(b"hello") < ArcBytes::borrowed(b"world"));
    // Test using the same underlying arc.
    let buffer = ArcBytes::borrowed(b"eq");
    let mut buffer_clone = buffer.clone();
    assert_eq!(buffer_clone, buffer);
    buffer_clone.next();
    assert_ne!(buffer_clone, buffer);
    assert!(buffer_clone > buffer);
}

impl<'a, 'b> PartialOrd<ArcBytes<'b>> for ArcBytes<'a> {
    fn partial_cmp(&self, other: &ArcBytes<'b>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> ArcBytes<'a> {
    /// Returns an empty buffer.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// assert!(ArcBytes::new().is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buffer: None,
            end: 0,
            position: 0,
        }
    }

    /// Returns a buffer with the owned bytes.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// assert_eq!(ArcBytes::owned(b"hello".to_vec()), b"hello");
    /// ```
    #[must_use]
    pub fn owned(buffer: Vec<u8>) -> Self {
        Self::from(Cow::Owned(buffer))
    }

    /// Returns a borrowed buffer.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// assert_eq!(ArcBytes::borrowed(b"hello"), b"hello");
    /// ```
    #[must_use]
    pub fn borrowed(buffer: &'a [u8]) -> Self {
        Self::from(Cow::Borrowed(buffer))
    }

    /// Converts this buffer into its slice and returns a static-lifetimed
    /// instance.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// assert_eq!(ArcBytes::borrowed(b"hello").to_owned(), b"hello");
    /// ```
    #[must_use]
    pub fn to_owned(&self) -> ArcBytes<'static> {
        ArcBytes::from(self.as_slice().to_vec())
    }

    /// Returns this buffer as a slice of `u8`s.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// assert_eq!(ArcBytes::borrowed(b"hello").as_slice(), b"hello");
    /// ```
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        if let Some(buffer) = &self.buffer {
            &buffer[self.position..self.end]
        } else {
            &[]
        }
    }

    /// Returns a slice of this buffer as its own `ArcBytes` instance. This
    /// performs no allocations, and instead references the original buffer.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// let original = ArcBytes::borrowed(b"abc");
    /// let b = original.slice(1..=1);
    /// assert_eq!(b, b"b");
    /// ```
    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> Self {
        let start = self.position.saturating_add(match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(start) => start.saturating_add(1),
            Bound::Unbounded => 0,
        });
        let end = match range.end_bound() {
            Bound::Included(&end) => self.position.saturating_add(end).saturating_add(1),
            Bound::Excluded(&end) => self.position.saturating_add(end),
            Bound::Unbounded => self.end,
        }
        .min(self.end);

        Self {
            buffer: self.buffer.clone(),
            position: start,
            end,
        }
    }

    /// Reads `count` bytes from the front of the buffer, returning a buffer
    /// that shares the same underlying buffer. `self` is advanced inside of the
    /// buffer to point.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// let mut buffer = ArcBytes::borrowed(b"abc");
    /// let ab = buffer.read_buffer(2).unwrap();
    /// assert_eq!(ab, b"ab");
    /// let c = buffer.read_buffer(1).unwrap();
    /// assert_eq!(c, b"c");
    /// assert_eq!(buffer, b"");
    /// ```
    pub fn read_buffer(&mut self, count: usize) -> Result<Self, std::io::Error> {
        let start = self.position;
        let end = self.position + count;
        if end > self.end {
            Err(std::io::Error::from(ErrorKind::UnexpectedEof))
        } else {
            self.position = end;
            Ok(Self {
                buffer: self.buffer.clone(),
                end,
                position: start,
            })
        }
    }

    /// Splits the buffer into two parts at `offset`. This method will not panic
    /// of `offset` is too large, instead it will be treated as if `offset` is
    /// `self.len()` -- the first buffer will contain all of the bytes, and the
    /// second buffer will be empty.
    ///
    /// ```rust
    /// # use arc_buffer::ArcBytes;
    /// let buffer = ArcBytes::borrowed(b"abc");
    /// let (ab, c) = buffer.split_at(2);
    /// assert_eq!(ab, b"ab");
    ///
    /// let (c, empty) = c.split_at(usize::MAX);
    /// assert_eq!(c, b"c");
    /// assert_eq!(empty, b"");
    /// ```
    #[must_use]
    pub fn split_at(self, offset: usize) -> (Self, Self) {
        let split_end = self.position.saturating_add(offset).min(self.end);
        (
            Self {
                buffer: self.buffer.clone(),
                position: self.position,
                end: split_end,
            },
            Self {
                buffer: self.buffer,
                position: split_end,
                end: self.end,
            },
        )
    }
}

#[test]
fn slice_tests() {
    let original = ArcBytes::borrowed(b"abc");
    let b = original.slice(1..=1);
    assert_eq!(b, b"b");
    let b = original.slice(1..2);
    assert_eq!(b, b"b");
    let ab = original.slice(..2);
    assert_eq!(ab, b"ab");
    let abc = original.slice(..);
    assert_eq!(abc, b"abc");
    let bc = original.slice(1..);
    assert_eq!(bc, b"bc");
}

impl<'a> From<Cow<'a, [u8]>> for ArcBytes<'a> {
    fn from(buffer: Cow<'a, [u8]>) -> Self {
        let end = buffer.len();
        Self {
            end,
            buffer: if end > 0 {
                Some(Arc::new(buffer))
            } else {
                None
            },
            position: 0,
        }
    }
}

#[test]
fn from_cow_tests() {
    let has_bytes = ArcBytes::from(Cow::Borrowed(&b"a"[..]));
    assert_eq!(has_bytes, b"a");

    let empty = ArcBytes::from(Cow::Borrowed(&b""[..]));
    assert!(empty.buffer.is_none());
}

impl<'a> From<Vec<u8>> for ArcBytes<'a> {
    fn from(buffer: Vec<u8>) -> Self {
        Self::owned(buffer)
    }
}

impl<'a> From<String> for ArcBytes<'a> {
    fn from(buffer: String) -> Self {
        Self::owned(buffer.into_bytes())
    }
}

impl<'a> From<&'a str> for ArcBytes<'a> {
    fn from(buffer: &'a str) -> Self {
        Self::borrowed(buffer.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for ArcBytes<'a> {
    fn from(buffer: &'a [u8]) -> Self {
        Self::borrowed(buffer)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for ArcBytes<'a> {
    fn from(buffer: &'a [u8; N]) -> Self {
        Self::borrowed(buffer)
    }
}

impl<'a, const N: usize> From<[u8; N]> for ArcBytes<'a> {
    fn from(buffer: [u8; N]) -> Self {
        Self::owned(buffer.to_vec())
    }
}

#[test]
fn conversion_tests() {
    assert_eq!(ArcBytes::from(b"hello".to_vec()), b"hello");
    assert_eq!(ArcBytes::from(String::from("hello")), b"hello");
    assert_eq!(ArcBytes::from("hello"), b"hello");
    assert_eq!(ArcBytes::from(&b"hello"[..]), b"hello");
    assert_eq!(ArcBytes::from(b"hello"), b"hello");
}

impl<'a> Deref for ArcBytes<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> Read for ArcBytes<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(buffer) = &self.buffer {
            let end = buffer.len().min(self.position + buf.len());
            let bytes_read = buf.len().min(end - self.position);

            if bytes_read == 0 {
                return Err(io::Error::from(ErrorKind::UnexpectedEof));
            }

            buf[..bytes_read].copy_from_slice(&buffer[self.position..end]);
            self.position = end;

            if self.position == self.end {
                self.buffer = None;
            }

            Ok(bytes_read)
        } else {
            Err(io::Error::from(ErrorKind::UnexpectedEof))
        }
    }
}

#[test]
fn read_tests() {
    let mut buffer = ArcBytes::borrowed(b"abc");
    let mut read_bytes = [0_u8; 2];
    assert_eq!(buffer.read(&mut read_bytes).unwrap(), 2);
    assert_eq!(&read_bytes, b"ab");
    assert_eq!(buffer.read(&mut read_bytes).unwrap(), 1);
    assert_eq!(&read_bytes, b"cb");
    assert!(buffer.read(&mut read_bytes).is_err());
    assert!(buffer.is_empty());
}

impl<'a> Iterator for ArcBytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.end {
            let byte = self[0];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }
}
#[test]
fn iterator_tests() {
    assert_eq!(ArcBytes::new().count(), 0);
    let iterated = ArcBytes::from(vec![0, 1, 2]).collect::<Vec<_>>();
    assert_eq!(iterated, vec![0, 1, 2]);
}

#[cfg(feature = "serde")]
mod serde;
