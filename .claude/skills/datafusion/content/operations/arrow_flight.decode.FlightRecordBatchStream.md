# `arrow_flight::decode::FlightRecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.decode.FlightRecordBatchStream.json).

<a id="op-21b308cb3e3a13c8cb4fc4c0"></a>
## FlightRecordBatchStream

`struct` · `arrow_flight::decode::FlightRecordBatchStream` · arrow-flight 59.3.0

```rust
struct FlightRecordBatchStream
```

Source: `src/decode.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Decodes a [Stream] of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) back into
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es. This can be used to decode the response from an
Arrow Flight server

# Note
To access the lower level Flight messages (e.g. to access
[`FlightData::app_metadata`](../operations/arrow_flight.gen.FlightData.md#op-28bbc79c13d963e87e09ea1b)), you can call [`Self::into_inner`](../operations/arrow_flight.decode.FlightRecordBatchStream.md#op-b3dd3be088628416debb9120)
and use the [`FlightDataDecoder`](../operations/arrow_flight.decode.FlightDataDecoder.md#op-38627753e30ce7cc30d78250) directly.

# Example:
```no_run
# async fn f() -> Result<(), arrow_flight::error::FlightError>{
# use bytes::Bytes;
// make a do_get request
use arrow_flight::{
  error::Result,
  decode::FlightRecordBatchStream,
  Ticket,
  flight_service_client::FlightServiceClient
};
use tonic::transport::Channel;
use futures::stream::{StreamExt, TryStreamExt};

let client: FlightServiceClient<Channel> = // make client..
# unimplemented!();

let request = tonic::Request::new(
  Ticket { ticket: Bytes::new() }
);

// Get a stream of FlightData;
let flight_data_stream = client
  .do_get(request)
  .await?
  .into_inner();

// Decode stream of FlightData to RecordBatches
let record_batch_stream = FlightRecordBatchStream::new_from_flight_data(
  // convert tonic::Status to FlightError
  flight_data_stream.map_err(|e| e.into())
);

// Read back RecordBatches
while let Some(batch) = record_batch_stream.next().await {
  match batch {
    Ok(batch) => { /* process batch */ },
    Err(e) => { /* handle error */ },
  };
}

# Ok(())
# }
```

Unresolved upstream links (retained, not inferred): `Stream`.

<a id="op-6285c5e754c7374bf79e5b46"></a>
## Item

`assoc_type` · `arrow_flight::decode::FlightRecordBatchStream::Item` · arrow-flight 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [193, 2], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/decode.rs:154`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3baf1e280a908c1e5fe512f8"></a>
## fmt

`function` · `arrow_flight::decode::FlightRecordBatchStream::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 10], "end": [83, 15], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decode.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c0c4e597884e8ef27856f84"></a>
## headers

`function` · `arrow_flight::decode::FlightRecordBatchStream::headers` · arrow-flight 59.3.0

```rust
fn headers(&self) -> &MetadataMap
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Headers attached to this stream.

<a id="op-b3dd3be088628416debb9120"></a>
## into_inner

`function` · `arrow_flight::decode::FlightRecordBatchStream::into_inner` · arrow-flight 59.3.0

```rust
fn into_inner(self) -> FlightDataDecoder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Consume self and return the wrapped [`FlightDataDecoder`](../operations/arrow_flight.decode.FlightDataDecoder.md#op-38627753e30ce7cc30d78250)

<a id="op-5ecb0e7df32c036f4e1b10e4"></a>
## new

`function` · `arrow_flight::decode::FlightRecordBatchStream::new` · arrow-flight 59.3.0

```rust
fn new(inner: FlightDataDecoder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new [`FlightRecordBatchStream`](../operations/arrow_flight.decode.FlightRecordBatchStream.md#op-21b308cb3e3a13c8cb4fc4c0) from a decoded stream

<a id="op-42823723baf24cdaa4b670d5"></a>
## new_from_flight_data

`function` · `arrow_flight::decode::FlightRecordBatchStream::new_from_flight_data` · arrow-flight 59.3.0

```rust
fn new_from_flight_data<S>(inner: S) -> Self where S: Stream<Item = Result<FlightData>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:105`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new [`FlightRecordBatchStream`](../operations/arrow_flight.decode.FlightRecordBatchStream.md#op-21b308cb3e3a13c8cb4fc4c0) from a stream of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5)

<a id="op-04d3e464cd06d165b61a236d"></a>
## poll_next

`function` · `arrow_flight::decode::FlightRecordBatchStream::poll_next` · arrow-flight 59.3.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Result<RecordBatch>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [193, 2], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/decode.rs:158`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the next [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) available in this stream, or `None` if
there are no further results available.

<a id="op-92c356358efbf4dacfe9b064"></a>
## schema

`function` · `arrow_flight::decode::FlightRecordBatchStream::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> Option<&SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return schema for the stream, if it has been received

<a id="op-37cbb5ac79be01d61bc1eebd"></a>
## trailers

`function` · `arrow_flight::decode::FlightRecordBatchStream::trailers` · arrow-flight 59.3.0

```rust
fn trailers(&self) -> Option<MetadataMap>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Trailers attached to this stream.

Note that this will return `None` until the entire stream is consumed.
Only after calling `next()` returns `None`, might any available trailers be returned.

<a id="op-526f5da3a796c880a272625e"></a>
## with_headers

`function` · `arrow_flight::decode::FlightRecordBatchStream::with_headers` · arrow-flight 59.3.0

```rust
fn with_headers(self, headers: MetadataMap) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Record response headers.

<a id="op-220983776127f249a4701318"></a>
## with_trailers

`function` · `arrow_flight::decode::FlightRecordBatchStream::with_trailers` · arrow-flight 59.3.0

```rust
fn with_trailers(self, trailers: LazyTrailers) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::FlightRecordBatchStream", "path": "FlightRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [151, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Record response trailers.
