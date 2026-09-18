# `arrow_flight::encode::FlightDataEncoderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.encode.FlightDataEncoderBuilder.json).

<a id="op-417e00a5164d321214a222fc"></a>
## FlightDataEncoderBuilder

`struct` · `arrow_flight::encode::FlightDataEncoderBuilder` · arrow-flight 59.3.0

```rust
struct FlightDataEncoderBuilder
```

Source: `src/encode.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates a [`Stream`] of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5)s from a
`Stream` of [`Result`](../operations/arrow_flight.error.Result.md#op-946b2547adca3a863c890c3c)<[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34), [`FlightError`]>.

This can be used to implement [`FlightService::do_get`] in an
Arrow Flight implementation;

This structure encodes a stream of `Result`s rather than `RecordBatch`es  to
propagate errors from streaming execution, where the generation of the
`RecordBatch`es is incremental, and an error may occur even after
several have already been successfully produced.

# Caveats
1. When [`DictionaryHandling`](../operations/arrow_flight.encode.DictionaryHandling.md#op-3909e2ed806a45e433fc92ad) is [`DictionaryHandling::Hydrate`](../operations/arrow_flight.encode.DictionaryHandling.md#op-1274858835cec039b729e6f3),
   [`DictionaryArray`]s are converted to their underlying types prior to
   transport.
   When [`DictionaryHandling`](../operations/arrow_flight.encode.DictionaryHandling.md#op-3909e2ed806a45e433fc92ad) is [`DictionaryHandling::Resend`](../operations/arrow_flight.encode.DictionaryHandling.md#op-4ca86ac204d7df4411001d3a), Dictionary [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) is sent with every
   [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) that contains a [`DictionaryArray`](arrow_array::array::DictionaryArray).
   See <https://github.com/apache/arrow-rs/issues/3389>.

[`DictionaryArray`]: arrow_array::array::DictionaryArray

# Example
```no_run
# use std::sync::Arc;
# use arrow_array::{ArrayRef, RecordBatch, UInt32Array};
# async fn f() {
# let c1 = UInt32Array::from(vec![1, 2, 3, 4, 5, 6]);
# let batch = RecordBatch::try_from_iter(vec![
#      ("a", Arc::new(c1) as ArrayRef)
#   ])
#   .expect("cannot create record batch");
use arrow_flight::encode::FlightDataEncoderBuilder;

// Get an input stream of Result<RecordBatch, FlightError>
let input_stream = futures::stream::iter(vec![Ok(batch)]);

// Build a stream of `Result<FlightData>` (e.g. to return for do_get)
let flight_data_stream = FlightDataEncoderBuilder::new()
 .build(input_stream);

// Create a tonic `Response` that can be returned from a Flight server
let response = tonic::Response::new(flight_data_stream);
# }
```

# Example: Sending `Vec<RecordBatch>`

You can create a [`Stream`] to pass to [`Self::build`](../operations/arrow_flight.encode.FlightDataEncoderBuilder.md#op-b487052b2bbe55e641cbd40b) from an existing
`Vec` of `RecordBatch`es like this:

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, RecordBatch, UInt32Array};
# async fn f() {
# fn make_batches() -> Vec<RecordBatch> {
#   let c1 = UInt32Array::from(vec![1, 2, 3, 4, 5, 6]);
#   let batch = RecordBatch::try_from_iter(vec![
#      ("a", Arc::new(c1) as ArrayRef)
#   ])
#   .expect("cannot create record batch");
#   vec![batch.clone(), batch.clone()]
# }
use arrow_flight::encode::FlightDataEncoderBuilder;

// Get batches that you want to send via Flight
let batches: Vec<RecordBatch> = make_batches();

// Create an input stream of Result<RecordBatch, FlightError>
let input_stream = futures::stream::iter(
  batches.into_iter().map(Ok)
);

// Build a stream of `Result<FlightData>` (e.g. to return for do_get)
let flight_data_stream = FlightDataEncoderBuilder::new()
 .build(input_stream);
# }
```

# Example: Determining schema of encoded data

