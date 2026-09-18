# `deltalake_core::kernel::snapshot::iterators::LogicalFileView`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.iterators.LogicalFileView.json).

<a id="op-f5a43d85036c73510b6d2a44"></a>
## LogicalFileView

`struct` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LogicalFileView
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L107).

Source: `crates/core/src/kernel/snapshot/iterators.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provides semantic, typed access to file metadata from Delta log replay.

This struct wraps a RecordBatch containing file data and provides zero-copy
access to individual file entries through an index. It serves as a view into
the kernel's log replay results, offering convenient methods to extract
file properties without unnecessary data copies.

<a id="op-5dec0224081a700f134710e7"></a>
## add_action

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::add_action` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_action(&self) -> Add
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:376`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Converts this file view into an Add action for log operations.

<a id="op-de83171d6c8d4e90c177a5ee"></a>
## clone

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LogicalFileView
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 10], "end": [106, 15], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3cb05f20bcc6907115f8ec1"></a>
## deletion_vector_descriptor

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::deletion_vector_descriptor` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deletion_vector_descriptor(&self) -> Option<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L325).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:325`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the underlying [DeletionVectorDescriptor](../operations/deltalake_core.kernel.models.actions.DeletionVectorDescriptor.md#op-ce822833f66e7aced10003ce) if it exists.

**NOTE**: THis API may be removed in the future without deprecation warnings as the
utilization of deletion vectors inside of delta-rs becomes more sophisticated.

<a id="op-4b9f91bb643806a1ca6898ec"></a>
## max_values

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::max_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn max_values(&self) -> Option<Scalar>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L314).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:314`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns maximum values for all columns in this file as structured data.

For timestamp columns, values are rounded up to handle microsecond truncation
in checkpoint statistics.

<a id="op-23a7319b042f7c1b8a0c2b34"></a>
## min_values

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::min_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn min_values(&self) -> Option<Scalar>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L304).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:304`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns minimum values for all columns with statics in this file as structured data.

<a id="op-34009151af36eb5c332de9ea"></a>
## modification_datetime

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::modification_datetime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn modification_datetime(&self) -> DeltaResult<chrono::DateTime<Utc>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L170).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:170`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the file modification time as a UTC DateTime.

<a id="op-47026c5c7ecfeb2ec00f817a"></a>
## modification_time

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::modification_time` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn modification_time(&self) -> i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L162).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:162`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the file modification time in milliseconds since Unix epoch.

<a id="op-1445f87a0d69116ddd32cb0c"></a>
## null_counts

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::null_counts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn null_counts(&self) -> Option<Scalar>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L297).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:297`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns null counts for all columns in this file as structured data.

<a id="op-44a34951d2e33377a33cc851"></a>
## num_records

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::num_records` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_records(&self) -> Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L285).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:285`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the raw file row count from Add-action statistics before deletion-vector filtering.

Returns `None` when the stat is missing or invalid for the current platform.

<a id="op-7616b0c1ebd02be43cb147c4"></a>
## object_store_path

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::object_store_path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn object_store_path(&self) -> Path
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L144).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:144`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

An object store [`Path`] to the file.

this tries to parse the file string and if that fails, it will return the string as is.

Unresolved upstream links (retained, not inferred): ``Path``.

<a id="op-c4759d8e0a4eb0d7e533592d"></a>
## partition_values

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partition_values(&self) -> Option<StructData>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L195).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:195`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the parsed partition values as structured data.

<a id="op-067248aac26b59379c6551b0"></a>
## partition_values_map

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::partition_values_map` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partition_values_map(&self) -> HashMap<String, Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L254).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:254`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the raw partition value map stored in the log for this file.

This preserves all partition columns even when `partitionValues_parsed` was narrowed to the
predicate-referenced subset for data skipping.

<a id="op-566c937235b9797f9e720a0f"></a>
## path

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(&self) -> Cow<'_, str>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L119).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the file path with URL decoding applied.

<a id="op-bda87e705602cc99a43b6c81"></a>
## remove_action

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::remove_action` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn remove_action(&self, data_change: bool) -> Remove
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L381).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:381`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Converts this file view into a Remove action for log operations.

<a id="op-11497c28cc139d645795905a"></a>
## size

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size(&self) -> i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L154).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:154`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the file size in bytes.

<a id="op-0447282d5c484c7f5aec0718"></a>
## stats

`function` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn stats(&self) -> Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L180).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::LogicalFileView", "path": "LogicalFileView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [396, 2], "filename": "crates/core/src/kernel/snapshot/iterators.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators.rs:180`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the raw JSON statistics string for this file, if available.

<a id="op-529f48b7fd23f68a3bc3022b"></a>
## files

`struct_field` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::files` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
files: arrow_array::RecordBatch
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L108).

Source: `crates/core/src/kernel/snapshot/iterators.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b2e2d2f94b446f4704c9643"></a>
## index

`struct_field` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView::index` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
index: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L109).

Source: `crates/core/src/kernel/snapshot/iterators.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
