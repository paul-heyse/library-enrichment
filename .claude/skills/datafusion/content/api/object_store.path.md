# `object_store::path`

Crate `object_store` · 5 public items · structured records in [`model/object_store.path.json`](../model/object_store.path.json)

## DELIMITER

`constant` · `object_store::path::DELIMITER`

```rust
const DELIMITER: &str = "/"
```

[Full member, field, variant and typed contracts](../operations/object_store.path.DELIMITER.md).


The delimiter to separate object namespaces, creating a directory structure.

---

## DELIMITER_BYTE

`constant` · `object_store::path::DELIMITER_BYTE`

```rust
const DELIMITER_BYTE: u8 = _
```

[Full member, field, variant and typed contracts](../operations/object_store.path.DELIMITER_BYTE.md).


The path delimiter as a single byte

---

## DELIMITER_CHAR

`constant` · `object_store::path::DELIMITER_CHAR`

```rust
const DELIMITER_CHAR: char = _
```

[Full member, field, variant and typed contracts](../operations/object_store.path.DELIMITER_CHAR.md).


The path delimiter as a single char

---

## Error

`enum` · `object_store::path::Error`

```rust
enum Error
```

**Variants**: `EmptySegment`, `BadSegment`, `Canonicalize`, `InvalidPath`, `NonUnicode`, `PrefixMismatch`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/object_store.path.Error.md).


Error returned by [`Path::parse`]

---

## Path

`struct` · `object_store::path::Path`

```rust
struct Path
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `datafusion_execution::cache::CacheKey`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (14)

```rust
fn child<'a>(&self, child: impl Into<PathPart<'a>>) -> Self
fn extension(&self) -> Option<&str>
fn filename(&self) -> Option<&str>
fn from_absolute_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error>
fn from_filesystem_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error>
fn from_url_path(path: impl AsRef<str>) -> Result<Self, Error>
fn is_root(&self) -> bool
fn join<'a>(self, child: impl Into<PathPart<'a>>) -> Self
fn parent(&self) -> Option<Self>
fn parse(path: impl AsRef<str>) -> Result<Self, Error>
fn parts(&self) -> PathParts<'_>
fn parts_count(&self) -> usize
fn prefix_match(&self, prefix: &Self) -> Option<impl Iterator<Item = PathPart<'_>> + '_>
fn prefix_matches(&self, prefix: &Self) -> bool
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(path: String) -> Self
fn from(path: &str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = I>>(&mut self, iter: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.path.Path.md).


A parsed path representation that can be safely written to object storage

A [`Path`] maintains the following invariants:

* Paths are delimited by `/`
* Paths do not contain leading or trailing `/`
* Paths do not contain relative path segments, i.e. `.` or `..`
* Paths do not contain empty path segments
* Paths do not contain any ASCII control characters

There are no enforced restrictions on path length, however, it should be noted that most
object stores do not permit paths longer than 1024 bytes, and many filesystems do not
support path segments longer than 255 bytes.

# Encode

In theory object stores support any UTF-8 character sequence, however, certain character
sequences cause compatibility problems with some applications and protocols. Additionally
some filesystems may impose character restrictions, see [`LocalFileSystem`]. As such the
naming guidelines for [S3], [GCS] and [Azure Blob Storage] all recommend sticking to a
limited character subset.

[S3]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html
[GCS]: https://cloud.google.com/storage/docs/naming-objects
[Azure Blob Storage]: https://docs.microsoft.com/en-us/rest/api/storageservices/Naming-and-Referencing-Containers--Blobs--and-Metadata#blob-names

A string containing potentially problematic path segments can therefore be encoded to a [`Path`]
using [`Path::from`] or [`Path::from_iter`]. This will percent encode any problematic
segments according to [RFC 1738].

```
# use object_store::path::Path;
assert_eq!(Path::from("foo/bar").as_ref(), "foo/bar");
assert_eq!(Path::from("foo//bar").as_ref(), "foo/bar");
assert_eq!(Path::from("foo/../bar").as_ref(), "foo/%2E%2E/bar");
assert_eq!(Path::from("/").as_ref(), "");
assert_eq!(Path::from_iter(["foo", "foo/bar"]).as_ref(), "foo/foo%2Fbar");
```

Note: if provided with an already percent encoded string, this will encode it again

```
# use object_store::path::Path;
assert_eq!(Path::from("foo/foo%2Fbar").as_ref(), "foo/foo%252Fbar");
```

# Parse

Alternatively a [`Path`] can be parsed from an existing string, returning an
error if it is invalid. Unlike the encoding methods above, this will permit
arbitrary unicode, including percent encoded sequences.

```
# use object_store::path::Path;
assert_eq!(Path::parse("/foo/foo%2Fbar").unwrap().as_ref(), "foo/foo%2Fbar");
Path::parse("..").unwrap_err(); // Relative path segments are disallowed
Path::parse("/foo//").unwrap_err(); // Empty path segments are disallowed
Path::parse("\x00").unwrap_err(); // ASCII control characters are disallowed
```

[RFC 1738]: https://www.ietf.org/rfc/rfc1738.txt
[`LocalFileSystem`]: crate::local::LocalFileSystem

---
