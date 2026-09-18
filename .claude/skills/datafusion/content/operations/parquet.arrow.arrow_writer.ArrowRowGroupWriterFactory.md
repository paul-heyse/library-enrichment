# `parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_writer.ArrowRowGroupWriterFactory.json).

<a id="op-94cdff34b1d4b2324c1c0d54"></a>
## ArrowRowGroupWriterFactory

`struct` · `parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory` · parquet 59.3.0

```rust
struct ArrowRowGroupWriterFactory
```

Source: `src/arrow/arrow_writer/mod.rs:1277`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Factory that creates new column writers for each row group in the Parquet file.

You can create this structure via an [`ArrowWriter::into_serialized_writer`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-6087fb00683dd3af12093f58).
See the example on [`ArrowColumnWriter`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-b2f79c40222ecdd56db8aa79) for how to encode columns in parallel

<a id="op-b35c886fb1bdfbe323669f50"></a>
## create_column_writers

`function` · `parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory::create_column_writers` · parquet 59.3.0

```rust
fn create_column_writers(&self, row_group_index: usize) -> Result<Vec<ArrowColumnWriter>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory", "path": "ArrowRowGroupWriterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1347, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:1321`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create column writers for a new row group, with the given row group index

<a id="op-a20296266d81ba7bde7c4691"></a>
## fmt

`function` · `parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory", "path": "ArrowRowGroupWriterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1276, 10], "end": [1276, 15], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_writer/mod.rs:1276`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52467870c9dcedc2722e238b"></a>
## new

`function` · `parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory::new` · parquet 59.3.0

```rust
fn new<W: Write + Send>(file_writer: &SerializedFileWriter<W>, arrow_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory", "path": "ArrowRowGroupWriterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1347, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:1288`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ArrowRowGroupWriterFactory`](../operations/parquet.arrow.arrow_writer.ArrowRowGroupWriterFactory.md#op-94cdff34b1d4b2324c1c0d54) for the provided file writer and Arrow schema

<a id="op-7fc72d28cbf2c6a9a688d043"></a>
## with_page_store_factory

`function` · `parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory::with_page_store_factory` · parquet 59.3.0

```rust
fn with_page_store_factory(self, page_store_factory: Arc<dyn PageStoreFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory", "path": "ArrowRowGroupWriterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1347, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:1307`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the [`PageStoreFactory`](../operations/parquet.column.page_store.PageStoreFactory.md#op-e91a99482c4168f6c3f8f594) used to allocate the buffer for each column
chunk, e.g. to spill completed pages to a temp file or object storage
instead of the heap. Defaults to [`InMemoryPageStoreFactory`](../operations/parquet.column.page_store.InMemoryPageStoreFactory.md#op-d9200fd88c6a992e0a79494d).
