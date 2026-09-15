# `arrow_ipc::writer`

Crate `arrow-ipc` · 10 public items · structured records in [`model/arrow_ipc.writer.json`](../model/arrow_ipc.writer.json)

## DictionaryHandling

`enum` · `arrow_ipc::writer::DictionaryHandling`

```rust
enum DictionaryHandling
```

**Variants**: `Resend`, `Delta`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Controls how dictionaries are handled in Arrow IPC messages

---

## DictionaryUpdate

`enum` · `arrow_ipc::writer::DictionaryUpdate`

```rust
enum DictionaryUpdate
```

**Variants**: `None`, `New`, `Replaced`, `Delta`

**Derives**: Clone, Debug

Describes what kind of update took place after a call to [`DictionaryTracker::insert`].

---

## write_message

`function` · `arrow_ipc::writer::write_message`

```rust
fn write_message<W: Write>(writer: W, encoded: EncodedData, write_options: &IpcWriteOptions) -> Result<(usize, usize), ArrowError>
```

Write a message's IPC data and buffers, returning metadata and buffer data lengths written

---

## DictionaryTracker

`struct` · `arrow_ipc::writer::DictionaryTracker`

```rust
struct DictionaryTracker
```

**Derives**: Debug

**Methods** (6)

```rust
fn clear(&mut self)
fn dict_id(&mut self) -> &[i64]
fn insert(&mut self, dict_id: i64, column: &ArrayRef) -> Result<bool, ArrowError>
fn insert_column(&mut self, dict_id: i64, column: &ArrayRef, dict_handling: DictionaryHandling) -> Result<DictionaryUpdate, ArrowError>
fn new(error_on_replacement: bool) -> Self
fn next_dict_id(&mut self) -> i64
```

Keeps track of dictionaries that have been written, to avoid emitting the same dictionary
multiple times.

Can optionally error if an update to an existing dictionary is attempted, which
isn't allowed in the `FileWriter`.

---

## EncodedData

`struct` · `arrow_ipc::writer::EncodedData`

```rust
struct EncodedData
```

**Fields**: `ipc_message`, `arrow_data`

Stores the encoded data, which is an crate::Message, and optional Arrow data

---

## FileWriter

`struct` · `arrow_ipc::writer::FileWriter`

```rust
struct FileWriter<W>
```

**Implements**: `arrow_array::record_batch::RecordBatchWriter`

**Methods** (11)

```rust
fn finish(&mut self) -> Result<(), ArrowError>
fn flush(&mut self) -> Result<(), ArrowError>
fn get_mut(&mut self) -> &mut W
fn get_ref(&self) -> &W
fn into_inner(self) -> Result<W, ArrowError>
fn schema(&self) -> &SchemaRef
fn try_new(writer: W, schema: &Schema) -> Result<Self, ArrowError>
fn try_new_buffered(writer: W, schema: &Schema) -> Result<Self, ArrowError>
fn try_new_with_options(writer: W, schema: &Schema, write_options: IpcWriteOptions) -> Result<Self, ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
fn write_metadata(&mut self, key: impl Into<String>, value: impl Into<String>)
```

**via `arrow_array::record_batch::RecordBatchWriter`**

