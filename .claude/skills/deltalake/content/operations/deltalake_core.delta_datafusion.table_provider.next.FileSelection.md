# `deltalake_core::delta_datafusion::table_provider::next::FileSelection`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.FileSelection.json).

<a id="op-44d6faced7d64d02caf12640"></a>
## FileSelection

`struct` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileSelection
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L102).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:102`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File selection for a [`DeltaScan`](../operations/deltalake_core.delta_datafusion.table_provider.next.DeltaScan.md#op-e35fcdd4ce5cf85731bf3f69) snapshot.

A selection limits a scan to explicit files, for example paths returned by
[`crate::DeltaTable::get_files_by_partitions`](../operations/deltalake_core.table.DeltaTable.md#op-3c06b53ce88237aa694f0955) or Add actions produced by maintenance tasks.

The input identifies files. Metadata comes from the scan snapshot, including
deletion vectors, partition values, statistics, column mapping, and tags.

Empty selections produce empty scans. Duplicate inputs are deduplicated after the scan snapshot
is known. Add inputs contribute only their path. The default policy returns an error for files
that are not active in the scan snapshot. [`MissingSelectedFilePolicy::Ignore`](../operations/deltalake_core.delta_datafusion.table_provider.next.MissingSelectedFilePolicy.md#op-32653216dfc55154bf74f90f) skips those
files. Query pruning does not mark a selected file as missing.

Absolute URLs must be under the table root. Paths outside the table root are rejected during
scan planning with redacted error output.
Username, password, query, and fragment are stripped from URLs before storage.

Selections with paths are resolved against snapshot metadata before the data scan. Each scan
with a file selection requires that metadata pass.

<a id="op-127998d2af0198b620e77c14"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> FileSelection
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::FileSelection", "path": "FileSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 10], "end": [101, 15], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36adf4028918b7e0b1b8b8c2"></a>
## deserialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::FileSelection", "path": "FileSelection"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 28], "end": [101, 39], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33f18fb06366428fbc42f853"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::FileSelection", "path": "FileSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [182, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c2d43c20a2942c4b989f74f"></a>
## from_adds

`function` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::from_adds` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_adds(adds: impl IntoIterator<Item = Add>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L131).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::FileSelection", "path": "FileSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:131`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Select files from Add actions.

Only the path is used. File metadata is loaded from the scan snapshot.

<a id="op-260f00c1b1030065789d08ec"></a>
## from_file_paths

`function` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::from_file_paths` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_file_paths(paths: impl IntoIterator<Item = impl Into<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L143).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::FileSelection", "path": "FileSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Select files by path.

Paths may be relative to the table root or absolute URLs under the same table root.
Username, password, query, and fragment are stripped from URLs.
Path validation happens when a scan is planned against a concrete table root.

<a id="op-9392b80921f1142db91d236c"></a>
## serialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::FileSelection", "path": "FileSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 17], "end": [101, 26], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1f9ef87a8b3141597a178d5"></a>
## with_missing_file_policy

`function` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::with_missing_file_policy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_missing_file_policy(self, policy: MissingSelectedFilePolicy) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L155).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::FileSelection", "path": "FileSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the policy for selected files that are not active in the scan snapshot.

<a id="op-4307fe12a97d26b331432851"></a>
## missing_file_policy

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::missing_file_policy` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
missing_file_policy: MissingSelectedFilePolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L104).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8e67b53edd911e590549ac"></a>
## paths

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection::paths` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
paths: Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L103).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
