# `parquet::arrow::arrow_reader::ArrowReaderMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.ArrowReaderMetadata.json).

<a id="op-c28319e4a1a2024ec909ac17"></a>
## ArrowReaderMetadata

`struct` · `parquet::arrow::arrow_reader::ArrowReaderMetadata` · parquet 59.3.0

```rust
struct ArrowReaderMetadata
```

Source: `src/arrow/arrow_reader/mod.rs:901`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The metadata necessary to construct a [`ArrowReaderBuilder`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-2cb4803ec228a018e60491ee)

Note this structure is cheaply clone-able as it consists of several arcs.

This structure allows

1. Loading metadata for a file once and then using that same metadata to
   construct multiple separate readers, for example, to distribute readers
   across multiple threads

2. Using a cached copy of the [`ParquetMetadata`] rather than reading it
   from the file each time a reader is constructed.

[`ParquetMetadata`]: crate::file::metadata::ParquetMetaData

<a id="op-0ad82fad9b14d238e23480de"></a>
## clone

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ArrowReaderMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 17], "end": [900, 22], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_reader/mod.rs:900`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee019a9f1877412a98e9c391"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 10], "end": [900, 15], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/mod.rs:900`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af27598b22047b6ad64b6068"></a>
## load

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::load` · parquet 59.3.0

```rust
fn load<T: ChunkReader>(reader: &T, options: ArrowReaderOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 1], "end": [1055, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:922`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create [`ArrowReaderMetadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-c28319e4a1a2024ec909ac17) from the provided [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4)
and [`ChunkReader`](../operations/parquet.file.reader.ChunkReader.md#op-8eae425b3b94361396eb9d94)

See [`ParquetRecordBatchReaderBuilder::new_with_metadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-d99349eb3adc8af58b40d077) for an
example of how this can be used

# Notes

If `options` has [`ArrowReaderOptions::with_page_index`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-4c872ea4ae6007c2db6ecd25) true, but
`Self::metadata` is missing the page index, this function will attempt
to load the page index by making an object store request.

<a id="op-37819abc5fa8016cb4a3a94b"></a>
## load_async

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::load_async` · parquet 59.3.0

```rust
async fn load_async<T: AsyncFileReader>(input: &mut T, options: ArrowReaderOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "crate::arrow::arrow_reader::ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [274, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:267`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a new [`ArrowReaderMetadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-c28319e4a1a2024ec909ac17) for this builder

See [`ParquetRecordBatchStreamBuilder::new_with_metadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-d99349eb3adc8af58b40d077) for how this can be used

<a id="op-673bf95b39ea7b7ada6d9ec9"></a>
## metadata

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::metadata` · parquet 59.3.0

```rust
fn metadata(&self) -> &Arc<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 1], "end": [1055, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1042`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) for this parquet file

<a id="op-0eb987fba57c20f986da3496"></a>
## parquet_schema

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::parquet_schema` · parquet 59.3.0

```rust
fn parquet_schema(&self) -> &SchemaDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 1], "end": [1055, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1047`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the parquet [`SchemaDescriptor`](../operations/parquet.schema.types.SchemaDescriptor.md#op-cb960d451851ace5640135f7) for this parquet file

<a id="op-d5a48b0df7f181e573e82ead"></a>
## schema

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::schema` · parquet 59.3.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 1], "end": [1055, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1052`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the arrow [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a) for this parquet file

<a id="op-885196b08a0102c7b2b48506"></a>
## try_new

`function` · `parquet::arrow::arrow_reader::ArrowReaderMetadata::try_new` · parquet 59.3.0

```rust
fn try_new(metadata: Arc<ParquetMetaData>, options: ArrowReaderOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderMetadata", "path": "ArrowReaderMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 1], "end": [1055, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:942`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ArrowReaderMetadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-c28319e4a1a2024ec909ac17) from a pre-existing
[`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) and [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4).

# Notes

This function will not attempt to load the PageIndex if not present in the metadata, regardless
of the settings in `options`. See [`Self::load`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-af27598b22047b6ad64b6068) to load metadata including the page index if needed.
