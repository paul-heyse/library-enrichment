# `arrow_ipc::convert::IpcSchemaEncoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.convert.IpcSchemaEncoder.json).

<a id="op-a6118676eee6db817c8167cf"></a>
## IpcSchemaEncoder

`struct` · `arrow_ipc::convert::IpcSchemaEncoder` · arrow-ipc 59.3.0

```rust
struct IpcSchemaEncoder<'a>
```

Source: `src/convert.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Low level Arrow [Schema](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) to IPC bytes converter

See also [`fb_to_schema`](../operations/arrow_ipc.convert.fb_to_schema.md#op-6ffcfc831af9be3afc89cc82) for the reverse operation

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

<a id="op-78f5ed0b7480f0d5ba72bb9c"></a>
## default

`function` · `arrow_ipc::convert::IpcSchemaEncoder::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::convert::IpcSchemaEncoder", "path": "IpcSchemaEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [74, 2], "filename": "src/convert.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/convert.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cde4faa56569d6c244c4351"></a>
## fmt

`function` · `arrow_ipc::convert::IpcSchemaEncoder::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::convert::IpcSchemaEncoder", "path": "IpcSchemaEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/convert.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/convert.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec5f42604368ca7b68700155"></a>
## new

`function` · `arrow_ipc::convert::IpcSchemaEncoder::new` · arrow-ipc 59.3.0

```rust
fn new() -> IpcSchemaEncoder<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::convert::IpcSchemaEncoder", "path": "IpcSchemaEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [129, 2], "filename": "src/convert.rs"}, "trait": null, "trait_path": null}`

Source: `src/convert.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Create a new schema encoder

<a id="op-91e7eecc97924a33d44a13fa"></a>
## schema_to_fb

`function` · `arrow_ipc::convert::IpcSchemaEncoder::schema_to_fb` · arrow-ipc 59.3.0

```rust
fn schema_to_fb<'b>(&mut self, schema: &Schema) -> FlatBufferBuilder<'b>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::convert::IpcSchemaEncoder", "path": "IpcSchemaEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [129, 2], "filename": "src/convert.rs"}, "trait": null, "trait_path": null}`

Source: `src/convert.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Serialize a schema in IPC format, returning a completed [`FlatBufferBuilder`]

Note: Call [`FlatBufferBuilder::finished_data`] to get the serialized bytes

Unresolved upstream links (retained, not inferred): ``FlatBufferBuilder``, ``FlatBufferBuilder::finished_data``.

<a id="op-a2f0138a343437afb11968ef"></a>
## schema_to_fb_offset

`function` · `arrow_ipc::convert::IpcSchemaEncoder::schema_to_fb_offset` · arrow-ipc 59.3.0

```rust
fn schema_to_fb_offset<'b>(&mut self, fbb: &mut FlatBufferBuilder<'b>, schema: &Schema) -> WIPOffset<Schema<'b>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::convert::IpcSchemaEncoder", "path": "IpcSchemaEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [129, 2], "filename": "src/convert.rs"}, "trait": null, "trait_path": null}`

Source: `src/convert.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Serialize a schema to an in progress [`FlatBufferBuilder`], returning the in progress offset.

Unresolved upstream links (retained, not inferred): ``FlatBufferBuilder``.

<a id="op-a229f53807b2e33b75f17600"></a>
## with_dictionary_tracker

`function` · `arrow_ipc::convert::IpcSchemaEncoder::with_dictionary_tracker` · arrow-ipc 59.3.0

```rust
fn with_dictionary_tracker(self, dictionary_tracker: &'a mut DictionaryTracker) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::convert::IpcSchemaEncoder", "path": "IpcSchemaEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [129, 2], "filename": "src/convert.rs"}, "trait": null, "trait_path": null}`

Source: `src/convert.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specify a dictionary tracker to use
