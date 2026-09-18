# `arrow_ipc::writer::IpcDataGenerator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.IpcDataGenerator.json).

<a id="op-26ea3c515d0494f20ce6fa88"></a>
## IpcDataGenerator

`struct` · `arrow_ipc::writer::IpcDataGenerator` · arrow-ipc 59.3.0

```rust
struct IpcDataGenerator
```

Source: `src/writer.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Handles low level details of encoding [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) and [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) into the
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

<a id="op-f25c128452542ffef81bd928"></a>
## default

`function` · `arrow_ipc::writer::IpcDataGenerator::default` · arrow-ipc 59.3.0

```rust
fn default() -> IpcDataGenerator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcDataGenerator", "path": "IpcDataGenerator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 17], "end": [537, 24], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94bac1422afc35476a74a1b4"></a>
## encode

`function` · `arrow_ipc::writer::IpcDataGenerator::encode` · arrow-ipc 59.3.0

```rust
fn encode(&self, batch: &RecordBatch, dictionary_tracker: &mut DictionaryTracker, write_options: &IpcWriteOptions, ipc_write_context: &mut IpcWriteContext) -> Result<(Vec<EncodedData>, EncodedData), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcDataGenerator", "path": "IpcDataGenerator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [1194, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:853`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Encodes a batch to a number of [EncodedData](../operations/arrow_ipc.writer.EncodedData.md#op-7935547a30bcba70bbdf01dd) items (dictionary batches + the record batch).
The [DictionaryTracker](../operations/arrow_ipc.writer.DictionaryTracker.md#op-29e5b236e3aaa8b8b95a1657) keeps track of dictionaries with new `dict_id`s  (so they are only sent once)
Make sure the [DictionaryTracker](../operations/arrow_ipc.writer.DictionaryTracker.md#op-29e5b236e3aaa8b8b95a1657) is initialized at the start of the stream.

<a id="op-cce885a8d437db4d736458fd"></a>
## encoded_batch

`function` · `arrow_ipc::writer::IpcDataGenerator::encoded_batch` · arrow-ipc 59.3.0

```rust
fn encoded_batch(&self, batch: &RecordBatch, dictionary_tracker: &mut DictionaryTracker, write_options: &IpcWriteOptions) -> Result<(Vec<EncodedData>, EncodedData), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcDataGenerator", "path": "IpcDataGenerator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [1194, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:988`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Encodes a batch to a number of [EncodedData](../operations/arrow_ipc.writer.EncodedData.md#op-7935547a30bcba70bbdf01dd) items (dictionary batches + the record batch).
The [DictionaryTracker](../operations/arrow_ipc.writer.DictionaryTracker.md#op-29e5b236e3aaa8b8b95a1657) keeps track of dictionaries with new `dict_id`s  (so they are only sent once)
Make sure the [DictionaryTracker](../operations/arrow_ipc.writer.DictionaryTracker.md#op-29e5b236e3aaa8b8b95a1657) is initialized at the start of the stream.

<a id="op-37230f33bad8f8e90ec97e61"></a>
## fmt

`function` · `arrow_ipc::writer::IpcDataGenerator::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcDataGenerator", "path": "IpcDataGenerator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 10], "end": [537, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83e0c888ed5d8a6925c4953a"></a>
## schema_to_bytes_with_dictionary_tracker

`function` · `arrow_ipc::writer::IpcDataGenerator::schema_to_bytes_with_dictionary_tracker` · arrow-ipc 59.3.0

```rust
fn schema_to_bytes_with_dictionary_tracker(&self, schema: &Schema, dictionary_tracker: &mut DictionaryTracker, write_options: &IpcWriteOptions) -> EncodedData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcDataGenerator", "path": "IpcDataGenerator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [1194, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:576`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Converts a schema to an IPC message along with `dictionary_tracker`
and returns it encoded inside [EncodedData](../operations/arrow_ipc.writer.EncodedData.md#op-7935547a30bcba70bbdf01dd) as a flatbuffer.
