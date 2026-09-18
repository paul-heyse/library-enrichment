# `datafusion_catalog::stream::FileStreamProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.stream.FileStreamProvider.json).

<a id="op-0188029627a6d69bf313cbfc"></a>
## FileStreamProvider

`struct` · `datafusion_catalog::stream::FileStreamProvider` · datafusion-catalog 55.1.0

```rust
struct FileStreamProvider
```

Source: `src/stream.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Stream data from the file at `location`

* Data will be read sequentially from the provided `location`
* New data will be appended to the end of the file

The encoding can be configured with [`Self::with_encoding`](../operations/datafusion_catalog.stream.FileStreamProvider.md#op-eeba55f63392a4fa07568029) and
defaults to [`StreamEncoding::Csv`](../operations/datafusion_catalog.stream.StreamEncoding.md#op-4887e84e26047f5b1855bdb7)

<a id="op-c46b4116b672e980e5c1a2ce"></a>
## fmt

`function` · `datafusion_catalog::stream::FileStreamProvider::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 10], "end": [140, 15], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stream.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6eb5588e2363e5b5ba5ffde6"></a>
## new_file

`function` · `datafusion_catalog::stream::FileStreamProvider::new_file` · datafusion-catalog 55.1.0

```rust
fn new_file(schema: SchemaRef, location: PathBuf) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [185, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Stream data from the file at `location`

* Data will be read sequentially from the provided `location`
* New data will be appended to the end of the file

The encoding can be configured with [`Self::with_encoding`](../operations/datafusion_catalog.stream.FileStreamProvider.md#op-eeba55f63392a4fa07568029) and
defaults to [`StreamEncoding::Csv`](../operations/datafusion_catalog.stream.StreamEncoding.md#op-4887e84e26047f5b1855bdb7)

<a id="op-5c7f74d0412c9e98712ab0ac"></a>
## reader

`function` · `datafusion_catalog::stream::FileStreamProvider::reader` · datafusion-catalog 55.1.0

```rust
fn reader(&self) -> Result<Box<dyn RecordBatchReader>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [250, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_catalog::stream::StreamProvider", "path": "StreamProvider"}, "trait_path": "datafusion_catalog::stream::StreamProvider"}`

Source: `src/stream.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-863bb5904aa43680bab7fd51"></a>
## schema

`function` · `datafusion_catalog::stream::FileStreamProvider::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [250, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_catalog::stream::StreamProvider", "path": "StreamProvider"}, "trait_path": "datafusion_catalog::stream::StreamProvider"}`

Source: `src/stream.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7f14be0645490e67f9aec9e"></a>
## schema

`struct_field` · `datafusion_catalog::stream::FileStreamProvider::schema` · datafusion-catalog 55.1.0

```rust
schema: arrow::datatypes::SchemaRef
```

Source: `src/stream.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Get a reference to the schema for this file stream

<a id="op-94b5692c37d7d08f445e7339"></a>
## stream_write_display

`function` · `datafusion_catalog::stream::FileStreamProvider::stream_write_display` · datafusion-catalog 55.1.0

```rust
fn stream_write_display(&self, _t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [250, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_catalog::stream::StreamProvider", "path": "StreamProvider"}, "trait_path": "datafusion_catalog::stream::StreamProvider"}`

Source: `src/stream.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b418e3be4bec8c615495aae"></a>
## with_batch_size

`function` · `datafusion_catalog::stream::FileStreamProvider::with_batch_size` · datafusion-catalog 55.1.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [185, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Set the batch size (the number of rows to load at one time)

<a id="op-eeba55f63392a4fa07568029"></a>
## with_encoding

`function` · `datafusion_catalog::stream::FileStreamProvider::with_encoding` · datafusion-catalog 55.1.0

```rust
fn with_encoding(self, encoding: StreamEncoding) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [185, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Specify an encoding for the stream

<a id="op-da90a25914e11c76d9dab370"></a>
## with_header

`function` · `datafusion_catalog::stream::FileStreamProvider::with_header` · datafusion-catalog 55.1.0

```rust
fn with_header(self, header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [185, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Specify whether the file has a header (only applicable for [`StreamEncoding::Csv`](../operations/datafusion_catalog.stream.StreamEncoding.md#op-4887e84e26047f5b1855bdb7))

<a id="op-c8f9b106b2ff5770b7b99c0f"></a>
## writer

`function` · `datafusion_catalog::stream::FileStreamProvider::writer` · datafusion-catalog 55.1.0

```rust
fn writer(&self) -> Result<Box<dyn RecordBatchWriter>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::FileStreamProvider", "path": "FileStreamProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [250, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_catalog::stream::StreamProvider", "path": "StreamProvider"}, "trait_path": "datafusion_catalog::stream::StreamProvider"}`

Source: `src/stream.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
