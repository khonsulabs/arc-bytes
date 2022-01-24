# arc-bytes

![arc-bytes forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
![arc-bytes is considered alpha](https://img.shields.io/badge/status-alpha-orange)
[![crate version](https://img.shields.io/crates/v/arc-bytes.svg)](https://crates.io/crates/arc-bytes)
[![Live Build Status](https://img.shields.io/github/workflow/status/khonsulabs/arc-bytes/Tests/main)](https://github.com/khonsulabs/arc-bytes/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/arc-bytes/coverage/badge.svg)](https://arc-bytes.bonsaidb.io/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://arc-bytes.bonsaidb.io/main/arc-bytes/)

A reference-counted byte buffer.

ArcBytes is a type that is useful when parsing a buffer of bytes and breaking it into smaller pieces without extra allocations or extending the lifetime of the underlying buffer. This is done with no unsafe code by leveraging `std::sync::Arc`. When the final reference of the buffer goes away, the underlying bytes will be released.

This type also implements `std::io::Read` and `std::iter::Iterator` for easier integration with existing code.

With the feature flag `serde` enabled, `serde::Serialize`/`serde::Deserialize` are implemented in such a way that ensures the bytes are written optimally and not as a sequence of u8s. This is almost identical to the approach that [serde_bytes](https://crates.io/crates/serde_bytes) utilizes, except ArcBytes uses a single type and introduces the use of Arc.


## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are
open-source. This repository is available under the [MIT License](./LICENSE-MIT)
or the [Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