```rust
fn close(self) -> Result<(), ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Arrow File Writer

Writes Arrow [`RecordBatch`]es in the [IPC File Format].

# See Also

* [`StreamWriter`] for writing IPC Streams

# Example
```
# use arrow_array::record_batch;
# use arrow_ipc::writer::FileWriter;
# let mut file = vec![]; // mimic a file for the example
let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// create a new writer, the schema must be known in advance
let mut writer = FileWriter::try_new(&mut file, &batch.schema()).unwrap();
// write each batch to the underlying writer
writer.write(&batch).unwrap();
// When all batches are written, call finish to flush all buffers
writer.finish().unwrap();
```
[IPC File Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format

---

## IpcDataGenerator

`struct` · `arrow_ipc::writer::IpcDataGenerator`

```rust
struct IpcDataGenerator
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn encode(&self, batch: &RecordBatch, dictionary_tracker: &mut DictionaryTracker, write_options: &IpcWriteOptions, ipc_write_context: &mut IpcWriteContext) -> Result<(Vec<EncodedData>, EncodedData), ArrowError>
fn encoded_batch(&self, batch: &RecordBatch, dictionary_tracker: &mut DictionaryTracker, write_options: &IpcWriteOptions) -> Result<(Vec<EncodedData>, EncodedData), ArrowError>
fn schema_to_bytes_with_dictionary_tracker(&self, schema: &Schema, dictionary_tracker: &mut DictionaryTracker, write_options: &IpcWriteOptions) -> EncodedData
```

Handles low level details of encoding [`Array`] and [`Schema`] into the
[Arrow IPC Format].

# Example
```
# fn run() {
# use std::sync::Arc;
# use arrow_array::UInt64Array;
# use arrow_array::RecordBatch;
# use arrow_ipc::writer::{IpcWriteContext, DictionaryTracker, IpcDataGenerator, IpcWriteOptions};

// Create a record batch
let batch = RecordBatch::try_from_iter(vec![
 ("col2", Arc::new(UInt64Array::from_iter([10, 23, 33])) as _)
]).unwrap();

// Error of dictionary ids are replaced.
let error_on_replacement = true;
let options = IpcWriteOptions::default();
let mut dictionary_tracker = DictionaryTracker::new(error_on_replacement);

let mut ipc_write_context = IpcWriteContext::default();

// encode the batch into zero or more encoded dictionaries
// and the data for the actual array.
let data_gen = IpcDataGenerator::default();
let (encoded_dictionaries, encoded_message) = data_gen
  .encode(&batch, &mut dictionary_tracker, &options, &mut ipc_write_context)
  .unwrap();
# }
```

[Arrow IPC Format]: https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc

---

## IpcWriteOptions

`struct` · `arrow_ipc::writer::IpcWriteOptions`

```rust
struct IpcWriteOptions
```

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn try_new(alignment: usize, write_legacy_ipc_format: bool, metadata_version: MetadataVersion) -> Result<Self, ArrowError>
fn try_with_compression(self, batch_compression_type: Option<CompressionType>) -> Result<Self, ArrowError>
fn try_with_compression_level(self, batch_compression_level: Option<i32>) -> Result<Self, ArrowError>
fn with_dictionary_handling(self, dictionary_handling: DictionaryHandling) -> Self
```

IPC write options used to control the behaviour of the [`IpcDataGenerator`]

---

## StreamEncoder

`struct` · `arrow_ipc::writer::StreamEncoder`

```rust
struct StreamEncoder
```

**Methods** (4)

```rust
fn encode(&mut self, batch: &RecordBatch) -> Result<Vec<Buffer>, ArrowError>
fn finish(self) -> Result<Vec<Buffer>, ArrowError>
fn try_new(schema: &Schema) -> Result<Self, ArrowError>
fn try_new_with_options(schema: &Schema, write_options: IpcWriteOptions) -> Result<Self, ArrowError>
```

Arrow IPC stream encoder.

Encodes Arrow [`RecordBatch`]es to byte buffers using the [IPC Streaming Format],
without performing any IO.

The returned [`Buffer`]s are ordered and should be written to the destination
stream in order. Uncompressed record batch body buffers can share the original
Arrow buffers instead of being copied into an intermediate contiguous buffer.

# Example
```
# use arrow_array::record_batch;
# use arrow_ipc::writer::StreamEncoder;
# use arrow_schema::ArrowError;
# fn main() -> Result<(), ArrowError> {
let batch = record_batch!(("a", Int32, [1, 2, 3]))?;

let mut encoder = StreamEncoder::try_new(&batch.schema())?;
let mut stream = vec![];
for buffer in encoder.encode(&batch)? {
    stream.extend_from_slice(buffer.as_slice());
}
for buffer in encoder.finish()? {
    stream.extend_from_slice(buffer.as_slice());
}
# Ok(())
# }
```

