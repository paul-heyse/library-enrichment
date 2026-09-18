# `deltalake_core::datafile::datafusion_ext::DataFusionDataReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.datafusion_ext.DataFusionDataReader.json).

<a id="op-ec999f6bbf3a136a67732883"></a>
## DataFusionDataReader

`struct` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DataFusionDataReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L107).

Source: `crates/core/src/datafile/datafusion_ext.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A DataFusion-backed reader wrapping the existing `DeltaScanNext` provider.
It carries its own session so it can also satisfy [`DeltaDataReader`](../operations/deltalake_core.datafile.DeltaDataReader.md#op-3d4fa2249fef5fac72168f8d).

<a id="op-5b21f9b1f0dca7ae30da43c5"></a>
## new

`function` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(provider: Arc<dyn TableProvider>, session: Arc<dyn Session>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::DataFusionDataReader", "path": "DataFusionDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [125, 2], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/datafusion_ext.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a reader from an already-built table provider and session.

<a id="op-18e4002dfe30acb2226b639c"></a>
## read

`function` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader::read` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read(&self, options: ReadOptions) -> DeltaResult<BatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L164).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::DataFusionDataReader", "path": "DataFusionDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [168, 2], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DeltaDataReader", "path": "DeltaDataReader"}, "trait_path": "deltalake_core::datafile::DeltaDataReader"}`

Source: `crates/core/src/datafile/datafusion_ext.rs:164`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9830db9433b1bc1f087f245d"></a>
## scan

`function` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader::scan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn scan(&self, session: &dyn Session, options: ScanOptions) -> DeltaResult<SendableRecordBatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L151).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::DataFusionDataReader", "path": "DataFusionDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [160, 2], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt", "path": "DeltaDataReaderExt"}, "trait_path": "deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt"}`

Source: `crates/core/src/datafile/datafusion_ext.rs:151`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b943b8786ddc489514959de"></a>
## try_new

`function` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(table: &DeltaTable, session: Arc<dyn Session>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L120).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::datafusion_ext::DataFusionDataReader", "path": "DataFusionDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [125, 2], "filename": "crates/core/src/datafile/datafusion_ext.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/datafusion_ext.rs:120`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a reader for `table`, registering the table's object store with
`session` (idempotent) and resolving the `DeltaScanNext` provider.

<a id="op-1eeccfbfcfbdff30a035eea1"></a>
## provider

`struct_field` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader::provider` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
provider: std::sync::Arc<dyn TableProvider>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L108).

Source: `crates/core/src/datafile/datafusion_ext.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-572ea9e0f6a0b2d3d8d0f795"></a>
## session

`struct_field` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader::session` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
session: std::sync::Arc<dyn Session>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L109).

Source: `crates/core/src/datafile/datafusion_ext.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
