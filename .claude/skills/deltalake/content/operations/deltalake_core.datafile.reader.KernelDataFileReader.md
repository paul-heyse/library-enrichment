# `deltalake_core::datafile::reader::KernelDataFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.reader.KernelDataFileReader.json).

<a id="op-fec287867c89e5c2414ac5f7"></a>
## KernelDataFileReader

`struct` · `deltalake_core::datafile::reader::KernelDataFileReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct KernelDataFileReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L202).

Source: `crates/core/src/datafile/reader.rs:202`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File-tier reader backed by `delta-kernel`'s parquet handler (placeholder).

<a id="op-2fccea44e2de03ae05a7718a"></a>
## clone

`function` · `deltalake_core::datafile::reader::KernelDataFileReader::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> KernelDataFileReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L201).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataFileReader", "path": "KernelDataFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 17], "end": [201, 22], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/reader.rs:201`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb96f636aab75882cbdfa3f5"></a>
## default

`function` · `deltalake_core::datafile::reader::KernelDataFileReader::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> KernelDataFileReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L201).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataFileReader", "path": "KernelDataFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 24], "end": [201, 31], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/datafile/reader.rs:201`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8844964fa9397cca0166ada3"></a>
## fmt

`function` · `deltalake_core::datafile::reader::KernelDataFileReader::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L201).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataFileReader", "path": "KernelDataFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 10], "end": [201, 15], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/reader.rs:201`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47141808973fd799a0e2c481"></a>
## read_file

`function` · `deltalake_core::datafile::reader::KernelDataFileReader::read_file` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read_file(&self, _path: Path) -> DeltaResult<BatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L206).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::reader::KernelDataFileReader", "path": "KernelDataFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [209, 2], "filename": "crates/core/src/datafile/reader.rs"}, "trait": {"args": null, "id": "deltalake_core::datafile::DataFileReader", "path": "DataFileReader"}, "trait_path": "deltalake_core::datafile::DataFileReader"}`

Source: `crates/core/src/datafile/reader.rs:206`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
