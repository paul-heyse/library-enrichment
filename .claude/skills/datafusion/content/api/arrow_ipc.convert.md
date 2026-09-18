# `arrow_ipc::convert`

Crate `arrow-ipc` · 7 public items · structured records in [`model/arrow_ipc.convert.json`](../model/arrow_ipc.convert.json)

## fb_to_schema

`function` · `arrow_ipc::convert::fb_to_schema`

```rust
fn fb_to_schema(fb: Schema<'_>) -> Schema
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.convert.fb_to_schema.md).


Deserialize an ipc [crate::Schema`] from flat buffers to an arrow [Schema].

---

## metadata_to_fb

`function` · `arrow_ipc::convert::metadata_to_fb`

```rust
fn metadata_to_fb<'a>(fbb: &mut flatbuffers::FlatBufferBuilder<'a>, metadata: &std::collections::HashMap<String, String>) -> flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.convert.metadata_to_fb.md).


Push a key-value metadata into a FlatBufferBuilder and return [WIPOffset]

---

## schema_to_fb_offset

`function` · `arrow_ipc::convert::schema_to_fb_offset`

```rust
fn schema_to_fb_offset<'a>(fbb: &mut flatbuffers::FlatBufferBuilder<'a>, schema: &Schema) -> flatbuffers::WIPOffset<Schema<'a>>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.convert.schema_to_fb_offset.md).


Adds a [Schema] to a flatbuffer and returns the offset

---

## try_schema_from_flatbuffer_bytes

`function` · `arrow_ipc::convert::try_schema_from_flatbuffer_bytes`

```rust
fn try_schema_from_flatbuffer_bytes(bytes: &[u8]) -> Result<Schema, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.convert.try_schema_from_flatbuffer_bytes.md).


Try deserialize flat buffer format bytes into a schema

---

## try_schema_from_ipc_buffer

`function` · `arrow_ipc::convert::try_schema_from_ipc_buffer`

```rust
fn try_schema_from_ipc_buffer(buffer: &[u8]) -> Result<Schema, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.convert.try_schema_from_ipc_buffer.md).


Try deserialize the IPC format bytes into a schema

---

## IpcSchemaEncoder

`struct` · `arrow_ipc::convert::IpcSchemaEncoder`

```rust
struct IpcSchemaEncoder<'a>
```

**Derives**: Debug, Default

**Methods** (4)

```rust
fn new() -> IpcSchemaEncoder<'a>
fn schema_to_fb<'b>(&mut self, schema: &Schema) -> FlatBufferBuilder<'b>
fn schema_to_fb_offset<'b>(&mut self, fbb: &mut FlatBufferBuilder<'b>, schema: &Schema) -> WIPOffset<Schema<'b>>
fn with_dictionary_tracker(self, dictionary_tracker: &'a mut DictionaryTracker) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.convert.IpcSchemaEncoder.md).


Low level Arrow [Schema] to IPC bytes converter

See also [`fb_to_schema`] for the reverse operation

# Example
```
# use arrow_ipc::convert::{fb_to_schema, IpcSchemaEncoder};
# use arrow_ipc::root_as_schema;
# use arrow_ipc::writer::DictionaryTracker;
# use arrow_schema::{DataType, Field, Schema};
// given an arrow schema to serialize
let schema = Schema::new(vec![
   Field::new("a", DataType::Int32, false),
]);

// Use a dictionary tracker to track dictionary id if needed
 let mut dictionary_tracker = DictionaryTracker::new(true);
// create a FlatBuffersBuilder that contains the encoded bytes
 let fb = IpcSchemaEncoder::new()
   .with_dictionary_tracker(&mut dictionary_tracker)
   .schema_to_fb(&schema);

// the bytes are in `fb.finished_data()`
let ipc_bytes = fb.finished_data();

 // convert the IPC bytes back to an Arrow schema
 let ipc_schema = root_as_schema(ipc_bytes).unwrap();
 let schema2 = fb_to_schema(ipc_schema);
assert_eq!(schema, schema2);
```

---

## MessageBuffer

`struct` · `arrow_ipc::convert::MessageBuffer`

```rust
struct MessageBuffer
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn as_ref(&self) -> Message<'_>
fn try_new(buf: Buffer) -> Result<Self, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.convert.MessageBuffer.md).


An owned container for a validated [`Message`]

Safely decoding a flatbuffer requires validating the various embedded offsets,
see [`Verifier`]. This is a potentially expensive operation, and it is therefore desirable
to only do this once. [`crate::root_as_message`] performs this validation on construction,
however, it returns a [`Message`] borrowing the provided byte slice. This prevents
storing this [`Message`] in the same data structure that owns the buffer, as this
would require self-referential borrows.

[`MessageBuffer`] solves this problem by providing a safe API for a [`Message`]
without a lifetime bound.

---
