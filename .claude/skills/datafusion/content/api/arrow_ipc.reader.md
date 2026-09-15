# `arrow_ipc::reader`

Crate `arrow-ipc` · 9 public items · structured records in [`model/arrow_ipc.reader.json`](../model/arrow_ipc.reader.json)

## read_dictionary

`function` · `arrow_ipc::reader::read_dictionary`

```rust
fn read_dictionary(buf: &arrow_buffer::Buffer, batch: DictionaryBatch<'_>, schema: &Schema, dictionaries_by_id: &mut std::collections::HashMap<i64, ArrayRef>, metadata: &MetadataVersion) -> Result<(), ArrowError>
```

Read the dictionary from the buffer and provided metadata,
updating the `dictionaries_by_id` with the resulting dictionary

---

## read_dictionary_impl

`function` · `arrow_ipc::reader::read_dictionary_impl`

```rust
fn read_dictionary_impl(buf: &arrow_buffer::Buffer, batch: DictionaryBatch<'_>, schema: &Schema, dictionaries_by_id: &mut std::collections::HashMap<i64, ArrayRef>, metadata: &MetadataVersion, require_alignment: bool, skip_validation: arrow_data::UnsafeFlag) -> Result<(), ArrowError>
```

Low-level version of [`read_dictionary`] with alignment and validation controls

---

## read_footer_length

`function` · `arrow_ipc::reader::read_footer_length`

```rust
fn read_footer_length(buf: [u8; 10]) -> Result<usize, ArrowError>
```

Read the footer length from the last 10 bytes of an Arrow IPC file

Expects a 4 byte footer length followed by `b"ARROW1"`

---

## read_record_batch

`function` · `arrow_ipc::reader::read_record_batch`

```rust
fn read_record_batch(buf: &arrow_buffer::Buffer, batch: RecordBatch<'_>, schema: SchemaRef, dictionaries_by_id: &std::collections::HashMap<i64, ArrayRef>, projection: Option<&[usize]>, metadata: &MetadataVersion) -> Result<RecordBatch, ArrowError>
```

Creates a record batch from binary data using the `crate::RecordBatch` indexes and the `Schema`.

If `require_alignment` is true, this function will return an error if any array data in the
input `buf` is not properly aligned.
Under the hood it will use [`arrow_data::ArrayDataBuilder::build`] to construct [`arrow_data::ArrayData`].

If `require_alignment` is false, this function will automatically allocate a new aligned buffer
and copy over the data if any array data in the input `buf` is not properly aligned.
(Properly aligned array data will remain zero-copy.)
Under the hood it will use [`arrow_data::ArrayDataBuilder::align_buffers`] to construct [`arrow_data::ArrayData`].

---

## FileDecoder

`struct` · `arrow_ipc::reader::FileDecoder`

```rust
struct FileDecoder
```

**Derives**: Debug

**Methods** (6)

```rust
fn new(schema: SchemaRef, version: MetadataVersion) -> Self
fn read_dictionary(&mut self, block: &Block, buf: &Buffer) -> Result<(), ArrowError>
fn read_record_batch(&self, block: &Block, buf: &Buffer) -> Result<Option<RecordBatch>, ArrowError>
fn with_projection(self, projection: Vec<usize>) -> Self
fn with_require_alignment(self, require_alignment: bool) -> Self
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

A low-level, push-based interface for reading an IPC file

For a higher-level interface see [`FileReader`]

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

---

## FileReader

`struct` · `arrow_ipc::reader::FileReader`

```rust
struct FileReader<R>
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (9)

