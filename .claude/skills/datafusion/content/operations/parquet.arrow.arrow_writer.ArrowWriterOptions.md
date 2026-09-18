# `parquet::arrow::arrow_writer::ArrowWriterOptions`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_writer.ArrowWriterOptions.json).

<a id="op-74a6b6dbd93b8f0178eea49c"></a>
## ArrowWriterOptions

`struct` · `parquet::arrow::arrow_writer::ArrowWriterOptions` · parquet 59.3.0

```rust
struct ArrowWriterOptions
```

Source: `src/arrow/arrow_writer/mod.rs:565`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Arrow-specific configuration settings for writing parquet files.

See [`ArrowWriter`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-dc781972049f13083eb8f4f3) for how to configure the writer.

<a id="op-dbf2f4b2be65687ef879572a"></a>
## clone

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ArrowWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 17], "end": [564, 22], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_writer/mod.rs:564`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fcf12524c91ad6ad98337fa"></a>
## default

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::default` · parquet 59.3.0

```rust
fn default() -> ArrowWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 24], "end": [564, 31], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow/arrow_writer/mod.rs:564`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8e2ba16164fcb957a8b8dd1"></a>
## fmt

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 10], "end": [564, 15], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_writer/mod.rs:564`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7f6e6d0c2a7e56bab944487"></a>
## new

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [708, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:575`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new [`ArrowWriterOptions`](../operations/parquet.arrow.arrow_writer.ArrowWriterOptions.md#op-74a6b6dbd93b8f0178eea49c) with the default settings.

<a id="op-b6f72a5a58f583ee71723b6b"></a>
## with_page_store_factory

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::with_page_store_factory` · parquet 59.3.0

```rust
fn with_page_store_factory(self, page_store_factory: Arc<dyn PageStoreFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [708, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:669`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`PageStoreFactory`](../operations/parquet.column.page_store.PageStoreFactory.md#op-e91a99482c4168f6c3f8f594) used to buffer completed pages while a row
group is being written.

The default implementation ([`InMemoryPageStore`](../operations/parquet.column.page_store.InMemoryPageStore.md#op-591fcf71081a9ebdf4c5471b)) buffers all completed
pages on the heap until the row group is flushed, so peak write memory
grows with the row group size. Using this API, pages can be spilled to a
file or object storage instead, reducing peak write memory substantially
at the expense of an extra write to and read from secondary storage.

# Example: spilling pages to a temp file

A simple spilling backend uses one temp file per column chunk; `put`
appends the page and `take` reads it back.

```
# use std::fs::File;
# use std::io::{Read, Seek, SeekFrom, Write};
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{ArrayRef, Int64Array, RecordBatch};
# use parquet::arrow::arrow_writer::{
#     ArrowWriter, ArrowWriterOptions, PageKey, PageStore, PageStoreArgs, PageStoreFactory,
# };
# use parquet::arrow::arrow_reader::ParquetRecordBatchReader;
# use parquet::errors::Result;
struct TempFilePageStore {
    file: File,
    /// Total size of the file
    end: u64,
    /// Location of pages: (offset, len)
    locs: Vec<(u64, usize)>,
}

impl PageStore for TempFilePageStore {
    fn put(&mut self, value: Bytes) -> Result<PageKey> {
        // Append to the end of the file
        self.file.seek(SeekFrom::Start(self.end))?;
        self.file.write_all(&value)?;
        let key = PageKey::new(self.locs.len() as u64);
        self.locs.push((self.end, value.len()));
        self.end += value.len() as u64;
        Ok(key)
    }

    fn take(&mut self, key: PageKey) -> Result<Bytes> {
        let (offset, len) = self.locs[key.get() as usize];
        let mut buf = vec![0u8; len];
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut buf)?;
        Ok(Bytes::from(buf))
    }
}

/// Factory for creating [`TempFilePageStore`]
#[derive(Debug)]
struct TempFilePageStoreFactory;

impl PageStoreFactory for TempFilePageStoreFactory {
    fn create(&self, args: &PageStoreArgs<'_>) -> Result<Box<dyn PageStore>> {
        // `args` exposes the column index and descriptor (physical/logical
        // type, path), so a real backend might choose to spill only large columns.
        let _ = (args.column_index(), args.column_descriptor());
        Ok(Box::new(TempFilePageStore {
            file: tempfile::tempfile()?, // temp file is cleaned on drop
            end: 0,
            locs: Vec::new(),
        }))
    }
}
// write 1000 integers
let col = Arc::new(Int64Array::from_iter_values(0..1000)) as ArrayRef;
let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();

let options =
    ArrowWriterOptions::new().with_page_store_factory(Arc::new(TempFilePageStoreFactory));
let mut buffer = Vec::new();
let mut writer =
    ArrowWriter::try_new_with_options(&mut buffer, to_write.schema(), options).unwrap();
writer.write(&to_write).unwrap();
writer.close().unwrap();

// buffer now holds valid Parquet data, which can be read as normal:
let mut reader = ParquetRecordBatchReader::try_new(Bytes::from(buffer), 1024).unwrap();
assert_eq!(to_write, reader.next().unwrap().unwrap());
```

<a id="op-5f4c8a22bb5faa5da9fe04be"></a>
## with_parquet_schema

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::with_parquet_schema` · parquet 59.3.0

```rust
fn with_parquet_schema(self, schema_descr: SchemaDescriptor) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [708, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:702`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Explicitly specify the Parquet schema to be used

If omitted (the default), the [`ArrowSchemaConverter`] is used to compute the
Parquet [`SchemaDescriptor`](../operations/parquet.schema.types.SchemaDescriptor.md#op-cb960d451851ace5640135f7). This may be used When the [`SchemaDescriptor`](../operations/parquet.schema.types.SchemaDescriptor.md#op-cb960d451851ace5640135f7) is
already known or must be calculated using custom logic.

Unresolved upstream links (retained, not inferred): ``ArrowSchemaConverter``.

<a id="op-ad584574fd5c5dd8cb47f5e4"></a>
## with_properties

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::with_properties` · parquet 59.3.0

```rust
fn with_properties(self, properties: WriterProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [708, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:580`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2) for writing parquet files.

<a id="op-795c743e48e2eefbcfbfdcd4"></a>
## with_schema_root

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::with_schema_root` · parquet 59.3.0

```rust
fn with_schema_root(self, schema_root: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [708, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:690`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the name of the root parquet schema element (defaults to `"arrow_schema"`)

<a id="op-b78067f38bfa66a3388b69d2"></a>
## with_skip_arrow_metadata

`function` · `parquet::arrow::arrow_writer::ArrowWriterOptions::with_skip_arrow_metadata` · parquet 59.3.0

```rust
fn with_skip_arrow_metadata(self, skip_arrow_metadata: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowWriterOptions", "path": "ArrowWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [708, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:682`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skip encoding the embedded arrow metadata (defaults to `false`)

Parquet files generated by the [`ArrowWriter`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-dc781972049f13083eb8f4f3) contain embedded arrow schema
by default.

Set `skip_arrow_metadata` to true, to skip encoding the embedded metadata.
