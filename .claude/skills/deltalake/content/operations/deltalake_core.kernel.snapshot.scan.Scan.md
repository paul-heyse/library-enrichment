# `deltalake_core::kernel::snapshot::scan::Scan`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.scan.Scan.json).

<a id="op-ef5be9790dda876312ea948b"></a>
## Scan

`struct` · `deltalake_core::kernel::snapshot::scan::Scan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Scan
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L389).

Source: `crates/core/src/kernel/snapshot/scan.rs:389`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A configured, executable scan over a table snapshot.

Produced by [`ScanBuilder::build`](../operations/deltalake_core.kernel.snapshot.scan.ScanBuilder.md#op-f96293b28ce16cb63f815ada); drives log replay to enumerate the data files (and the
statistics materialization strategy) that satisfy the scan's schema and predicate.

<a id="op-689a6b097603b6b319b4c9fe"></a>
## fmt

`function` · `deltalake_core::kernel::snapshot::scan::Scan::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L388).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [388, 10], "end": [388, 15], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/snapshot/scan.rs:388`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-241d2cc14326d64830f4246c"></a>
## from

`function` · `deltalake_core::kernel::snapshot::scan::Scan::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(inner: Arc<KernelScan>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L404).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [410, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/kernel/snapshot/scan.rs:404`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-678849945edae8aaf7896a69"></a>
## from

`function` · `deltalake_core::kernel::snapshot::scan::Scan::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(inner: KernelScan) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L395).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [401, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::Scan", "path": "Scan"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/kernel/snapshot/scan.rs:395`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef469a59137052f6ca628a52"></a>
## logical_schema

`function` · `deltalake_core::kernel::snapshot::scan::Scan::logical_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L455).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [575, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:455`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a shared reference to the logical [`Schema`] of the scan (i.e. the output schema of the
scan). Note that the logical schema can differ from the physical schema due to e.g.
partition columns which are present in the logical schema but not in the physical schema.

[`Schema`]: crate::schema::Schema

<a id="op-6c90acbbc4c45c01ac56c76e"></a>
## physical_predicate

`function` · `deltalake_core::kernel::snapshot::scan::Scan::physical_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_predicate(&self) -> Option<PredicateRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L469).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [575, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:469`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the predicate [`PredicateRef`](../operations/buoyant_kernel.expressions.PredicateRef.md#op-70ac26119927823d3f871776) of the scan.
Returns the predicate pushed down to the physical scan, if any was derived.

<a id="op-dc8fe242e854d9a1622d8c71"></a>
## physical_schema

`function` · `deltalake_core::kernel::snapshot::scan::Scan::physical_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L463).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [575, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:463`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a shared reference to the physical [`Schema`] of the scan. This represents the schema
of the underlying data files which must be read from storage.

[`Schema`]: crate::schema::Schema

<a id="op-ca1a283d244ae97d445edff7"></a>
## scan_metadata

`function` · `deltalake_core::kernel::snapshot::scan::Scan::scan_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_metadata(&self, engine: Arc<dyn Engine>) -> SendableScanMetadataStream
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L474).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [575, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:474`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stream the per-file [`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2) for this scan, driving log replay on `engine`.

<a id="op-83d9571e35d85d71299c6021"></a>
## scan_metadata_from

`function` · `deltalake_core::kernel::snapshot::scan::Scan::scan_metadata_from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_metadata_from<T: Iterator<Item = RecordBatch> + Send + 'static>(&self, engine: Arc<dyn Engine>, existing_version: Version, existing_data: Box<T>, existing_predicate: Option<PredicateRef>) -> SendableScanMetadataStream
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L520).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [575, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:520`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stream [`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2) incrementally starting from a previously observed state.

Given the data already known at `existing_version` (and the predicate used to produce it),
only the log changes since that version are replayed, avoiding a full scan when refreshing
an already-loaded snapshot.

<a id="op-0c5026ae6d4c82c02b10a09f"></a>
## snapshot

`function` · `deltalake_core::kernel::snapshot::scan::Scan::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn snapshot(&self) -> &SnapshotRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L446).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [575, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:446`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a shared reference to the [`Snapshot`] of this scan.

[`Snapshot`]: crate::Snapshot

<a id="op-8e518627dd58f2fee2790dd1"></a>
## table_root

`function` · `deltalake_core::kernel::snapshot::scan::Scan::table_root` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L439).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::scan::Scan", "path": "Scan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [575, 2], "filename": "crates/core/src/kernel/snapshot/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/scan.rs:439`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The table's root URL. Any relative paths returned from `scan_data` (or in a callback from
[`ScanMetadata::visit_scan_files`]) must be resolved against this root to get the actual path to
the file.

[`ScanMetadata::visit_scan_files`]: crate::scan::ScanMetadata::visit_scan_files

<a id="op-aad25f91e2367a49ba7b6384"></a>
## inner

`struct_field` · `deltalake_core::kernel::snapshot::scan::Scan::inner` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<delta_kernel::scan::Scan>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L390).

Source: `crates/core/src/kernel/snapshot/scan.rs:390`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dff6f83e9172bbada2f2c8d0"></a>
## stats_materialization

`struct_field` · `deltalake_core::kernel::snapshot::scan::Scan::stats_materialization` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats_materialization: super::stats_projection::FileStatsMaterialization
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L391).

Source: `crates/core/src/kernel/snapshot/scan.rs:391`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
