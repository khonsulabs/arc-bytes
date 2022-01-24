A reference-counted byte buffer.

ArcBytes is a type that is useful when parsing a buffer of bytes and breaking it into smaller pieces without extra allocations or extending the lifetime of the underlying buffer. This is done with no unsafe code by leveraging `std::sync::Arc`. When the final reference of the buffer goes away, the underlying bytes will be released.

This type also implements `std::io::Read` and `std::iter::Iterator` for easier integration with existing code.

With the feature flag `serde` enabled, `serde::Serialize`/`serde::Deserialize` are implemented in such a way that ensures the bytes are written optimally and not as a sequence of u8s. This is almost identical to the approach that [serde_bytes](https://crates.io/crates/serde_bytes) utilizes, except ArcBytes uses a single type and introduces the use of Arc.

