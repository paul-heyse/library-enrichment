# `arrow_ipc::reader::RecordBatchDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.RecordBatchDecoder.json).

<a id="op-f22d90eb994c3c45a8db1727"></a>
## RecordBatchDecoder

`struct` · `arrow_ipc::reader::RecordBatchDecoder` · arrow-ipc 59.3.0

```rust
struct RecordBatchDecoder<'a>
```

Source: `src/reader.rs:444`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

State for decoding Arrow arrays from an [IPC RecordBatch] structure to
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

[IPC RecordBatch]: crate::RecordBatch


<a id="op-aca2cba57194ecbfe2df6dda"></a>
## read_record_batch

`function` · `arrow_ipc::reader::RecordBatchDecoder::read_record_batch` · arrow-ipc 59.3.0

```rust
fn read_record_batch(self) -> Result<RecordBatch, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::reader::RecordBatchDecoder", "path": "RecordBatchDecoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [755, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:548`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Read the record batch, consuming the reader

<a id="op-92b4769c85c09a5f039bb296"></a>
## try_new

`function` · `arrow_ipc::reader::RecordBatchDecoder::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(buf: &'a Buffer, batch: RecordBatch<'a>, schema: SchemaRef, dictionaries_by_id: &'a HashMap<i64, ArrayRef>, metadata: &'a MetadataVersion) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::reader::RecordBatchDecoder", "path": "RecordBatchDecoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [755, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:477`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Create a reader for decoding arrays from an encoded [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

<a id="op-eacdaa3b85ca050335f83217"></a>
## with_projection

`function` · `arrow_ipc::reader::RecordBatchDecoder::with_projection` · arrow-ipc 59.3.0

```rust
fn with_projection(self, projection: Option<&'a [usize]>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::reader::RecordBatchDecoder", "path": "RecordBatchDecoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [755, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:516`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Set the projection (default: None)

If set, the projection is the list  of column indices
that will be read

<a id="op-782e442abb302cf5ad807838"></a>
## with_require_alignment

`function` · `arrow_ipc::reader::RecordBatchDecoder::with_require_alignment` · arrow-ipc 59.3.0

```rust
fn with_require_alignment(self, require_alignment: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::reader::RecordBatchDecoder", "path": "RecordBatchDecoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [755, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:526`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Set require_alignment (default: false)

If true, buffers must be aligned appropriately or error will
result. If false, buffers will be copied to aligned buffers
if necessary.

<a id="op-906bb756e2a148abe7fb39a0"></a>
## with_skip_validation

`function` · `arrow_ipc::reader::RecordBatchDecoder::with_skip_validation` · arrow-ipc 59.3.0

```rust
fn with_skip_validation(self, skip_validation: UnsafeFlag) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::reader::RecordBatchDecoder", "path": "RecordBatchDecoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [755, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:542`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specifies if validation should be skipped when reading data (defaults to `false`)

When enabled, the following checks are bypassed:
- Offset bounds (e.g. list/string offsets pointing past the end of their value buffer)
- UTF-8 validity of string columns (`Utf8` / `LargeUtf8`)
- Null count consistency and buffer length checks
# Safety

Relies on the caller only passing a flag with `true` value if they are
certain that the data is valid. Invalid data that bypasses these checks
may cause undefined behavior when the arrays are later accessed.