---

## StreamWriter

`struct` · `arrow_ipc::writer::StreamWriter`

```rust
struct StreamWriter<W>
```

**Implements**: `arrow_array::record_batch::RecordBatchWriter`

**Methods** (9)

```rust
fn finish(&mut self) -> Result<(), ArrowError>
fn flush(&mut self) -> Result<(), ArrowError>
fn get_mut(&mut self) -> &mut W
fn get_ref(&self) -> &W
fn into_inner(self) -> Result<W, ArrowError>
fn try_new(writer: W, schema: &Schema) -> Result<Self, ArrowError>
fn try_new_buffered(writer: W, schema: &Schema) -> Result<Self, ArrowError>
fn try_new_with_options(writer: W, schema: &Schema, write_options: IpcWriteOptions) -> Result<Self, ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

**via `arrow_array::record_batch::RecordBatchWriter`**

```rust
fn close(self) -> Result<(), ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Arrow Stream Writer

Writes Arrow [`RecordBatch`]es to bytes using the [IPC Streaming Format].

# See Also

* [`FileWriter`] for writing IPC Files

# Example - Basic usage
```
# use arrow_array::record_batch;
# use arrow_ipc::writer::StreamWriter;
# let mut stream = vec![]; // mimic a stream for the example
let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// create a new writer, the schema must be known in advance
let mut writer = StreamWriter::try_new(&mut stream, &batch.schema()).unwrap();
// write each batch to the underlying stream
writer.write(&batch).unwrap();
// When all batches are written, call finish to flush all buffers
writer.finish().unwrap();
```
# Example - Efficient delta dictionaries
```
# use arrow_array::record_batch;
# use arrow_ipc::writer::{StreamWriter, IpcWriteOptions};
# use arrow_ipc::writer::DictionaryHandling;
# use arrow_schema::{DataType, Field, Schema, SchemaRef};
# use arrow_array::{
#    builder::StringDictionaryBuilder, types::Int32Type, Array, ArrayRef, DictionaryArray,
#    RecordBatch, StringArray,
# };
# use std::sync::Arc;

let schema = Arc::new(Schema::new(vec![Field::new(
   "col1",
   DataType::Dictionary(Box::from(DataType::Int32), Box::from(DataType::Utf8)),
   true,
)]));

let mut builder = StringDictionaryBuilder::<arrow_array::types::Int32Type>::new();

// `finish_preserve_values` will keep the dictionary values along with their
// key assignments so that they can be re-used in the next batch.
builder.append("a").unwrap();
builder.append("b").unwrap();
let array1 = builder.finish_preserve_values();
let batch1 = RecordBatch::try_new(schema.clone(), vec![Arc::new(array1) as ArrayRef]).unwrap();

// In this batch, 'a' will have the same dictionary key as 'a' in the previous batch,
// and 'd' will take the next available key.
builder.append("a").unwrap();
builder.append("d").unwrap();
let array2 = builder.finish_preserve_values();
let batch2 = RecordBatch::try_new(schema.clone(), vec![Arc::new(array2) as ArrayRef]).unwrap();

let mut stream = vec![];
// You must set `.with_dictionary_handling(DictionaryHandling::Delta)` to
// enable delta dictionaries in the writer
let options = IpcWriteOptions::default().with_dictionary_handling(DictionaryHandling::Delta);
let mut writer = StreamWriter::try_new_with_options(&mut stream, &schema, options).unwrap();

// When writing the first batch, a dictionary message with 'a' and 'b' will be written
// prior to the record batch.
writer.write(&batch1).unwrap();
// With the second batch only a delta dictionary with 'd' will be written
// prior to the record batch. This is only possible with `finish_preserve_values`.
// Without it, 'a' and 'd' in this batch would have different keys than the
// first batch and so we'd have to send a replacement dictionary with new keys
// for both.
writer.write(&batch2).unwrap();
writer.finish().unwrap();
```
[IPC Streaming Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format

---
