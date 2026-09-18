# `arrow_flight::decode::FlightDataDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.decode.FlightDataDecoder.json).

<a id="op-38627753e30ce7cc30d78250"></a>
## FlightDataDecoder

`struct` · `arrow_flight::decode::FlightDataDecoder` · arrow-flight 59.3.0

```rust
struct FlightDataDecoder
```

Source: `src/decode.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Wrapper around a stream of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) that handles the details
of decoding low level Flight messages into [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) and
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es, including details such as dictionaries.

# Protocol Details

The client handles flight messages as followes:

- **None:** This message has no effect. This is useful to
  transmit metadata without any actual payload.

- **Schema:** The schema is (re-)set. Dictionaries are cleared and
  the decoded schema is returned.

- **Dictionary Batch:** A new dictionary for a given column is registered. An existing
  dictionary for the same column will be overwritten. This
  message is NOT visible.

- **Record Batch:** Record batch is created based on the current
  schema and dictionaries. This fails if no schema was transmitted
  yet.

All other message types (at the time of writing: e.g. tensor and
sparse tensor) lead to an error.

Example usecases

1. Using this low level stream it is possible to receive a steam
   of RecordBatches in FlightData that have different schemas by
   handling multiple schema messages separately.

<a id="op-ef4a44f44ad916539ec39c4c"></a>
## Item

`assoc_type` · `arrow_flight::decode::FlightDataDecoder::Item` · arrow-flight 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightDataDecoder", "path": "FlightDataDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [373, 1], "end": [404, 2], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/decode.rs:374`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4773beabb07bc665916bc836"></a>
## fmt

`function` · `arrow_flight::decode::FlightDataDecoder::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightDataDecoder", "path": "FlightDataDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 1], "end": [245, 2], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decode.rs:237`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ff951b726069e37f3af0714"></a>
## new

`function` · `arrow_flight::decode::FlightDataDecoder::new` · arrow-flight 59.3.0

```rust
fn new<S>(response: S) -> Self where S: Stream<Item = Result<FlightData>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightDataDecoder", "path": "FlightDataDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 1], "end": [371, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:249`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new wrapper around the stream of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5)

<a id="op-898abe2840b525fb0dbfba97"></a>
## poll_next

`function` · `arrow_flight::decode::FlightDataDecoder::poll_next` · arrow-flight 59.3.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightDataDecoder", "path": "FlightDataDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [373, 1], "end": [404, 2], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/decode.rs:378`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the result of decoding the next [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) message
from the server, or `None` if there are no further results
available.

<a id="op-7c96a3e293881937078c68d0"></a>
## schema

`function` · `arrow_flight::decode::FlightDataDecoder::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> Option<&SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightDataDecoder", "path": "FlightDataDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 1], "end": [371, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:269`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the current schema for this stream

<a id="op-319f2a93c107b79aa7a65e8e"></a>
## with_skip_validation

`function` · `arrow_flight::decode::FlightDataDecoder::with_skip_validation` · arrow-flight 59.3.0

```rust
unsafe fn with_skip_validation(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightDataDecoder", "path": "FlightDataDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 1], "end": [371, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

# Safety
Invalid data may cause undefined behavior. Only use for trusted senders.
