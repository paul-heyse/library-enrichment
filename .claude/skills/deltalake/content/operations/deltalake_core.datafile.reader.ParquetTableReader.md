# `deltalake_core::datafile::reader::ParquetTableReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.reader.ParquetTableReader.json).

<a id="op-0befbd1bf96f43c3cfdb22bf"></a>
## ParquetTableReader

`struct` · `deltalake_core::datafile::reader::ParquetTableReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParquetTableReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L95).

Source: `crates/core/src/datafile/reader.rs:95`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Dataset-tier reader that reads all of a table's parquet data files directly —
no DataFusion, no predicate — composing [`ParquetFileReader`](../operations/deltalake_core.datafile.reader.ParquetFileReader.md#op-7c43616e51bb54d462d7ce6a) across the table.

Minimal by design: it reads raw parquet, so [`ParquetTableReader::try_new`](../operations/deltalake_core.datafile.reader.ParquetTableReader.md#op-5d617dcd89f16e40fc6d34b5)
rejects tables that use deletion vectors, column mapping, or partition columns
(those are the kernel-backed [`KernelDataReader`](../operations/deltalake_core.datafile.reader.KernelDataReader.md#op-8db980494e2cd454b717a106)'s job), and it does not unify
schemas across files — returned batches reflect each file's physical schema as
written. On a table whose schema evolved, older and newer files yield batches
with differing schemas, and the caller must reconcile them (e.g. cast to a
common schema before `concat`).

<a id="op-221c9b78a71fa9468d5d22f0"></a>
## clone

`function` · `deltalake_core::datafile::reader::ParquetTableReader::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ParquetTableReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L94).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetTableReader", "path": "ParquetTableReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 17], "end": [94, 22], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/reader.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-080d1f1e6f7a7ec0ee7822c7"></a>
## fmt

`function` · `deltalake_core::datafile::reader::ParquetTableReader::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L94).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetTableReader", "path": "ParquetTableReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 10], "end": [94, 15], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/reader.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5178d933bd2eb98275ec05a"></a>
## read

`function` · `deltalake_core::datafile::reader::ParquetTableReader::read` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read(&self, options: ReadOptions) -> DeltaResult<BatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L154).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetTableReader", "path": "ParquetTableReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [181, 2], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DeltaDataReader", "path": "DeltaDataReader"}, "trait_path": "deltalake_core::datafile::DeltaDataReader"}`

Source: `crates/core/src/datafile/reader.rs:154`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d617dcd89f16e40fc6d34b5"></a>
## try_new

`function` · `deltalake_core::datafile::reader::ParquetTableReader::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(table: &DeltaTable) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L107).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetTableReader", "path": "ParquetTableReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [150, 2], "filename": "crates/core/src/datafile/reader.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/reader.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a reader over the table's current data files.

Errors if the table uses a feature this raw reader cannot honor
(deletion vectors, column mapping, or partition columns).

<a id="op-94800f6a3bcaa7e4cad894eb"></a>
## file_reader

`struct_field` · `deltalake_core::datafile::reader::ParquetTableReader::file_reader` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_reader: ParquetFileReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L96).

Source: `crates/core/src/datafile/reader.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-388183daa7fed08d724e6981"></a>
## files

`struct_field` · `deltalake_core::datafile::reader::ParquetTableReader::files` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
files: std::sync::Arc<[(object_store::path::Path, u64)]>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L99).

Source: `crates/core/src/datafile/reader.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Data files as (object-store path, size in bytes); `Arc` so `read`/`Clone`
share the list instead of deep-copying every path.
