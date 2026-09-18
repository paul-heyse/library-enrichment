# `deltalake_core::datafile::reader::ParquetFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.reader.ParquetFileReader.json).

<a id="op-7c43616e51bb54d462d7ce6a"></a>
## ParquetFileReader

`struct` · `deltalake_core::datafile::reader::ParquetFileReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParquetFileReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L53).

Source: `crates/core/src/datafile/reader.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File-tier reader that reads a single parquet data file directly from object
storage, with no DataFusion. A concrete [`DataFileReader`](../operations/deltalake_core.datafile.DataFileReader.md#op-128d8387951106ecba58a62f) — the per-file read
seam where parquet decryption will later attach, mirroring the write side.

<a id="op-0061b1760c398546e84cd4c9"></a>
## clone

`function` · `deltalake_core::datafile::reader::ParquetFileReader::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ParquetFileReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 17], "end": [52, 22], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/reader.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47bf6de28e5392ff3c2036f7"></a>
## fmt

`function` · `deltalake_core::datafile::reader::ParquetFileReader::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/reader.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62312a05599db98d807d1a2f"></a>
## new

`function` · `deltalake_core::datafile::reader::ParquetFileReader::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(store: Arc<dyn ObjectStore>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [75, 2], "filename": "crates/core/src/datafile/reader.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/reader.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a reader over the given object store.

<a id="op-7ce7174af8eb0616b85a3551"></a>
## read_file

`function` · `deltalake_core::datafile::reader::ParquetFileReader::read_file` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read_file(&self, path: Path) -> DeltaResult<BatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L79).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [82, 2], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DataFileReader", "path": "DataFileReader"}, "trait_path": "deltalake_core::datafile::DataFileReader"}`

Source: `crates/core/src/datafile/reader.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5815cce8fe548e9627e63df4"></a>
## store

`struct_field` · `deltalake_core::datafile::reader::ParquetFileReader::store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
store: std::sync::Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L54).

Source: `crates/core/src/datafile/reader.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
