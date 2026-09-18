# `deltalake_core::kernel::snapshot::Snapshot`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.Snapshot.json).

<a id="op-ca78a516c68e7586104c0136"></a>
## Snapshot

`struct` · `deltalake_core::kernel::snapshot::Snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Snapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L212).

Source: `crates/core/src/kernel/snapshot/mod.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of a Delta table

<a id="op-3710d2f3dbaab9c203fa153c"></a>
## arrow_schema

`function` · `deltalake_core::kernel::snapshot::Snapshot::arrow_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_schema(&self) -> SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L413).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:413`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Convert the lgoical schema into an Arrow [SchemaRef]

NOTE: This can panic if the table's logical schema is not compatible with Arrow!

Unresolved upstream links (retained, not inferred): `SchemaRef`.

<a id="op-af2d5e496c54128e407a764a"></a>
## clone

`function` · `deltalake_core::kernel::snapshot::Snapshot::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Snapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L211).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 17], "end": [211, 22], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/snapshot/mod.rs:211`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-388a91822174c8b751909d7b"></a>
## deserialize

`function` · `deltalake_core::kernel::snapshot::Snapshot::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/serde.rs#L456).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "super::Snapshot"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [462, 2], "filename": "crates/core/src/kernel/snapshot/serde.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/snapshot/serde.rs:456`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0de14b0458be8495f501f06"></a>
## domain_metadata

`function` · `deltalake_core::kernel::snapshot::Snapshot::domain_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn domain_metadata(&self, log_store: &dyn LogStore, domain: impl ToString) -> DeltaResult<Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L1141).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:1141`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Fetch the [domainMetadata] for a specific domain in this snapshot.

This returns the latest configuration for the domain, or None if the domain does not exist.

[domainMetadata]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#domain-metadata

<a id="op-0f89d5d2af75d6403617ad2d"></a>
## eq

`function` · `deltalake_core::kernel::snapshot::Snapshot::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Snapshot) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L211).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 24], "end": [211, 33], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/snapshot/mod.rs:211`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0863d990339791fca9903f3f"></a>
## file_views

`function` · `deltalake_core::kernel::snapshot::Snapshot::file_views` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_views(&self, log_store: &dyn LogStore, predicate: Option<PredicateRef>) -> BoxStream<'_, DeltaResult<LogicalFileView>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L918).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:918`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stream the active files in the snapshot

This function returns a stream of [`LogicalFileView`](../operations/deltalake_core.kernel.snapshot.iterators.LogicalFileView.md#op-f5a43d85036c73510b6d2a44) objects,
which represent the active files in the snapshot.

## Parameters

* `log_store` - A reference to a [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) implementation.
* `predicate` - An optional predicate to filter the files.

## Returns

A stream of [`LogicalFileView`](../operations/deltalake_core.kernel.snapshot.iterators.LogicalFileView.md#op-f5a43d85036c73510b6d2a44) objects, newest first.

<a id="op-5e3f1e475a903e3fbc56409d"></a>
## files

`function` · `deltalake_core::kernel::snapshot::Snapshot::files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn files(&self, log_store: &dyn LogStore, predicate: Option<PredicateRef>) -> SendableRBStream
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L607).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:607`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the active files for the current snapshot.

This method returns a stream of record batches where each row
represents an active file for the current snapshot.

The files can be filtered using the provided predicate. This is a
best effort to skip files that are excluded by the predicate. Individual
files may still contain data that is not relevant to the predicate.

## Arguments

* `log_store` - The log store to use for reading the snapshot.
* `predicate` - An optional predicate to filter the files.

## Returns

A stream of active files for the current snapshot.

<a id="op-e409a97a46851fc52f29de30"></a>
## fmt

`function` · `deltalake_core::kernel::snapshot::Snapshot::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L211).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 10], "end": [211, 15], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/snapshot/mod.rs:211`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4489dd9db9739717f50e6fd7"></a>
## input_schema

`function` · `deltalake_core::kernel::snapshot::Snapshot::input_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn input_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L155).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "crate::kernel::Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [171, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23aa7403e432f9d203831cd1"></a>
## into_scan_builder

`function` · `deltalake_core::kernel::snapshot::Snapshot::into_scan_builder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_scan_builder(self) -> ScanBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L286).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:286`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Consume this snapshot and create a [`ScanBuilder`](../operations/deltalake_core.kernel.snapshot.scan.ScanBuilder.md#op-a287f3a037e75c6307dca12c) that owns the underlying state.

<a id="op-3da8c5d64c6406e8eb782c7e"></a>
## load_config

`function` · `deltalake_core::kernel::snapshot::Snapshot::load_config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn load_config(&self) -> &DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L433).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:433`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table config which is loaded with of the snapshot

<a id="op-2a6bc9f4a3b36d08c24b4300"></a>
## metadata

`function` · `deltalake_core::kernel::snapshot::Snapshot::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L423).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:423`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table metadata of the snapshot

<a id="op-c4b633ce84d77a02edffd5c2"></a>
## parse_predicate_expression

`function` · `deltalake_core::kernel::snapshot::Snapshot::parse_predicate_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L163).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "crate::kernel::Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [171, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-990eb8ea04ccac9d2459d6f0"></a>
## protocol