```rust
fn custom_metadata(&self) -> &HashMap<String, String>
fn get_mut(&mut self) -> &mut R
fn get_ref(&self) -> &R
fn num_batches(&self) -> usize
fn schema(&self) -> SchemaRef
fn set_index(&mut self, index: usize) -> Result<(), ArrowError>
fn try_new(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
fn try_new_buffered(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Arrow File Reader

Reads Arrow [`RecordBatch`]es from bytes in the [IPC File Format],
providing random access to the record batches.

# See Also

* [`Self::set_index`] for random access
* [`StreamReader`] for reading streaming data

# Example: Reading from a `File`
```
# use std::io::Cursor;
use arrow_array::record_batch;
# use arrow_ipc::reader::FileReader;
# use arrow_ipc::writer::FileWriter;
# let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
# let mut file = vec![]; // mimic a stream for the example
# {
#  let mut writer = FileWriter::try_new(&mut file, &batch.schema()).unwrap();
#  writer.write(&batch).unwrap();
#  writer.write(&batch).unwrap();
#  writer.finish().unwrap();
# }
# let mut file = Cursor::new(&file);
let projection = None; // read all columns
let mut reader = FileReader::try_new(&mut file, projection).unwrap();
// Position the reader to the second batch
reader.set_index(1).unwrap();
// read batches from the reader using the Iterator trait
let mut num_rows = 0;
for batch in reader {
   let batch = batch.unwrap();
   num_rows += batch.num_rows();
}
assert_eq!(num_rows, 3);
```
# Example: Reading from `mmap`ed file

For an example creating Arrays without copying using  memory mapped (`mmap`)
files see the [`zero_copy_ipc`] example.

[IPC File Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format
[`zero_copy_ipc`]: https://github.com/apache/arrow-rs/blob/main/arrow/examples/zero_copy_ipc.rs

---

## FileReaderBuilder

`struct` · `arrow_ipc::reader::FileReaderBuilder`

```rust
struct FileReaderBuilder
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn build<R: Read + Seek>(self, reader: R) -> Result<FileReader<R>, ArrowError>
fn new() -> Self
fn with_max_footer_fb_depth(self, max_footer_fb_depth: usize) -> Self
fn with_max_footer_fb_tables(self, max_footer_fb_tables: usize) -> Self
fn with_projection(self, projection: Vec<usize>) -> Self
```

Build an Arrow [`FileReader`] with custom options.

---

## RecordBatchDecoder

`struct` · `arrow_ipc::reader::RecordBatchDecoder`

```rust
struct RecordBatchDecoder<'a>
```

**Methods** (5)

```rust
fn read_record_batch(self) -> Result<RecordBatch, ArrowError>
fn try_new(buf: &'a Buffer, batch: RecordBatch<'a>, schema: SchemaRef, dictionaries_by_id: &'a HashMap<i64, ArrayRef>, metadata: &'a MetadataVersion) -> Result<Self, ArrowError>
fn with_projection(self, projection: Option<&'a [usize]>) -> Self
fn with_require_alignment(self, require_alignment: bool) -> Self
fn with_skip_validation(self, skip_validation: UnsafeFlag) -> Self
```

State for decoding Arrow arrays from an [IPC RecordBatch] structure to
[`RecordBatch`]

[IPC RecordBatch]: crate::RecordBatch

---

## StreamReader

`struct` · `arrow_ipc::reader::StreamReader`

```rust
struct StreamReader<R>
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (8)

```rust
fn get_mut(&mut self) -> &mut R
fn get_ref(&self) -> &R
fn is_finished(&self) -> bool
fn schema(&self) -> SchemaRef
fn try_new(reader: R, projection: Option<Vec<usize>>) -> Result<StreamReader<R>, ArrowError>
fn try_new_buffered(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
fn try_new_unbuffered(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Arrow Stream Reader

Reads Arrow [`RecordBatch`]es from bytes in the [IPC Streaming Format].

# See Also

* [`FileReader`] for random access.

# Example
```
# use arrow_array::record_batch;
# use arrow_ipc::reader::StreamReader;
# use arrow_ipc::writer::StreamWriter;
# let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
# let mut stream = vec![]; // mimic a stream for the example
# {
#  let mut writer = StreamWriter::try_new(&mut stream, &batch.schema()).unwrap();
#  writer.write(&batch).unwrap();
#  writer.finish().unwrap();
# }
# let stream = stream.as_slice();
let projection = None; // read all columns
let mut reader = StreamReader::try_new(stream, projection).unwrap();
// read batches from the reader using the Iterator trait
let mut num_rows = 0;
for batch in reader {
   let batch = batch.unwrap();
   num_rows += batch.num_rows();
}
assert_eq!(num_rows, 3);
```

[IPC Streaming Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format

---
