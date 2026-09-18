# `object_store::path::Path`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.path.Path.json).

<a id="op-387136a1d8baa9d740b7fc0b"></a>
## Path

`struct` · `object_store::path::Path` · object_store 0.13.2

```rust
struct Path
```

Source: `src/path/mod.rs:156`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A parsed path representation that can be safely written to object storage

A [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) maintains the following invariants:

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

A string containing potentially problematic path segments can therefore be encoded to a [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)
using [`Path::from`](../operations/object_store.path.Path.md#op-8c696131a30335d863498a5f) or [`Path::from_iter`](../operations/object_store.path.Path.md#op-a28dd2aea8d48f3cc09dd532). This will percent encode any problematic
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

Alternatively a [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) can be parsed from an existing string, returning an
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

<a id="op-011a2f683869e3084d134b62"></a>
## ROOT

`assoc_const` · `object_store::path::Path::ROOT` · object_store 0.13.2

```rust
ROOT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:172`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An empty [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) that points to the root of the store, equivalent to `Path::from("/")`.

See also [`Path::is_root`](../operations/object_store.path.Path.md#op-981993b69829b6b4775f88fc).

# Example

```
# use object_store::path::Path;
assert_eq!(Path::ROOT, Path::from("/"));
```

<a id="op-efffcb9ea612985a4710c129"></a>
## as_ref

`function` · `object_store::path::Path::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 1], "end": [390, 2], "filename": "src/path/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/path/mod.rs:387`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63250cf8153225fc60dcfed0"></a>
## child

`function` · `object_store::path::Path::child` · object_store 0.13.2

```rust
fn child<'a>(&self, child: impl Into<PathPart<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:364`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Creates a new child of this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

<a id="op-1e390cd1cdebed0ed608954b"></a>
## clone

`function` · `object_store::path::Path::clone` · object_store 0.13.2

```rust
fn clone(&self) -> Path
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 17], "end": [155, 22], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/path/mod.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfabfee69a4760f40c6d4ee2"></a>
## cmp

`function` · `object_store::path::Path::cmp` · object_store 0.13.2

```rust
fn cmp(&self, other: &Path) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 54], "end": [155, 57], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/path/mod.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a628c496d31c6654584862b"></a>
## default

`function` · `object_store::path::Path::default` · object_store 0.13.2

```rust
fn default() -> Path
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 24], "end": [155, 31], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/path/mod.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d7f3fa38cf46fb35ad5dd7f"></a>
## eq

`function` · `object_store::path::Path::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Path) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 33], "end": [155, 42], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/path/mod.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc8f79404ffec4f22b6ff3e1"></a>
## extend

`function` · `object_store::path::Path::extend` · object_store 0.13.2

```rust
fn extend<T: IntoIterator<Item = I>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [468, 2], "filename": "src/path/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/path/mod.rs:457`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6457f8477289ad26dc5de5a5"></a>
## extension

`function` · `object_store::path::Path::extension` · object_store 0.13.2

```rust
fn extension(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:334`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the extension of the file stored in this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b), if any

<a id="op-ef2fadc3eba789f8eb362bc8"></a>
## filename

`function` · `object_store::path::Path::filename` · object_store 0.13.2

```rust
fn filename(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:326`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the last path segment containing the filename stored in this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

Returns `None` only if this path is the root path.

<a id="op-5b927815d4fc59ae1ee96ae3"></a>
## fmt