`function` · `deltalake_core::kernel::snapshot::Snapshot::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L428).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:428`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table protocol of the snapshot

<a id="op-8c1009bfcbefcf42782c810e"></a>
## read_schema

`function` · `deltalake_core::kernel::snapshot::Snapshot::read_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L147).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "crate::kernel::Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [171, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:147`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b44f76c296d67cdcff2eb65"></a>
## scan_builder

`function` · `deltalake_core::kernel::snapshot::Snapshot::scan_builder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_builder(&self) -> ScanBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L281).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:281`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a [`ScanBuilder`](../operations/deltalake_core.kernel.snapshot.scan.ScanBuilder.md#op-a287f3a037e75c6307dca12c) borrowing this snapshot to configure a read of the table.

<a id="op-a4f48fbd35e37622341358d6"></a>
## schema

`function` · `deltalake_core::kernel::snapshot::Snapshot::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> KernelSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L406).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:406`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the logical table schema of the snapshot

<a id="op-79fab6945d12ec91b28e867a"></a>
## serialize

`function` · `deltalake_core::kernel::snapshot::Snapshot::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/serde.rs#L230).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "super::Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [295, 2], "filename": "crates/core/src/kernel/snapshot/serde.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/snapshot/serde.rs:230`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd6942ad65c3b48d7e2527f2"></a>
## table_configuration

`function` · `deltalake_core::kernel::snapshot::Snapshot::table_configuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_configuration(&self) -> &TableConfiguration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L577).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:577`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the resolved [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae) (protocol, metadata and parsed properties).

<a id="op-b07a0b158134b5d1848e4336"></a>
## table_properties

`function` · `deltalake_core::kernel::snapshot::Snapshot::table_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_properties(&self) -> &TableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L572).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:572`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Well known properties of the table

<a id="op-4b1252b13ce683654eb633da"></a>
## try_new

`function` · `deltalake_core::kernel::snapshot::Snapshot::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(log_store: &dyn LogStore, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L261).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:261`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`Snapshot`](../operations/deltalake_core.kernel.snapshot.Snapshot.md#op-ca78a516c68e7586104c0136) instance

<a id="op-1fe49f981f3fe7e8faa118e2"></a>
## try_new_with_engine

`function` · `deltalake_core::kernel::snapshot::Snapshot::try_new_with_engine` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new_with_engine(engine: Arc<dyn Engine>, table_root: Url, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L226).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:226`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a snapshot of the table at `table_root` using the provided kernel `engine`.

When `version` is `None` the latest available version is loaded. This is the engine-aware
constructor used when callers want to control the kernel [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) backing log replay.

<a id="op-3ca0aabc70ff551314858d24"></a>
## update

`function` · `deltalake_core::kernel::snapshot::Snapshot::update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn update(Arc<self>, engine: Arc<dyn Engine>, target_version: Option<Version>) -> DeltaResult<Arc<Self>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L291).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:291`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update the snapshot to the given version

<a id="op-a91d0e302bfe0be2d0820e4c"></a>
## version

`function` · `deltalake_core::kernel::snapshot::Snapshot::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L396).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [1155, 2], "filename": "crates/core/src/kernel/snapshot/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/mod.rs:396`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table version of the snapshot

<a id="op-3818b17e35403fbde134031b"></a>
## config

`struct_field` · `deltalake_core::kernel::snapshot::Snapshot::config` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
config: DeltaTableConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L216).

Source: `crates/core/src/kernel/snapshot/mod.rs:216`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration for the current session

<a id="op-433b15b2c5957f543ea06958"></a>
## inner

`struct_field` · `deltalake_core::kernel::snapshot::Snapshot::inner` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<delta_kernel::snapshot::Snapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L214).

Source: `crates/core/src/kernel/snapshot/mod.rs:214`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Log segment containing all log files in the snapshot

<a id="op-d84a29ae10bc96774d53561f"></a>
## materialized_files

`struct_field` · `deltalake_core::kernel::snapshot::Snapshot::materialized_files` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
materialized_files: Option<std::sync::Arc<MaterializedFiles>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/mod.rs#L218).

Source: `crates/core/src/kernel/snapshot/mod.rs:218`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optional materialized replay state owned by this snapshot.
