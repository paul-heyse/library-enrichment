# `parquet::file::metadata::reader::ParquetMetaDataReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.reader.ParquetMetaDataReader.json).

<a id="op-ecb8df4cd7e741eec25c5144"></a>
## ParquetMetaDataReader

`struct` · `parquet::file::metadata::reader::ParquetMetaDataReader` · parquet 59.3.0

```rust
struct ParquetMetaDataReader
```

Source: `src/file/metadata/reader.rs:71`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reads [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) from a byte stream, with either synchronous or
asynchronous I/O.

There are two flavors of APIs:
* Synchronous: [`Self::try_parse()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-bb655a6ce4ae9d13280d95a4), [`Self::try_parse_sized()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-003cc47f634427f8b2d4ef50), [`Self::parse_and_finish()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-4f0cd835edd69be7ad4f1989), etc.
* Asynchronous (requires `async` and `arrow` features): [`Self::try_load()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-ef050ff47f5643d64fa174b3), etc

 See the [`ParquetMetaDataPushDecoder`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-e00d43ab5b4fd446b342b085) for an API that does not require I/O.

# Format Notes

Parquet metadata is not necessarily contiguous in a Parquet file: a portion is stored
in the footer (the last bytes of the file), but other portions (such as the
PageIndex) can be stored elsewhere.
See [`crate::file::metadata::ParquetMetaDataWriter#output-format`](../operations/parquet.file.metadata.writer.ParquetMetaDataWriter.md#op-897397dc43ac19f9d052fbd0) for more details of
Parquet metadata.

This reader handles reading the footer as well as the non contiguous parts
of the metadata (`PageIndex` and `ColumnIndex`). It does not handle reading Bloom Filters.

# Example
```no_run
# use parquet::file::metadata::{PageIndexPolicy, ParquetMetaDataReader};
# fn open_parquet_file(path: &str) -> std::fs::File { unimplemented!(); }
// read parquet metadata including page indexes from a file
let file = open_parquet_file("some_path.parquet");
let mut reader = ParquetMetaDataReader::new()
    .with_page_index_policy(PageIndexPolicy::Required);
reader.try_parse(&file).unwrap();
let metadata = reader.finish().unwrap();
assert!(metadata.column_index().is_some());
assert!(metadata.offset_index().is_some());
```

<a id="op-4be6d0bc5c84e52764e44773"></a>
## decode_metadata

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::decode_metadata` · parquet 59.3.0

```rust
fn decode_metadata(buf: &[u8]) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:783`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decodes [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) from the provided bytes.

Typically this is used to decode the metadata from the end of a parquet
file. The format of `buf` is the Thrift compact binary protocol, as specified
by the [Parquet Spec].

[Parquet Spec]: https://github.com/apache/parquet-format#metadata

<a id="op-a2d935813595f41cec3b601c"></a>
## decode_metadata_with_options

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::decode_metadata_with_options` · parquet 59.3.0

```rust
fn decode_metadata_with_options(buf: &[u8], options: Option<&ParquetMetaDataOptions>) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:791`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decodes [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) from the provided bytes.

Like [`Self::decode_metadata`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-4be6d0bc5c84e52764e44773) but this also accepts
metadata parsing options.

<a id="op-4b1e597fd8dbf7707af6ab5e"></a>
## decode_schema

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::decode_schema` · parquet 59.3.0

```rust
fn decode_schema(buf: &[u8]) -> Result<Arc<SchemaDescriptor>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:800`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decodes the schema from the Parquet footer in `buf`. Returned as
a [`SchemaDescriptor`](../operations/parquet.schema.types.SchemaDescriptor.md#op-cb960d451851ace5640135f7).

<a id="op-a10e570552369877331eaed8"></a>
## default

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::default` · parquet 59.3.0

```rust
fn default() -> ParquetMetaDataReader
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 10], "end": [70, 17], "filename": "src/file/metadata/reader.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/metadata/reader.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c2f4bd1415d16c65d178e69"></a>
## finish

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::finish` · parquet 59.3.0

```rust
fn finish(&mut self) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:178`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the parsed [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) struct, leaving `None` in its place.

<a id="op-f7f90d0bf4228ecba61f9843"></a>
## fmt

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 19], "end": [70, 24], "filename": "src/file/metadata/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/reader.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d63281a8392ebd7994f34aba"></a>
## has_metadata

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::has_metadata` · parquet 59.3.0

```rust
fn has_metadata(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:173`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Indicates whether this reader has a [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) internally.

<a id="op-489a96f669d014070c870931"></a>
## load_and_finish

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::load_and_finish` · parquet 59.3.0

```rust
async fn load_and_finish<F: MetadataFetch>(self, fetch: F, file_size: u64) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:400`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Given a [`MetadataFetch`](../operations/parquet.arrow.async_reader.metadata.MetadataFetch.md#op-2f1a6adade5c3e921474d943), parse and return the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) in a single pass.

This call will consume `self`.

See [`Self::with_prefetch_hint`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-836e78360f27ee2a739d1678) for a discussion of how to reduce the number of fetches
performed by this function.

<a id="op-792bd4100e87f1c306b05f53"></a>
## load_page_index

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::load_page_index` · parquet 59.3.0

```rust
async fn load_page_index<F: MetadataFetch>(&mut self, fetch: F) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:469`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Asynchronously fetch the page index structures when a [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) has already
been obtained. See [`Self::new_with_metadata()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-1f621eeddad4ffb14049ae38).

<a id="op-576c2093ced257900b258135"></a>
## load_via_suffix_and_finish

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::load_via_suffix_and_finish` · parquet 59.3.0

```rust
async fn load_via_suffix_and_finish<F: MetadataSuffixFetch>(self, fetch: F) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:416`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Given a [`MetadataSuffixFetch`](../operations/parquet.arrow.async_reader.metadata.MetadataSuffixFetch.md#op-02be94b084ca684a67294ee4), parse and return the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) in a single pass.

This call will consume `self`.

See [`Self::with_prefetch_hint`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-836e78360f27ee2a739d1678) for a discussion of how to reduce the number of fetches
performed by this function.

<a id="op-bc20117e0e5af81ec7506755"></a>
## metadata_size

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::metadata_size` · parquet 59.3.0

```rust
fn metadata_size(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:563`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Size of the serialized thrift metadata plus the 8 byte footer. Only set if
`self.parse_metadata` is called.

<a id="op-e801454c744c6743ff0120a5"></a>
## new

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:107`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetMetaDataReader`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-ecb8df4cd7e741eec25c5144)

