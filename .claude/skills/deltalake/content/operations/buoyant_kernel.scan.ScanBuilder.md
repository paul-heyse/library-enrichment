# `buoyant_kernel::scan::ScanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.ScanBuilder.json).

<a id="op-bca3273cb6882c664105ba29"></a>
## ScanBuilder

`struct` · `buoyant_kernel::scan::ScanBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ScanBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L217).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:217`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder to scan a snapshot of a table.

<a id="op-9791982ec241b4d2a4899f18"></a>
## build

`function` · `buoyant_kernel::scan::ScanBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DeltaResult<Scan>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L351).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:351`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Build the [`Scan`](../operations/buoyant_kernel.scan.Scan.md#op-34567ab1d979ca48ec5e8a90).

This does not scan the table at this point, but does do some work to ensure that the
provided schema make sense, and to prepare some metadata that the scan will need.  The
[`Scan`](../operations/buoyant_kernel.scan.Scan.md#op-34567ab1d979ca48ec5e8a90) type itself can be used to fetch the files and associated metadata required to
perform actual data reads.

<a id="op-6e88a219f8ee260c6ff8e5af"></a>
## fmt

`function` · `buoyant_kernel::scan::ScanBuilder::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L228).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:228`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86e2531b0accbe5161f63c50"></a>
## new

`function` · `buoyant_kernel::scan::ScanBuilder::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(snapshot: impl Into<SnapshotRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L242).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:242`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new [`ScanBuilder`](../operations/buoyant_kernel.scan.ScanBuilder.md#op-bca3273cb6882c664105ba29) instance.

<a id="op-79d3239c4c8f74ea9e945af4"></a>
## with_correlation_id

`function` · `buoyant_kernel::scan::ScanBuilder::with_correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L314).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:314`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attach an opaque, caller-supplied correlation id for joining this scan's metric events to
the caller's own request or operation id. An empty id is treated as unset. When unset,
behavior is unchanged.

Note: like `operation_id`, the correlation id does not currently survive the
[`Scan::parallel_scan_metadata`](../operations/buoyant_kernel.scan.Scan.md#op-b7d12bb41b8bb2ae89677147) serialization boundary. A [`ParallelState`] reconstructed
from serialized bytes on a remote worker carries no correlation id (tracked in #2736).

[`ParallelState`]: crate::scan::ParallelState

<a id="op-8c97951876f2d3ff427f71f9"></a>
## with_partition_values

`function` · `buoyant_kernel::scan::ScanBuilder::with_partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_partition_values(self, partition_values: PartitionValuesOptions) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L340).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:340`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Configure partition value output for the scan. See [`PartitionValuesOptions`](../operations/buoyant_kernel.scan.PartitionValuesOptions.md#op-c151f7b66cbf5134df7c2604).

Defaults to [`PartitionValuesOptions::default`](../operations/buoyant_kernel.scan.PartitionValuesOptions.md#op-4e14c0078331319d2836f641) (string map only). Engines that
consume `partitionValues_parsed` directly should pass
[`PartitionValuesOptions::with_struct`](../operations/buoyant_kernel.scan.PartitionValuesOptions.md#op-82ae32148b14c2c6813c2520) to also emit the typed struct column.

<a id="op-7d75fd13ee78fd24e70b0f0b"></a>
## with_predicate

`function` · `buoyant_kernel::scan::ScanBuilder::with_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_predicate(self, predicate: impl Into<Option<PredicateRef>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L290).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:290`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Optionally provide an expression to filter rows. For example, using the predicate `x <
4` to return a subset of the rows in the scan which satisfy the filter. If `predicate_opt`
is `None`, this is a no-op.

NOTE: The filtering is best-effort and can produce false positives (rows that should
have been filtered out but were kept).

NOTE: Predicates referencing metadata columns the caller added to the projection via
[`StructType::add_metadata_column`] (row indexes, row ids, file paths) are not supported
and will error at build time.

A predicate alone enables internal data skipping; kernel does not surface stats
to the engine by default. Use [`with_stats`](Self::with_stats) if the engine
also wants stats in the scan metadata output.

[`StructType::add_metadata_column`]: crate::schema::StructType::add_metadata_column

<a id="op-8c01c1286622e6490de127ce"></a>
## with_schema

`function` · `buoyant_kernel::scan::ScanBuilder::with_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema(self, logical_read_schema: SchemaRef) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L261).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:261`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provide [`Schema`] for columns to select from the [`Snapshot`].

A table with columns `[a, b, c]` could have a scan which reads only the first
two columns by using the schema `[a, b]`.

[`Schema`]: crate::schema::Schema
[`Snapshot`]: crate::snapshot::Snapshot

<a id="op-266f6156069280b85541daa7"></a>
## with_schema_opt

`function` · `buoyant_kernel::scan::ScanBuilder::with_schema_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema_opt(self, schema_opt: Option<SchemaRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L270).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:270`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Optionally provide a [`SchemaRef`](../operations/buoyant_kernel.schema.SchemaRef.md#op-bcbc708676b5e88d4183ee59) for columns to select from the [`Snapshot`]. See
[`ScanBuilder::with_schema`](../operations/buoyant_kernel.scan.ScanBuilder.md#op-8c01c1286622e6490de127ce) for details. If `schema_opt` is `None` this is a no-op.

[`Snapshot`]: crate::Snapshot

<a id="op-22528da5a54fb36bd4f6afa5"></a>
## with_stats

`function` · `buoyant_kernel::scan::ScanBuilder::with_stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_stats(self, stats: StatsOptions) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L300).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:300`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Configure stats output for the scan. See [`StatsOptions`](../operations/buoyant_kernel.scan.StatsOptions.md#op-df28d0423c58aeff877d5915).

Defaults to [`StatsOptions::default`](../operations/buoyant_kernel.scan.StatsOptions.md#op-e2323dd940c219070dabd5c0) (JSON only). Engines that consume
`stats_parsed` directly should pass [`StatsOptions::all_struct`](../operations/buoyant_kernel.scan.StatsOptions.md#op-1f3dfe4a8f9225d84366d85b) to skip the
per-batch `ToJson` synthesis on parsed-stats checkpoints.

<a id="op-54f89c9f013e444f90b8d877"></a>
## without_row_transforms

`function` · `buoyant_kernel::scan::ScanBuilder::without_row_transforms` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn without_row_transforms(self) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L330).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanBuilder", "path": "ScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:330`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Declare that the engine will reconstruct logical rows itself and will not consume
[`ScanMetadata::scan_file_transforms`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-f4a43e323d258d84029ee124).

The kernel then skips building the per-file transform expressions and the per-row
partition-value parse done only to build them. The returned `scan_file_transforms` is left
empty (each row's transform is `None`); use [`Scan::scan_metadata`](../operations/buoyant_kernel.scan.Scan.md#op-d91f731ebc1cdf3b0f6a19d8) for listing.

With this set the engine must itself apply every physical-to-logical fixup the transform
would normally perform: partition column injection, column-mapping renames, and generated
row ids. Deletion vectors are unaffected: they are delivered per file in the scan metadata
regardless. [`Scan::execute`](../operations/buoyant_kernel.scan.Scan.md#op-6dc0b3ab69a2f354c06c9103) returns an error.

<a id="op-5138a4505c3367b546b81d46"></a>
## correlation_id

`struct_field` · `buoyant_kernel::scan::ScanBuilder::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L222).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:222`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a44c225446a102e2ce177c0"></a>
## logical_read_schema

`struct_field` · `buoyant_kernel::scan::ScanBuilder::logical_read_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
logical_read_schema: Option<schema::SchemaRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L219).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:219`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ccc637f9fa19f4cb6f5d899"></a>
## partition_values

`struct_field` · `buoyant_kernel::scan::ScanBuilder::partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_values: PartitionValuesOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L224).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:224`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f023274d20887d54fd4942"></a>
## predicate

`struct_field` · `buoyant_kernel::scan::ScanBuilder::predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<expressions::PredicateRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L220).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:220`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ef6bee6aae895fb31f03a3f"></a>
## snapshot

`struct_field` · `buoyant_kernel::scan::ScanBuilder::snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L218).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:218`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5990d8c87572ebfe237dcaf"></a>
## stats

`struct_field` · `buoyant_kernel::scan::ScanBuilder::stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats: StatsOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L221).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:221`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbdf6086922ce5339fffd8e9"></a>
## without_row_transforms

`struct_field` · `buoyant_kernel::scan::ScanBuilder::without_row_transforms` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
without_row_transforms: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L223).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:223`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
