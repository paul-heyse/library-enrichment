# `deltalake_core::logstore::parquet_reader::ParquetObjectReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.parquet_reader.ParquetObjectReader.json).

<a id="op-4dfcbd4d53ecdc0ab58a373e"></a>
## ParquetObjectReader

`struct` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParquetObjectReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L42).

Source: `crates/core/src/logstore/parquet_reader.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Reads Parquet files in object storage using [`ObjectStore`].

This struct provides a simple implementation of [`AsyncFileReader`] that
can be used with [`ParquetRecordBatchStreamBuilder`].

# Example

```no_run
# use std::sync::Arc;
# use deltalake_core::logstore::parquet_reader::ParquetObjectReader;
# use object_store::{ObjectStore, path::Path};
# use parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder;
# async fn run() {
# let store: Arc<dyn ObjectStore> = todo!();
# let location: Path = todo!();
# let file_size: u64 = todo!();
let reader = ParquetObjectReader::new(store, location).with_file_size(file_size);
let builder = ParquetRecordBatchStreamBuilder::new(reader).await.unwrap();
# }
```

Unresolved upstream links (retained, not inferred): ``AsyncFileReader``, ``ObjectStore``.

<a id="op-0bf34fa86e29e342508aa0f5"></a>
## clone

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ParquetObjectReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/parquet_reader.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb7963547864eb2265d26d8a"></a>
## fmt

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 22], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/parquet_reader.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28188b282a098df194936b41"></a>
## get_bytes

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::get_bytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L134).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `crates/core/src/logstore/parquet_reader.rs:134`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c72cfde30145e077a85b955"></a>
## get_metadata

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::get_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `crates/core/src/logstore/parquet_reader.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef86e59ca5f307b3f982f870"></a>
## new

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [107, 2], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/parquet_reader.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a new [`ParquetObjectReader`](../operations/deltalake_core.logstore.parquet_reader.ParquetObjectReader.md#op-4dfcbd4d53ecdc0ab58a373e) for the provided [`ObjectStore`] and [`Path`].

Unresolved upstream links (retained, not inferred): ``ObjectStore``, ``Path``.

<a id="op-ec3826880fd934a04841bbbc"></a>
## with_file_size

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::with_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_size(self, file_size: u64) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [107, 2], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/parquet_reader.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide a byte size of this file.

If provided, the file size will ensure that only bounded range requests are used. If file
size is not provided, the reader will use suffix range requests to fetch the metadata.

Providing this size up front is an important optimization to avoid extra calls when the
underlying store does not support suffix range requests.

<a id="op-6dff43359461056c7ff06d55"></a>
## with_footer_size_hint

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::with_footer_size_hint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_footer_size_hint(self, hint: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [107, 2], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/parquet_reader.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide a hint as to the size of the parquet file's footer.

<a id="op-ac18c3181723194dfcfc45f1"></a>
## with_preload_column_index

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::with_preload_column_index` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_preload_column_index(self, preload: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L93).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [107, 2], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/parquet_reader.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets whether to preload the column index.

<a id="op-09233a5716b6609cc016c34f"></a>
## with_preload_offset_index

`function` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::with_preload_offset_index` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_preload_offset_index(self, preload: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::parquet_reader::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [107, 2], "filename": "crates/core/src/logstore/parquet_reader.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/parquet_reader.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets whether to preload the offset index.

<a id="op-091183adf28c884754a52f60"></a>
## file_size

`struct_field` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::file_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_size: Option<u64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L48).

Source: `crates/core/src/logstore/parquet_reader.rs:48`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The file size, if known

<a id="op-2ee05466c8741c983cc0b49a"></a>
## metadata_size_hint

`struct_field` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::metadata_size_hint` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metadata_size_hint: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L50).

Source: `crates/core/src/logstore/parquet_reader.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The metadata size hint, if provided

<a id="op-b2cf284451e57b2213977a14"></a>
## path

`struct_field` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::path` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: object_store::path::Path
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L46).

Source: `crates/core/src/logstore/parquet_reader.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The path to the file

<a id="op-41c21922a138f3c852f1f1e1"></a>
## preload_column_index

`struct_field` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::preload_column_index` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
preload_column_index: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L52).

Source: `crates/core/src/logstore/parquet_reader.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to preload the column index

<a id="op-7645a52da4100ef18f5fecfc"></a>
## preload_offset_index

`struct_field` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::preload_offset_index` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
preload_offset_index: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L54).

Source: `crates/core/src/logstore/parquet_reader.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to preload the offset index

<a id="op-213bf9e08295fc9b162cc43d"></a>
## store

`struct_field` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader::store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
store: std::sync::Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L44).

Source: `crates/core/src/logstore/parquet_reader.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The ObjectStore instance to use for reading
