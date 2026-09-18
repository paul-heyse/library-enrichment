# `parquet::file::metadata::reader::PageIndexPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.reader.PageIndexPolicy.json).

<a id="op-7dc5da782b207c53dc34d37c"></a>
## PageIndexPolicy

`enum` · `parquet::file::metadata::reader::PageIndexPolicy` · parquet 59.3.0

```rust
enum PageIndexPolicy
```

Source: `src/file/metadata/reader.rs:86`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Describes the policy for reading page indexes

<a id="op-048be08cc587a4d901ecc115"></a>
## Optional

`variant` · `parquet::file::metadata::reader::PageIndexPolicy::Optional` · parquet 59.3.0

```rust
Optional
```

Source: `src/file/metadata/reader.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read the page index if it exists, otherwise do not error.

<a id="op-96a978c465bdb93a3a31f48c"></a>
## Required

`variant` · `parquet::file::metadata::reader::PageIndexPolicy::Required` · parquet 59.3.0

```rust
Required
```

Source: `src/file/metadata/reader.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Require the page index to exist, and error if it does not.

<a id="op-6f2d1d654ecec39294ec7a53"></a>
## Skip

`variant` · `parquet::file::metadata::reader::PageIndexPolicy::Skip` · parquet 59.3.0

```rust
Skip
```

Source: `src/file/metadata/reader.rs:89`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Do not read the page index.

<a id="op-7965237c8b0cc0df1275d4c4"></a>
## clone

`function` · `parquet::file::metadata::reader::PageIndexPolicy::clone` · parquet 59.3.0

```rust
fn clone(&self) -> PageIndexPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::PageIndexPolicy", "path": "PageIndexPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 17], "end": [85, 22], "filename": "src/file/metadata/reader.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/reader.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26dae83df68219b4f7032113"></a>
## default

`function` · `parquet::file::metadata::reader::PageIndexPolicy::default` · parquet 59.3.0

```rust
fn default() -> PageIndexPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::PageIndexPolicy", "path": "PageIndexPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 45], "end": [85, 52], "filename": "src/file/metadata/reader.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/metadata/reader.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04caf8e76841d5fed402bfe2"></a>
## eq

`function` · `parquet::file::metadata::reader::PageIndexPolicy::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &PageIndexPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::PageIndexPolicy", "path": "PageIndexPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 30], "end": [85, 39], "filename": "src/file/metadata/reader.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/reader.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a0764d091d9be4add998baf"></a>
## fmt

`function` · `parquet::file::metadata::reader::PageIndexPolicy::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::PageIndexPolicy", "path": "PageIndexPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 10], "end": [85, 15], "filename": "src/file/metadata/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/reader.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9620f36b1daf26107fd8a75f"></a>
## from

`function` · `parquet::file::metadata::reader::PageIndexPolicy::from` · parquet 59.3.0

```rust
fn from(value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::PageIndexPolicy", "path": "PageIndexPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [103, 2], "filename": "src/file/metadata/reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file/metadata/reader.rs:97`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
