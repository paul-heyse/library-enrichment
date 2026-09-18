# `arrow_ipc::reader::FileDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.FileDecoder.json).

<a id="op-6e691454a30392bcbceb3c88"></a>
## FileDecoder

`struct` · `arrow_ipc::reader::FileDecoder` · arrow-ipc 59.3.0

```rust
struct FileDecoder
```

Source: `src/reader.rs:1021`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

A low-level, push-based interface for reading an IPC file

For a higher-level interface see [`FileReader`](../operations/arrow_ipc.reader.FileReader.md#op-637d6dc146e8bf36b6e304bf)

For an example of using this API with `mmap` see the [`zero_copy_ipc`] example.

[`zero_copy_ipc`]: https://github.com/apache/arrow-rs/blob/main/arrow/examples/zero_copy_ipc.rs

```
# use std::sync::Arc;
# use arrow_array::*;
# use arrow_array::types::Int32Type;
# use arrow_buffer::Buffer;
# use arrow_ipc::convert::fb_to_schema;
# use arrow_ipc::reader::{FileDecoder, read_footer_length};
# use arrow_ipc::root_as_footer;
# use arrow_ipc::writer::FileWriter;
// Write an IPC file

let batch = RecordBatch::try_from_iter([
    ("a", Arc::new(Int32Array::from(vec![1, 2, 3])) as _),
    ("b", Arc::new(Int32Array::from(vec![1, 2, 3])) as _),
    ("c", Arc::new(DictionaryArray::<Int32Type>::from_iter(["hello", "hello", "world"])) as _),
]).unwrap();

let schema = batch.schema();

let mut out = Vec::with_capacity(1024);
let mut writer = FileWriter::try_new(&mut out, schema.as_ref()).unwrap();
writer.write(&batch).unwrap();
writer.finish().unwrap();

drop(writer);

// Read IPC file

let buffer = Buffer::from_vec(out);
let trailer_start = buffer.len() - 10;
let footer_len = read_footer_length(buffer[trailer_start..].try_into().unwrap()).unwrap();
let footer = root_as_footer(&buffer[trailer_start - footer_len..trailer_start]).unwrap();

let back = fb_to_schema(footer.schema().unwrap());
assert_eq!(&back, schema.as_ref());

let mut decoder = FileDecoder::new(schema, footer.version());

// Read dictionaries
for block in footer.dictionaries().iter().flatten() {
    let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
    let data = buffer.slice_with_length(block.offset() as _, block_len);
    decoder.read_dictionary(&block, &data).unwrap();
}

// Read record batch
let batches = footer.recordBatches().unwrap();
assert_eq!(batches.len(), 1); // Only wrote a single batch

let block = batches.get(0);
let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
let data = buffer.slice_with_length(block.offset() as _, block_len);
let back = decoder.read_record_batch(block, &data).unwrap().unwrap();

assert_eq!(batch, back);
```

<a id="op-54d2cb0be1f4a6526645bf61"></a>
## fmt

`function` · `arrow_ipc::reader::FileDecoder::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileDecoder", "path": "FileDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1020, 10], "end": [1020, 15], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader.rs:1020`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a20f9042276749a431d6212"></a>
## new

`function` · `arrow_ipc::reader::FileDecoder::new` · arrow-ipc 59.3.0

```rust
fn new(schema: SchemaRef, version: MetadataVersion) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileDecoder", "path": "FileDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1030, 1], "end": [1150, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1032`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Create a new [`FileDecoder`](../operations/arrow_ipc.reader.FileDecoder.md#op-6e691454a30392bcbceb3c88) with the given schema and version

<a id="op-cd113b10317005a1aa815923"></a>
## read_dictionary

`function` · `arrow_ipc::reader::FileDecoder::read_dictionary` · arrow-ipc 59.3.0

```rust
fn read_dictionary(&mut self, block: &Block, buf: &Buffer) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileDecoder", "path": "FileDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1030, 1], "end": [1150, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1094`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Read the dictionary with the given block and data buffer

<a id="op-94811364eb792d09c9651ea8"></a>
## read_record_batch

`function` · `arrow_ipc::reader::FileDecoder::read_record_batch` · arrow-ipc 59.3.0

```rust
fn read_record_batch(&self, block: &Block, buf: &Buffer) -> Result<Option<RecordBatch>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileDecoder", "path": "FileDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1030, 1], "end": [1150, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1116`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Read the RecordBatch with the given block and data buffer

<a id="op-4867a14351f3f7c88f4622b2"></a>
## with_projection

`function` · `arrow_ipc::reader::FileDecoder::with_projection` · arrow-ipc 59.3.0

```rust
fn with_projection(self, projection: Vec<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileDecoder", "path": "FileDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1030, 1], "end": [1150, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1044`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specify a projection

<a id="op-db3176b246a3ec2268493421"></a>
## with_require_alignment

`function` · `arrow_ipc::reader::FileDecoder::with_require_alignment` · arrow-ipc 59.3.0

```rust
fn with_require_alignment(self, require_alignment: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileDecoder", "path": "FileDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1030, 1], "end": [1150, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1061`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specifies if the array data in input buffers is required to be properly aligned.

If `require_alignment` is true, this decoder will return an error if any array data in the
input `buf` is not properly aligned.
Under the hood it will use [`arrow_data::ArrayDataBuilder::build`] to construct
[`arrow_data::ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

If `require_alignment` is false (the default), this decoder will automatically allocate a
new aligned buffer and copy over the data if any array data in the input `buf` is not
properly aligned. (Properly aligned array data will remain zero-copy.)
Under the hood it will use [`arrow_data::ArrayDataBuilder::align_buffers`] to construct
[`arrow_data::ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

Unresolved upstream links (retained, not inferred): ``arrow_data::ArrayDataBuilder::build``, ``arrow_data::ArrayDataBuilder::align_buffers``.

<a id="op-e750154fa82597609b22c72a"></a>
## with_skip_validation

`function` · `arrow_ipc::reader::FileDecoder::with_skip_validation` · arrow-ipc 59.3.0

```rust
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileDecoder", "path": "FileDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1030, 1], "end": [1150, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1076`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specifies if validation should be skipped when reading data (defaults to `false`)

# Safety

This flag must only be set to `true` when you trust the input data and are sure the data you are
reading is a valid Arrow IPC file, otherwise undefined behavior may
result.

For example, some programs may wish to trust reading IPC files written
by the same process that created the files.
