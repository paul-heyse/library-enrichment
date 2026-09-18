# `arrow_flight::gen::FlightData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.FlightData.json).

<a id="op-d385f038797edc1454fc35a5"></a>
## FlightData

`struct` · `arrow_flight::gen::FlightData` · arrow-flight 59.3.0

```rust
struct FlightData
```

Source: `src/arrow.flight.protocol.rs:344`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A batch of Arrow data as part of a stream of batches.

<a id="op-28bbc79c13d963e87e09ea1b"></a>
## app_metadata

`struct_field` · `arrow_flight::gen::FlightData::app_metadata` · arrow-flight 59.3.0

```rust
app_metadata: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Application-defined metadata.

<a id="op-1a928b714f8e5ba19a682103"></a>
## clear

`function` · `arrow_flight::gen::FlightData::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 38], "end": [343, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29d60dd0fbb9e0c6a4ead87f"></a>
## clone

`function` · `arrow_flight::gen::FlightData::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> FlightData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 10], "end": [343, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f60777eb4f22cbef8b95cb6"></a>
## data_body

`struct_field` · `arrow_flight::gen::FlightData::data_body` · arrow-flight 59.3.0

```rust
data_body: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:364`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The actual batch of Arrow data. Preferably handled with minimal-copies
coming last in the definition to help with sidecar patterns (it is
expected that some implementations will fetch this field off the wire
with specialized code to avoid extra memory copies).

<a id="op-d29806900d8159405b51af9d"></a>
## data_header

`struct_field` · `arrow_flight::gen::FlightData::data_header` · arrow-flight 59.3.0

```rust
data_header: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:353`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Header for message data as described in Message.fbs::Message.

<a id="op-9144764ce745a9bd6a278f6a"></a>
## default

`function` · `arrow_flight::gen::FlightData::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 38], "end": [343, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb5a075a97bcfa0627b97a34"></a>
## encoded_len

`function` · `arrow_flight::gen::FlightData::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 38], "end": [343, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8e9c4ad7e0b86e4f9d55cd9"></a>
## eq

`function` · `arrow_flight::gen::FlightData::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &FlightData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 17], "end": [343, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-965ce8329b1ee3953f452900"></a>
## flight_descriptor

`struct_field` · `arrow_flight::gen::FlightData::flight_descriptor` · arrow-flight 59.3.0

```rust
flight_descriptor: ::core::option::Option<FlightDescriptor>
```

Source: `src/arrow.flight.protocol.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The descriptor of the data. This is only relevant when a client is
starting a new DoPut stream.

<a id="op-31e1be119507b3eb16617068"></a>
## fmt

`function` · `arrow_flight::gen::FlightData::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 1], "end": [209, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-393675d39d1d76c42250a1ed"></a>
## fmt

`function` · `arrow_flight::gen::FlightData::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 38], "end": [343, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6301a912361694224f72e2a5"></a>
## from

`function` · `arrow_flight::gen::FlightData::from` · arrow-flight 59.3.0

```rust
fn from(data: EncodedData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [374, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::EncodedData", "path": "EncodedData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:367`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8ce2eb048c630ce7fdecaf1"></a>
## from

`function` · `arrow_flight::gen::FlightData::from` · arrow-flight 59.3.0

```rust
fn from(schema_ipc: SchemaAsIpc<'_>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [384, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0934e095e42a9a6aa0507a41"></a>
## hash

`function` · `arrow_flight::gen::FlightData::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 32], "end": [343, 36], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-590dc7ac4d0af224a81a238b"></a>
## new

`function` · `arrow_flight::gen::FlightData::new` · arrow-flight 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [514, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:485`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5).

# See Also

See [`FlightDataEncoderBuilder`] for a higher level API to
convert a stream of [`RecordBatch`]es to [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5)s

# Example:

```
# use bytes::Bytes;
# use arrow_flight::{FlightData, FlightDescriptor};
# fn encode_data() -> Bytes { Bytes::new() } // dummy data
// Get encoded Arrow IPC data:
let data_body: Bytes = encode_data();
// Create the FlightData message
let flight_data = FlightData::new()
  .with_descriptor(FlightDescriptor::new_cmd("the command"))
  .with_app_metadata("My apps metadata")
  .with_data_body(data_body);
```

[`FlightDataEncoderBuilder`]: crate::encode::FlightDataEncoderBuilder
[`RecordBatch`]: arrow_array::RecordBatch

<a id="op-e1ab8f7e9edf9062699fc0c1"></a>
## with_app_metadata

`function` · `arrow_flight::gen::FlightData::with_app_metadata` · arrow-flight 59.3.0

```rust
fn with_app_metadata(self, app_metadata: impl Into<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [514, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add optional application specific metadata to the message

<a id="op-21c97b7d4e5574cb4d7c483f"></a>
## with_data_body

`function` · `arrow_flight::gen::FlightData::with_data_body` · arrow-flight 59.3.0

```rust
fn with_data_body(self, data_body: impl Into<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [514, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add a data body. See [`IpcDataGenerator`] to create this data.

[`IpcDataGenerator`]: arrow_ipc::writer::IpcDataGenerator

<a id="op-a403ecb092e3729c6679d1d7"></a>
## with_data_header

`function` · `arrow_flight::gen::FlightData::with_data_header` · arrow-flight 59.3.0

```rust
fn with_data_header(self, data_header: impl Into<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [514, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:496`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add a data header

<a id="op-904eee9d1852fb798d046c31"></a>
## with_descriptor

`function` · `arrow_flight::gen::FlightData::with_descriptor` · arrow-flight 59.3.0

```rust
fn with_descriptor(self, flight_descriptor: FlightDescriptor) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightData", "path": "FlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [514, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:490`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add a [`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326) describing the data