Encoding flight data may hydrate dictionaries, see [`DictionaryHandling`](../operations/arrow_flight.encode.DictionaryHandling.md#op-3909e2ed806a45e433fc92ad) for more information,
which changes the schema of the encoded data compared to the input record batches.
The fully hydrated schema can be accessed using the [`FlightDataEncoder::known_schema`](../operations/arrow_flight.encode.FlightDataEncoder.md#op-9072e717861c149fdfc554ad) method
and explicitly informing the builder of the schema using [`FlightDataEncoderBuilder::with_schema`](../operations/arrow_flight.encode.FlightDataEncoderBuilder.md#op-54884f346f4f7fe90f915141).

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, RecordBatch, UInt32Array};
# async fn f() {
# let c1 = UInt32Array::from(vec![1, 2, 3, 4, 5, 6]);
# let batch = RecordBatch::try_from_iter(vec![
#      ("a", Arc::new(c1) as ArrayRef)
#   ])
#   .expect("cannot create record batch");
use arrow_flight::encode::FlightDataEncoderBuilder;

// Get the schema of the input stream
let schema = batch.schema();

// Get an input stream of Result<RecordBatch, FlightError>
let input_stream = futures::stream::iter(vec![Ok(batch)]);

// Build a stream of `Result<FlightData>` (e.g. to return for do_get)
let flight_data_stream = FlightDataEncoderBuilder::new()
 // Inform the builder of the input stream schema
 .with_schema(schema)
 .build(input_stream);

// Retrieve the schema of the encoded data
let encoded_schema = flight_data_stream.known_schema();
# }
```

[`FlightService::do_get`]: crate::flight_service_server::FlightService::do_get
[`FlightError`]: crate::error::FlightError

Unresolved upstream links (retained, not inferred): ``Stream``.

<a id="op-b487052b2bbe55e641cbd40b"></a>
## build

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::build` · arrow-flight 59.3.0

```rust
fn build<S>(self, input: S) -> FlightDataEncoder where S: Stream<Item = Result<RecordBatch>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:241`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Takes a [`Stream`] of [`Result<RecordBatch>`](../operations/arrow_flight.error.Result.md#op-946b2547adca3a863c890c3c) and returns a [`Stream`]
of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5), consuming self.

See example on [`Self`](../operations/arrow_flight.encode.FlightDataEncoderBuilder.md#op-417e00a5164d321214a222fc) and [`FlightDataEncoder`](../operations/arrow_flight.encode.FlightDataEncoder.md#op-e4d1d137c901fe86782ae657) for more details

Unresolved upstream links (retained, not inferred): ``Stream``.

<a id="op-ef5ab83060002452ed58c2a4"></a>
## default

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [179, 2], "filename": "src/encode.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/encode.rs:169`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f896efecbbadd2ec8be5a8f"></a>
## fmt

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 10], "end": [144, 15], "filename": "src/encode.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/encode.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad22e330b2987d1d993801ee"></a>
## new

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::new` · arrow-flight 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new [`FlightDataEncoderBuilder`](../operations/arrow_flight.encode.FlightDataEncoderBuilder.md#op-417e00a5164d321214a222fc).

<a id="op-58c6a2c13837a7ebf197954f"></a>
## with_dictionary_handling

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::with_dictionary_handling` · arrow-flight 59.3.0

```rust
fn with_dictionary_handling(self, dictionary_handling: DictionaryHandling) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:203`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set [`DictionaryHandling`](../operations/arrow_flight.encode.DictionaryHandling.md#op-3909e2ed806a45e433fc92ad) for encoder

<a id="op-7ad249c516e7382d5702764a"></a>
## with_flight_descriptor

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::with_flight_descriptor` · arrow-flight 59.3.0

```rust
fn with_flight_descriptor(self, descriptor: Option<FlightDescriptor>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:232`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specify a flight descriptor in the first FlightData message.

<a id="op-8a1ebf04767905b3e0ab5421"></a>
## with_max_flight_data_size

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::with_max_flight_data_size` · arrow-flight 59.3.0

```rust
fn with_max_flight_data_size(self, max_flight_data_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:197`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set the (approximate) maximum size, in bytes, of the
[`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) produced by this encoder. Defaults to 2MB.

Since there is often a maximum message size for gRPC messages
(typically around 4MB), this encoder splits up [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)s
(preserving order) into multiple [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) objects to
limit the size individual messages sent via gRPC.

The size is approximate because of the additional encoding
overhead on top of the underlying data buffers themselves.

<a id="op-0c291bad794763267d00af68"></a>
## with_metadata

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::with_metadata` · arrow-flight 59.3.0

```rust
fn with_metadata(self, app_metadata: Bytes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:211`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specify application specific metadata included in the
[`FlightData::app_metadata`](../operations/arrow_flight.gen.FlightData.md#op-28bbc79c13d963e87e09ea1b) field of the the first Schema
message

<a id="op-cf0807748a6dcd1a000ac2dd"></a>
## with_options

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::with_options` · arrow-flight 59.3.0

```rust
fn with_options(self, options: IpcWriteOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:217`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set the [`IpcWriteOptions`](../operations/arrow_ipc.writer.IpcWriteOptions.md#op-7958df777a3d4faa0bb68d5a) used to encode the [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es for transport.

<a id="op-54884f346f4f7fe90f915141"></a>
## with_schema

`function` · `arrow_flight::encode::FlightDataEncoderBuilder::with_schema` · arrow-flight 59.3.0

```rust
fn with_schema(self, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoderBuilder", "path": "FlightDataEncoderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [264, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:226`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specify a schema for the RecordBatches being sent. If a schema
is not specified, an encoded Schema message will be sent when
the first [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34), if any, is encoded. Some clients
expect a Schema message even if there is no data sent.
