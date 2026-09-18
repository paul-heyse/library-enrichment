# `datafusion_datasource::PartitionedFile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.PartitionedFile.json).

<a id="op-25225d18e868fe5ac8eeb098"></a>
## PartitionedFile

`struct` · `datafusion_datasource::PartitionedFile` · datafusion-datasource 55.1.0

```rust
struct PartitionedFile
```

Source: `src/mod.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A single file or part of a file that should be read, along with its schema, statistics
and partition column values that need to be appended to each row.

# Statistics

The [`Self::statistics`](../operations/datafusion_datasource.PartitionedFile.md#op-5f7ce370a96c8dddb08ec40b) field contains statistics for the **full table schema**,
which includes both file columns and partition columns. When statistics are set via
[`Self::with_statistics`](../operations/datafusion_datasource.PartitionedFile.md#op-b10d71edbae8bd5a81229f02), exact statistics for partition columns are automatically
computed from [`Self::partition_values`](../operations/datafusion_datasource.PartitionedFile.md#op-d8901b2cd43ad730d84b8935):

- `min = max = partition_value` (all rows in a file share the same partition value)
- `null_count = 0` (partition values extracted from paths are never null)
- `distinct_count = 1` (single distinct value per file for each partition column)

This enables query optimizers to use partition column bounds for pruning and planning.

<a id="op-13b666e688dfe3624cd8023a"></a>
## Error

`assoc_type` · `datafusion_datasource::PartitionedFile::Error` · datafusion-datasource 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "crate::PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [132, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PartitionedFile", "path": "PartitionedFile"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cc5f49119d85d17d558ffeb"></a>
## arrow_schema

`struct_field` · `datafusion_datasource::PartitionedFile::arrow_schema` · datafusion-datasource 55.1.0

```rust
arrow_schema: Option<arrow::datatypes::SchemaRef>
```

Source: `src/mod.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A user-provided physical Arrow schema for this file.

