# `arrow_ipc::writer::StreamEncoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.StreamEncoder.json).

<a id="op-2fa1cc7101dbf770ab37308a"></a>
## StreamEncoder

`struct` · `arrow_ipc::writer::StreamEncoder` · arrow-ipc 59.3.0

```rust
struct StreamEncoder
```

Source: `src/writer.rs:1853`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow IPC stream encoder.

Encodes Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to byte buffers using the [IPC Streaming Format],
without performing any IO.

The returned [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s are ordered and should be written to the destination
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

<a id="op-9ad7986f29a54432f480e485"></a>
## encode

`function` · `arrow_ipc::writer::StreamEncoder::encode` · arrow-ipc 59.3.0

```rust
fn encode(&mut self, batch: &RecordBatch) -> Result<Vec<Buffer>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::StreamEncoder", "path": "StreamEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 1], "end": [1940, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1898`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Encode a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) into buffers.

The first call also includes the IPC stream schema message before the
record batch message. Later calls only include dictionary and record
batch messages.

# Errors

Returns an error if encoding fails.

<a id="op-a313bddb413e1609cc55a84d"></a>
## finish

`function` · `arrow_ipc::writer::StreamEncoder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> Result<Vec<Buffer>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::StreamEncoder", "path": "StreamEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 1], "end": [1940, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1919`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Encode the end-of-stream marker.

If no batches have been encoded, this also emits the IPC stream schema
message so the returned buffers form a valid empty IPC stream.

# Errors

Returns an error if encoding the schema or end-of-stream marker fails.

<a id="op-e8ed75996494d18d491ceb7e"></a>
## try_new

`function` · `arrow_ipc::writer::StreamEncoder::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(schema: &Schema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::StreamEncoder", "path": "StreamEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 1], "end": [1940, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1867`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new stream encoder.

<a id="op-12ffca3723e738c733e005a4"></a>
## try_new_with_options

`function` · `arrow_ipc::writer::StreamEncoder::try_new_with_options` · arrow-ipc 59.3.0

```rust
fn try_new_with_options(schema: &Schema, write_options: IpcWriteOptions) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::StreamEncoder", "path": "StreamEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 1], "end": [1940, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1873`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new stream encoder with [`IpcWriteOptions`](../operations/arrow_ipc.writer.IpcWriteOptions.md#op-7958df777a3d4faa0bb68d5a).
