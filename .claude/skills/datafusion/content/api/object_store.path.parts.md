# `object_store::path::parts`

Crate `object_store` · 3 public items · structured records in [`model/object_store.path.parts.json`](../model/object_store.path.parts.json)

## InvalidPart

`struct` · `object_store::path::parts::InvalidPart`

Also reachable as `object_store::path::InvalidPart`

```rust
struct InvalidPart
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Error returned by [`PathPart::parse`]

---

## PathPart

`struct` · `object_store::path::parts::PathPart`

Also reachable as `object_store::path::PathPart`

```rust
struct PathPart<'a>
```

**Implements**: `core::convert::AsRef`, `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn parse(segment: &'a str) -> Result<Self, InvalidPart>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(s: String) -> Self
fn from(v: &'a str) -> Self
fn from(v: &'a [u8]) -> Self
```

The PathPart type exists to validate the directory/file names that form part
of a path.

A [`PathPart`] is guaranteed to:

* Contain no ASCII control characters or `/`
* Not be a relative path segment, i.e. `.` or `..`

---

## PathParts

`struct` · `object_store::path::parts::PathParts`

Also reachable as `object_store::path::PathParts`

```rust
struct PathParts<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

See [`Path::parts`](super::Path::parts)

---