This schema describes only the columns stored in the file. It must not
include partition columns; those are represented separately by
[`Self::partition_values`](../operations/datafusion_datasource.PartitionedFile.md#op-d8901b2cd43ad730d84b8935) and the scan's table partition columns.

When provided, this field will be used by the Parquet reader to avoid
parsing the Arrow schema from the `ARROW:schema` metadata key. Other
built-in file sources ignore it for now.

<a id="op-366f74f7c6dca211dadbd196"></a>
## clone

`function` · `datafusion_datasource::PartitionedFile::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> PartitionedFile
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 17], "end": [109, 22], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/mod.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71e6ad2e5ca1f360dcb045f7"></a>
## effective_size

`function` · `datafusion_datasource::PartitionedFile::effective_size` · datafusion-datasource 55.1.0

```rust
fn effective_size(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Size of the file to be scanned (taking into account the range, if present).

<a id="op-4fcbda67421f52f7b2fb3e62"></a>
## extension

`function` · `datafusion_datasource::PartitionedFile::extension` · datafusion-datasource 55.1.0

```rust
fn extension<T: Any + Send + Sync>(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Borrow the extension of type `T`, if one is attached.

<a id="op-29329e555f8b444fb1ce50bd"></a>
## extensions

`struct_field` · `datafusion_datasource::PartitionedFile::extensions` · datafusion-datasource 55.1.0

```rust
extensions: FileExtensions
```

Source: `src/mod.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

User-defined per-file metadata, keyed by Rust type. Multiple
independent components can each attach their own data here without
conflict — see [`FileExtensions`](../operations/datafusion_datasource.FileExtensions.md#op-f98598f3aec2efb2fae85138).

<a id="op-789d0a91b18f0d60927519b9"></a>
## fmt

`function` · `datafusion_datasource::PartitionedFile::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 10], "end": [109, 15], "filename": "src/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/mod.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ab37253c575a5def50e390f"></a>
## from

`function` · `datafusion_datasource::PartitionedFile::from` · datafusion-datasource 55.1.0

```rust
fn from(object_meta: ObjectMeta) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [398, 1], "end": [412, 2], "filename": "src/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::ObjectMeta", "path": "ObjectMeta"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/mod.rs:399`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a0a30c9522d5faaf7c03358"></a>
## from_path

`function` · `datafusion_datasource::PartitionedFile::from_path` · datafusion-datasource 55.1.0

```rust
fn from_path(path: String) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return a file reference from the given path

<a id="op-4acd38629911258ee5a858bf"></a>
## has_statistics

`function` · `datafusion_datasource::PartitionedFile::has_statistics` · datafusion-datasource 55.1.0

```rust
fn has_statistics(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Check if this file has any statistics.
This returns `true` if the file has any Exact or Inexact statistics
and `false` if all statistics are `Precision::Absent`.

<a id="op-8487517282f50829d64862ac"></a>
## metadata_size_hint

`struct_field` · `datafusion_datasource::PartitionedFile::metadata_size_hint` · datafusion-datasource 55.1.0

```rust
metadata_size_hint: Option<usize>
```

Source: `src/mod.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The estimated size of the parquet metadata, in bytes

<a id="op-33632db1d3ad60e9d0290240"></a>
## new

`function` · `datafusion_datasource::PartitionedFile::new` · datafusion-datasource 55.1.0

```rust
fn new(path: impl Into<String>, size: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a simple file without metadata or partition

<a id="op-f5c9e1d11dea7ff96af2f29e"></a>
## new_from_meta

`function` · `datafusion_datasource::PartitionedFile::new_from_meta` · datafusion-datasource 55.1.0

```rust
fn new_from_meta(object_meta: ObjectMeta) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a file from a known ObjectMeta without partition

<a id="op-d77c73189fe4fd1f136f0750"></a>
## new_with_range

`function` · `datafusion_datasource::PartitionedFile::new_with_range` · datafusion-datasource 55.1.0

```rust
fn new_with_range(path: String, size: u64, start: i64, end: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a file range without metadata or partition

<a id="op-16f41e34f0f0978411a81bd1"></a>
## object_meta

`struct_field` · `datafusion_datasource::PartitionedFile::object_meta` · datafusion-datasource 55.1.0

```rust
object_meta: object_store::ObjectMeta
```

Source: `src/mod.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Path for the file (e.g. URL, filesystem path, etc)

<a id="op-4a8cdafba953eab901b49a89"></a>
## ordering

`struct_field` · `datafusion_datasource::PartitionedFile::ordering` · datafusion-datasource 55.1.0

```rust
ordering: Option<datafusion_physical_expr::LexOrdering>
```

Source: `src/mod.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The known lexicographical ordering of the rows in this file, if any.

This describes how the data within the file is sorted with respect to one or more
columns, and is used by the optimizer for planning operations that depend on input
ordering (e.g. merges, sorts, and certain aggregations).

When available, this is typically inferred from file-level metadata exposed by the
underlying format (for example, Parquet `sorting_columns`), but it may also be set
explicitly via [`Self::with_ordering`](../operations/datafusion_datasource.PartitionedFile.md#op-2996355a52fc9ea0a1c42113).

<a id="op-d8901b2cd43ad730d84b8935"></a>
## partition_values

`struct_field` · `datafusion_datasource::PartitionedFile::partition_values` · datafusion-datasource 55.1.0

```rust
partition_values: Vec<datafusion_common::ScalarValue>
```

Source: `src/mod.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Values of partition columns to be appended to each row.

These MUST have the same count, order, and type than the [`table_partition_cols`].

You may use [`wrap_partition_value_in_dict`] to wrap them if you have used [`wrap_partition_type_in_dict`] to wrap the column type.


[`wrap_partition_type_in_dict`]: crate::file_scan_config::wrap_partition_type_in_dict
[`wrap_partition_value_in_dict`]: crate::file_scan_config::wrap_partition_value_in_dict
[`table_partition_cols`]: https://github.com/apache/datafusion/blob/main/datafusion/core/src/datasource/file_format/options.rs#L87

<a id="op-bd3066791e556c0b1dc3c312"></a>
## path

`function` · `datafusion_datasource::PartitionedFile::path` · datafusion-datasource 55.1.0

```rust
fn path(&self) -> &Path
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the path of this partitioned file

<a id="op-bea6a7f2402b66ff993485c7"></a>
## range

`struct_field` · `datafusion_datasource::PartitionedFile::range` · datafusion-datasource 55.1.0

```rust
range: Option<FileRange>
```

Source: `src/mod.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

An optional file range for a more fine-grained parallel execution

<a id="op-df64de053f0d6790b9aa1c10"></a>
## range

`function` · `datafusion_datasource::PartitionedFile::range` · datafusion-datasource 55.1.0

```rust
fn range(&self) -> (u64, u64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Effective range of the file to be scanned.

<a id="op-5f7ce370a96c8dddb08ec40b"></a>
## statistics

`struct_field` · `datafusion_datasource::PartitionedFile::statistics` · datafusion-datasource 55.1.0

```rust
statistics: Option<std::sync::Arc<datafusion_common::Statistics>>
```

Source: `src/mod.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Optional statistics that describe the data in this file if known.

DataFusion relies on these statistics for planning (in particular to sort file groups),
so if they are incorrect, incorrect answers may result.

These statistics cover the full table schema: file columns plus partition columns.
When set via [`Self::with_statistics`](../operations/datafusion_datasource.PartitionedFile.md#op-b10d71edbae8bd5a81229f02), partition column statistics are automatically
computed from [`Self::partition_values`](../operations/datafusion_datasource.PartitionedFile.md#op-d8901b2cd43ad730d84b8935) with exact min/max/null_count/distinct_count.

<a id="op-ae1a380e14dd09cc4d3b1534"></a>
## table_reference

`struct_field` · `datafusion_datasource::PartitionedFile::table_reference` · datafusion-datasource 55.1.0

```rust
table_reference: Option<datafusion_common::TableReference>
```

Source: `src/mod.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b9d021b85f7ede53916d6e3"></a>
## try_from

`function` · `datafusion_datasource::PartitionedFile::try_from` · datafusion-datasource 55.1.0

```rust
fn try_from(file: &protobuf::PartitionedFile) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "crate::PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [132, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PartitionedFile", "path": "PartitionedFile"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e315f25d30dc2759f7417801"></a>
## with_arrow_schema

`function` · `datafusion_datasource::PartitionedFile::with_arrow_schema` · datafusion-datasource 55.1.0

```rust
fn with_arrow_schema(self, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Provide a physical Arrow schema for this file.

The schema must describe only columns stored in the file and must not
include partition columns. See [`Self::arrow_schema`](../operations/datafusion_datasource.PartitionedFile.md#op-7cc5f49119d85d17d558ffeb) for details.

<a id="op-52591cabfd2d61e16c0508ef"></a>
## with_extension

`function` · `datafusion_datasource::PartitionedFile::with_extension` · datafusion-datasource 55.1.0

```rust
fn with_extension<T: Any + Send + Sync>(self, value: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Attach a typed user-defined extension to this file. Multiple
independent extensions can be attached, each keyed by its concrete
Rust type. Inserting a value of a type that already has an extension
replaces the previous one.

This can be used to pass reader-specific information (e.g. a
`ParquetAccessPlan`, or a custom index entry).

<a id="op-8362bbdc0eb3e2d87fcba3f7"></a>
## with_extensions

`function` · `datafusion_datasource::PartitionedFile::with_extensions` · datafusion-datasource 55.1.0

```rust
fn with_extensions(self, extensions: Arc<dyn Any + Send + Sync>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Attach a type-erased extension to this file.

Kept as a backwards-compatible shim; prefer [`Self::with_extension`](../operations/datafusion_datasource.PartitionedFile.md#op-52591cabfd2d61e16c0508ef)
which keys the extension by its concrete Rust type at the call site.

<a id="op-628c8564fcebfb6e36c63502"></a>
## with_metadata_size_hint

`function` · `datafusion_datasource::PartitionedFile::with_metadata_size_hint` · datafusion-datasource 55.1.0

```rust
fn with_metadata_size_hint(self, metadata_size_hint: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Provide a hint to the size of the file metadata. If a hint is provided
the reader will try and fetch the last `size_hint` bytes of the parquet file optimistically.
Without an appropriate hint, two read may be required to fetch the metadata.

<a id="op-2996355a52fc9ea0a1c42113"></a>
## with_ordering

`function` · `datafusion_datasource::PartitionedFile::with_ordering` · datafusion-datasource 55.1.0

```rust
fn with_ordering(self, ordering: Option<LexOrdering>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the known ordering of data in this file.

The ordering represents the lexicographical sort order of the data,
typically inferred from file metadata (e.g., Parquet sorting_columns).

<a id="op-ea9ede7d6377509708f3d128"></a>
## with_partition_values

`function` · `datafusion_datasource::PartitionedFile::with_partition_values` · datafusion-datasource 55.1.0

```rust
fn with_partition_values(self, partition_values: Vec<ScalarValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Attach partition values to this file.
This replaces any existing partition values.

<a id="op-899967579acfc6e96d3c08b7"></a>
## with_range

`function` · `datafusion_datasource::PartitionedFile::with_range` · datafusion-datasource 55.1.0

```rust
fn with_range(self, start: i64, end: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Update the file to only scan the specified range (in bytes)

<a id="op-b10d71edbae8bd5a81229f02"></a>
## with_statistics

`function` · `datafusion_datasource::PartitionedFile::with_statistics` · datafusion-datasource 55.1.0

```rust
fn with_statistics(self, file_statistics: Arc<Statistics>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Update the statistics for this file.

The provided `statistics` should cover only the file schema columns.
This method will automatically append exact statistics for partition columns
based on `partition_values`:
- `min = max = partition_value` (all rows have the same value)
- `null_count = 0` (partition values from paths are never null)
- `distinct_count = 1` (all rows have the same partition value)

<a id="op-44495c4f62659b9348548b86"></a>
## with_table_reference

`function` · `datafusion_datasource::PartitionedFile::with_table_reference` · datafusion-datasource 55.1.0

```rust
fn with_table_reference(self, table_reference: Option<TableReference>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [396, 2], "filename": "src/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
