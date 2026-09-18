# `datafusion_datasource::FileRange`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.FileRange.json).

<a id="op-b0c915aa68004cf392eb089d"></a>
## FileRange

`struct` · `datafusion_datasource::FileRange` · datafusion-datasource 55.1.0

```rust
struct FileRange
```

Source: `src/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Only scan a subset of Row Groups from the Parquet file whose data "midpoint"
lies within the [start, end) byte offsets. This option can be used to scan non-overlapping
sections of a Parquet file in parallel.

<a id="op-7ed1ab68ea3f443687f41334"></a>
## Error

`assoc_type` · `datafusion_datasource::FileRange::Error` · datafusion-datasource 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "crate::FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [64, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileRange", "path": "FileRange"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fff0bf05fb2cabce88cbc147"></a>
## clone

`function` · `datafusion_datasource::FileRange::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileRange
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 17], "end": [94, 22], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c383de5e09537f1f1a13ab8"></a>
## cmp

`function` · `datafusion_datasource::FileRange::cmp` · datafusion-datasource 55.1.0

```rust
fn cmp(&self, other: &FileRange) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 57], "end": [94, 60], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77a04c91776c45b043816d5f"></a>
## contains

`function` · `datafusion_datasource::FileRange::contains` · datafusion-datasource 55.1.0

```rust
fn contains(&self, offset: i64) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [107, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

returns true if this file range contains the specified offset

<a id="op-7820d05dc4d7caf9b5258749"></a>
## end

`struct_field` · `datafusion_datasource::FileRange::end` · datafusion-datasource 55.1.0

```rust
end: i64
```

Source: `src/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Range end

<a id="op-0e48c1a000ea7b2525132897"></a>
## eq

`function` · `datafusion_datasource::FileRange::eq` · datafusion-datasource 55.1.0

```rust
fn eq(&self, other: &FileRange) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 24], "end": [94, 33], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6c0834add8c8c60a69e9d80"></a>
## fmt

`function` · `datafusion_datasource::FileRange::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 10], "end": [94, 15], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a68d88e4cf1d763ce1b60151"></a>
## hash

`function` · `datafusion_datasource::FileRange::hash` · datafusion-datasource 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 35], "end": [94, 39], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-070714ae9c1107b9d300568f"></a>
## partial_cmp

`function` · `datafusion_datasource::FileRange::partial_cmp` · datafusion-datasource 55.1.0

```rust
fn partial_cmp(&self, other: &FileRange) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 45], "end": [94, 55], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb52c449574d0fb82cfc74b3"></a>
## start

`struct_field` · `datafusion_datasource::FileRange::start` · datafusion-datasource 55.1.0

```rust
start: i64
```

Source: `src/mod.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Range start

<a id="op-1bb59f5a92eeea29c6388ad5"></a>
## try_from

`function` · `datafusion_datasource::FileRange::try_from` · datafusion-datasource 55.1.0

```rust
fn try_from(range: &protobuf::FileRange) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::FileRange", "path": "crate::FileRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [64, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileRange", "path": "FileRange"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