<a id="op-1f621eeddad4ffb14049ae38"></a>
## new_with_metadata

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::new_with_metadata` · parquet 59.3.0

```rust
fn new_with_metadata(metadata: ParquetMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:113`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetMetaDataReader`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-ecb8df4cd7e741eec25c5144) populated with a [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) struct
obtained via other means.

<a id="op-4f0cd835edd69be7ad4f1989"></a>
## parse_and_finish

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::parse_and_finish` · parquet 59.3.0

```rust
fn parse_and_finish<R: ChunkReader>(self, reader: &R) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:202`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Given a [`ChunkReader`](../operations/parquet.file.reader.ChunkReader.md#op-8eae425b3b94361396eb9d94), parse and return the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) in a single pass.

If `reader` is [`Bytes`] based, then the buffer must contain sufficient bytes to complete
the request, and must include the Parquet footer. If page indexes are desired, the buffer
must contain the entire file, or [`Self::try_parse_sized()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-003cc47f634427f8b2d4ef50) should be used.

This call will consume `self`.

# Example
```no_run
# use parquet::file::metadata::{PageIndexPolicy, ParquetMetaDataReader};
# fn open_parquet_file(path: &str) -> std::fs::File { unimplemented!(); }
// read parquet metadata including page indexes
let file = open_parquet_file("some_path.parquet");
let metadata = ParquetMetaDataReader::new()
    .with_page_index_policy(PageIndexPolicy::Required)
    .parse_and_finish(&file).unwrap();
```

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-e394b09b915f1f587c31a4ab"></a>
## read_page_indexes

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::read_page_indexes` · parquet 59.3.0

```rust
fn read_page_indexes<R: ChunkReader>(&mut self, reader: &R) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:319`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read the page index structures when a [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) has already been obtained.
See [`Self::new_with_metadata()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-1f621eeddad4ffb14049ae38) and [`Self::has_metadata()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-d63281a8392ebd7994f34aba).

<a id="op-de01d4e5bdbd02b4d60cd4f9"></a>
## read_page_indexes_sized

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::read_page_indexes_sized` · parquet 59.3.0

```rust
fn read_page_indexes_sized<R: ChunkReader>(&mut self, reader: &R, file_size: u64) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:328`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read the page index structures when a [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) has already been obtained.
This variant is used when `reader` cannot access the entire Parquet file (e.g. it is
a [`Bytes`] struct containing the tail of the file).
See [`Self::new_with_metadata()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-1f621eeddad4ffb14049ae38) and [`Self::has_metadata()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-d63281a8392ebd7994f34aba). Like
[`Self::try_parse_sized()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-003cc47f634427f8b2d4ef50) this function may return [`ParquetError::NeedMoreData`](../operations/parquet.errors.ParquetError.md#op-4738b9ae6e77ab1806f3cac9).

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-ef050ff47f5643d64fa174b3"></a>
## try_load

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::try_load` · parquet 59.3.0

```rust
async fn try_load<F: MetadataFetch>(&mut self, fetch: F, file_size: u64) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:429`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Attempts to (asynchronously) parse the footer metadata (and optionally page indexes)
given a [`MetadataFetch`](../operations/parquet.arrow.async_reader.metadata.MetadataFetch.md#op-2f1a6adade5c3e921474d943).

See [`Self::with_prefetch_hint`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-836e78360f27ee2a739d1678) for a discussion of how to reduce the number of fetches
performed by this function.

<a id="op-05a0b562c73536c5ad8c4afe"></a>
## try_load_via_suffix

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::try_load_via_suffix` · parquet 59.3.0

```rust
async fn try_load_via_suffix<F: MetadataSuffixFetch>(&mut self, fetch: F) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:449`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Attempts to (asynchronously) parse the footer metadata (and optionally page indexes)
given a [`MetadataSuffixFetch`](../operations/parquet.arrow.async_reader.metadata.MetadataSuffixFetch.md#op-02be94b084ca684a67294ee4).

See [`Self::with_prefetch_hint`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-836e78360f27ee2a739d1678) for a discussion of how to reduce the number of fetches
performed by this function.

<a id="op-bb655a6ce4ae9d13280d95a4"></a>
## try_parse

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::try_parse` · parquet 59.3.0

```rust
fn try_parse<R: ChunkReader>(&mut self, reader: &R) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:212`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Attempts to parse the footer metadata (and optionally page indexes) given a [`ChunkReader`](../operations/parquet.file.reader.ChunkReader.md#op-8eae425b3b94361396eb9d94).

If `reader` is [`Bytes`] based, then the buffer must contain sufficient bytes to complete
the request, and must include the Parquet footer. If page indexes are desired, the buffer
must contain the entire file, or [`Self::try_parse_sized()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-003cc47f634427f8b2d4ef50) should be used.

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-003cc47f634427f8b2d4ef50"></a>
## try_parse_sized

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::try_parse_sized` · parquet 59.3.0

```rust
fn try_parse_sized<R: ChunkReader>(&mut self, reader: &R, file_size: u64) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:288`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Same as [`Self::try_parse()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-bb655a6ce4ae9d13280d95a4), but provide the original file size in the case that `reader`
is a [`Bytes`] struct that does not contain the entire file. This information is necessary
when the page indexes are desired. `reader` must have access to the Parquet footer.

Using this function also allows for retrying with a larger buffer.

# Errors

This function will return [`ParquetError::NeedMoreData`](../operations/parquet.errors.ParquetError.md#op-4738b9ae6e77ab1806f3cac9) in the event `reader` does not
provide enough data to fully parse the metadata (see example below). The returned error
will be populated with a `usize` field indicating the number of bytes required from the
tail of the file to completely parse the requested metadata.

Other errors returned include [`ParquetError::General`](../operations/parquet.errors.ParquetError.md#op-aba3bc6a5e60d075362ac1d8) and [`ParquetError::EOF`](../operations/parquet.errors.ParquetError.md#op-2830373c06954b892e473eff).

# Example
```no_run
# use parquet::file::metadata::{PageIndexPolicy, ParquetMetaDataReader};
# use parquet::errors::ParquetError;
# use crate::parquet::file::reader::Length;
# fn get_bytes(file: &std::fs::File, range: std::ops::Range<u64>) -> bytes::Bytes { unimplemented!(); }
# fn open_parquet_file(path: &str) -> std::fs::File { unimplemented!(); }
let file = open_parquet_file("some_path.parquet");
let len = file.len();
// Speculatively read 1 kilobyte from the end of the file
let bytes = get_bytes(&file, len - 1024..len);
let mut reader = ParquetMetaDataReader::new().with_page_index_policy(PageIndexPolicy::Required);
match reader.try_parse_sized(&bytes, len) {
    Ok(_) => (),
    Err(ParquetError::NeedMoreData(needed)) => {
        // Read the needed number of bytes from the end of the file
        let bytes = get_bytes(&file, len - needed as u64..len);
        reader.try_parse_sized(&bytes, len).unwrap();
    }
    _ => panic!("unexpected error")
}
let metadata = reader.finish().unwrap();
```

Note that it is possible for the file metadata to be completely read, but there are
insufficient bytes available to read the page indexes. [`Self::has_metadata()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-d63281a8392ebd7994f34aba) can be used
to test for this. In the event the file metadata is present, re-parsing of the file
metadata can be skipped by using [`Self::read_page_indexes_sized()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-de01d4e5bdbd02b4d60cd4f9), as shown below.
```no_run
# use parquet::file::metadata::{PageIndexPolicy, ParquetMetaDataReader};
# use parquet::errors::ParquetError;
# use crate::parquet::file::reader::Length;
# fn get_bytes(file: &std::fs::File, range: std::ops::Range<u64>) -> bytes::Bytes { unimplemented!(); }
# fn open_parquet_file(path: &str) -> std::fs::File { unimplemented!(); }
let file = open_parquet_file("some_path.parquet");
let len = file.len();
// Speculatively read 1 kilobyte from the end of the file
let mut bytes = get_bytes(&file, len - 1024..len);
let mut reader = ParquetMetaDataReader::new().with_page_index_policy(PageIndexPolicy::Required);
// Loop until `bytes` is large enough
loop {
    match reader.try_parse_sized(&bytes, len) {
        Ok(_) => break,
        Err(ParquetError::NeedMoreData(needed)) => {
            // Read the needed number of bytes from the end of the file
            bytes = get_bytes(&file, len - needed as u64..len);
            // If file metadata was read only read page indexes, otherwise continue loop
            if reader.has_metadata() {
                reader.read_page_indexes_sized(&bytes, len).unwrap();
                break;
            }
        }
        _ => panic!("unexpected error")
    }
}
let metadata = reader.finish().unwrap();
```

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-1ea653d76c2956ec0d501f43"></a>
## with_arrow_reader_options

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::with_arrow_reader_options` · parquet 59.3.0

```rust
fn with_arrow_reader_options(self, options: Option<&ArrowReaderOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "crate::file::metadata::ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [848, 1], "end": [884, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:862`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Applies the metadata related settings from [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4),
such as the [`ParquetMetaDataOptions`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-4261537800a4fe33ff35b722), decryption properties, and
[`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) to this reader.

The page index policies are only applied if at least one of them is not
[`PageIndexPolicy::Skip`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-6f2d1d654ecec39294ec7a53), so policies previously configured on this
reader (e.g. from a preload setting) are preserved when the options do
not request the page index.

This encodes the canonical way to construct a `ParquetMetaDataReader`
inside `AsyncFileReader::get_metadata` (available with the `async`
feature), so implementations outside this crate do not need to
duplicate it.

<a id="op-ee7dfaea0fe66c38becb72fe"></a>
## with_column_index_policy

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::with_column_index_policy` · parquet 59.3.0

```rust
fn with_column_index_policy(self, policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:127`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for the column index

<a id="op-dcd64cec3c21252d3af6aa30"></a>
## with_decryption_properties

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::with_decryption_properties` · parquet 59.3.0

```rust
fn with_decryption_properties(self, properties: Option<std::sync::Arc<FileDecryptionProperties>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:164`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide the FileDecryptionProperties to use when decrypting the file.

This is only necessary when the file is encrypted.

<a id="op-217a4be87447e6e2f25fcda9"></a>
## with_metadata_options

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::with_metadata_options` · parquet 59.3.0

```rust
fn with_metadata_options(self, options: Option<ParquetMetaDataOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:139`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`ParquetMetaDataOptions`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-4261537800a4fe33ff35b722) to use when decoding

<a id="op-4f598d4e2509b33b275d347e"></a>
## with_offset_index_policy

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::with_offset_index_policy` · parquet 59.3.0

```rust
fn with_offset_index_policy(self, policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:133`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for the offset index

<a id="op-93b48192a6764f852675521b"></a>
## with_page_index_policy

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::with_page_index_policy` · parquet 59.3.0

```rust
fn with_page_index_policy(self, policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:121`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for the column and offset indexes

<a id="op-836e78360f27ee2a739d1678"></a>
## with_prefetch_hint

`function` · `parquet::file::metadata::reader::ParquetMetaDataReader::with_prefetch_hint` · parquet 59.3.0

```rust
fn with_prefetch_hint(self, prefetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::reader::ParquetMetaDataReader", "path": "ParquetMetaDataReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [803, 2], "filename": "src/file/metadata/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/reader.rs:155`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a hint as to the number of bytes needed to fully parse the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b).
Only used for the asynchronous [`Self::try_load()`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-ef050ff47f5643d64fa174b3) method.

By default, the reader will first fetch the last 8 bytes of the input file to obtain the
size of the footer metadata. A second fetch will be performed to obtain the needed bytes.
After parsing the footer metadata, a third fetch will be performed to obtain the bytes
needed to decode the page index structures, if they have been requested. To avoid
unnecessary fetches, `prefetch` can be set to an estimate of the number of bytes needed
to fully decode the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b), which can reduce the number of fetch requests and
reduce latency. Setting `prefetch` too small will not trigger an error, but will result
in extra fetches being performed.
