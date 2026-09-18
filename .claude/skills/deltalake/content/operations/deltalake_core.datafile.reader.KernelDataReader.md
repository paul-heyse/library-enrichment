# `deltalake_core::datafile::reader::KernelDataReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.reader.KernelDataReader.json).

<a id="op-8db980494e2cd454b717a106"></a>
## KernelDataReader

`struct` · `deltalake_core::datafile::reader::KernelDataReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct KernelDataReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L216).

Source: `crates/core/src/datafile/reader.rs:216`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Dataset-tier reader backed by `delta-kernel`'s scan engine (placeholder).

Unlike [`ParquetTableReader`](../operations/deltalake_core.datafile.reader.ParquetTableReader.md#op-0befbd1bf96f43c3cfdb22bf), this will apply deletion vectors, partition
values, and column-mapping transforms — the full Delta read semantics.

<a id="op-5c9a8f33a67d9f9b1ce3b13c"></a>
## clone

`function` · `deltalake_core::datafile::reader::KernelDataReader::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> KernelDataReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataReader", "path": "KernelDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 17], "end": [215, 22], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/reader.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b537faaaf3d96cead09c4d46"></a>
## default

`function` · `deltalake_core::datafile::reader::KernelDataReader::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> KernelDataReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataReader", "path": "KernelDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 24], "end": [215, 31], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/datafile/reader.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e919ac87966691f065eae0e"></a>
## fmt

`function` · `deltalake_core::datafile::reader::KernelDataReader::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataReader", "path": "KernelDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 10], "end": [215, 15], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/reader.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86400ce332a968495d71529d"></a>
## read

`function` · `deltalake_core::datafile::reader::KernelDataReader::read` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read(&self, _options: ReadOptions) -> DeltaResult<BatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L220).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataReader", "path": "KernelDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [223, 2], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DeltaDataReader", "path": "DeltaDataReader"}, "trait_path": "deltalake_core::datafile::DeltaDataReader"}`

Source: `crates/core/src/datafile/reader.rs:220`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