`function` · `object_store::path::Path::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 10], "end": [155, 15], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/path/mod.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fac695e243c9e9d673bbbbc8"></a>
## fmt

`function` · `object_store::path::Path::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [410, 1], "end": [414, 2], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/path/mod.rs:411`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53fbd4921383217df50d8fc2"></a>
## from

`function` · `object_store::path::Path::from` · object_store 0.13.2

```rust
fn from(path: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [398, 1], "end": [402, 2], "filename": "src/path/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path/mod.rs:399`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c696131a30335d863498a5f"></a>
## from

`function` · `object_store::path::Path::from` · object_store 0.13.2

```rust
fn from(path: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [392, 1], "end": [396, 2], "filename": "src/path/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path/mod.rs:393`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1322e988ea9aee9c612eb1e2"></a>
## from_absolute_path

`function` · `object_store::path::Path::from_absolute_path` · object_store 0.13.2

```rust
fn from_absolute_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:225`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Convert an absolute filesystem path to a [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) relative to the filesystem root

This will return an error if the path contains illegal character sequences,
as defined on the docstring for [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b), or `base` is not an absolute path

<a id="op-38c397932733010603a5ec72"></a>
## from_filesystem_path

`function` · `object_store::path::Path::from_filesystem_path` · object_store 0.13.2

```rust
fn from_filesystem_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:211`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Convert a filesystem path to a [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) relative to the filesystem root

This will return an error if the path contains illegal character sequences
as defined on the docstring for [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) or does not exist

Note: this will canonicalize the provided path, resolving any symlinks

<a id="op-a28dd2aea8d48f3cc09dd532"></a>
## from_iter

`function` · `object_store::path::Path::from_iter` · object_store 0.13.2

```rust
fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "I"}}}]}, "is_negative": false, "span": {"begin": [416, 1], "end": [425, 2], "filename": "src/path/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/path/mod.rs:420`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4087eb0f5cfa3447882a799"></a>
## from_url_path

`function` · `object_store::path::Path::from_url_path` · object_store 0.13.2

```rust
fn from_url_path(path: impl AsRef<str>) -> Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:260`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parse a url encoded string as a [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b), returning a [`Error`](../operations/object_store.path.Error.md#op-28f038cdccc6e35e04b70c9a) if invalid

This will return an error if the path contains illegal character sequences
as defined on the docstring for [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

<a id="op-ee457f63e8945aa1981e2a83"></a>
## hash

`function` · `object_store::path::Path::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 48], "end": [155, 52], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/path/mod.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-981993b69829b6b4775f88fc"></a>
## is_root

`function` · `object_store::path::Path::is_root` · object_store 0.13.2

```rust
fn is_root(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:295`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

True if this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) points to the root of the store, equivalent to `Path::from("/")`.

See also [`Path::ROOT`](../operations/object_store.path.Path.md#op-011a2f683869e3084d134b62).

# Example

```
# use object_store::path::Path;
assert!(Path::from("/").is_root());
assert!(Path::parse("").unwrap().is_root());
```

<a id="op-f490ebef4de2ead67cee1869"></a>
## join

`function` · `object_store::path::Path::join` · object_store 0.13.2

```rust
fn join<'a>(self, child: impl Into<PathPart<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:369`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Appends a single path segment to this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

<a id="op-844375e8d384308b1f52aa0d"></a>
## parent

`function` · `object_store::path::Path::parent` · object_store 0.13.2

```rust
fn parent(&self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:309`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns a copy of this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) with the last path segment removed

Returns `None` if this path has zero segments.

<a id="op-93733e7df06c53ee809baf53"></a>
## parse

`function` · `object_store::path::Path::parse` · object_store 0.13.2

```rust
fn parse(path: impl AsRef<str>) -> Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:178`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parse a string as a [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b), returning a [`Error`](../operations/object_store.path.Error.md#op-28f038cdccc6e35e04b70c9a) if invalid,
as defined on the docstring for [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

Note: this will strip any leading `/` or trailing `/`

<a id="op-0f015bcd12aeb2eca0f68f19"></a>
## partial_cmp

`function` · `object_store::path::Path::partial_cmp` · object_store 0.13.2

```rust
fn partial_cmp(&self, other: &Path) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 59], "end": [155, 69], "filename": "src/path/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/path/mod.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5978848ccd726f2ebc77e1e"></a>
## parts

`function` · `object_store::path::Path::parts` · object_store 0.13.2

```rust
fn parts(&self) -> PathParts<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:302`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the [`PathPart`](../operations/object_store.path.parts.PathPart.md#op-c427f5625ecac2ba0151d814)s of this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

Equivalent to calling `.into_iter()` on a `&Path`.

<a id="op-558519a2a6216e5570463776"></a>
## parts_count

`function` · `object_store::path::Path::parts_count` · object_store 0.13.2

```rust
fn parts_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:280`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the number of [`PathPart`](../operations/object_store.path.parts.PathPart.md#op-c427f5625ecac2ba0151d814)s in this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

This is equivalent to calling `.parts().count()` manually.

# Performance

This operation is `O(n)`.

<a id="op-d34adabdef579634e8e20d07"></a>
## prefix_match

`function` · `object_store::path::Path::prefix_match` · object_store 0.13.2

```rust
fn prefix_match(&self, prefix: &Self) -> Option<impl Iterator<Item = PathPart<'_>> + '_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:349`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns an iterator of the [`PathPart`](../operations/object_store.path.parts.PathPart.md#op-c427f5625ecac2ba0151d814) of this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) after `prefix`

Returns `None` if the prefix does not match.

<a id="op-1d2f4e2c260d49521fdbb6d1"></a>
## prefix_matches

`function` · `object_store::path::Path::prefix_matches` · object_store 0.13.2

```rust
fn prefix_matches(&self, prefix: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::path::Path", "path": "Path"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [384, 2], "filename": "src/path/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/mod.rs:358`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns true if this [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) starts with `prefix`
