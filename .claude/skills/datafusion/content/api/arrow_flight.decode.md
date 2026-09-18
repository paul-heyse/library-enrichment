# `arrow_flight::decode`

Crate `arrow-flight` · 4 public items · structured records in [`model/arrow_flight.decode.json`](../model/arrow_flight.decode.json)

## DecodedPayload

`enum` · `arrow_flight::decode::DecodedPayload`

```rust
enum DecodedPayload
```

**Variants**: `None`, `Schema`, `RecordBatch`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/arrow_flight.decode.DecodedPayload.md).


The result of decoding [`FlightData`]

---

## DecodedFlightData

`struct` · `arrow_flight::decode::DecodedFlightData`

```rust
struct DecodedFlightData
```

**Fields**: `inner`, `payload`

**Derives**: Debug

**Methods** (4)

```rust
fn app_metadata(&self) -> Bytes
fn new_none(inner: FlightData) -> Self
fn new_record_batch(inner: FlightData, batch: RecordBatch) -> Self
fn new_schema(inner: FlightData, schema: SchemaRef) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.decode.DecodedFlightData.md).


FlightData and the decoded payload (Schema, RecordBatch), if any

---

## FlightDataDecoder

`struct` · `arrow_flight::decode::FlightDataDecoder`

```rust
struct FlightDataDecoder
```

**Implements**: `futures_core::stream::Stream`

**Derives**: Debug

**Methods** (3)

```rust
fn new<S>(response: S) -> Self where S: Stream<Item = Result<FlightData>> + Send + 'static
fn schema(&self) -> Option<&SchemaRef>
unsafe fn with_skip_validation(self) -> Self
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.decode.FlightDataDecoder.md).


Wrapper around a stream of [`FlightData`] that handles the details
of decoding low level Flight messages into [`Schema`] and
[`RecordBatch`]es, including details such as dictionaries.

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

---

## FlightRecordBatchStream

`struct` · `arrow_flight::decode::FlightRecordBatchStream`

```rust
struct FlightRecordBatchStream
```

**Implements**: `futures_core::stream::Stream`

**Derives**: Debug

**Methods** (8)

```rust
fn headers(&self) -> &MetadataMap
fn into_inner(self) -> FlightDataDecoder
fn new(inner: FlightDataDecoder) -> Self
fn new_from_flight_data<S>(inner: S) -> Self where S: Stream<Item = Result<FlightData>> + Send + 'static
fn schema(&self) -> Option<&SchemaRef>
fn trailers(&self) -> Option<MetadataMap>
fn with_headers(self, headers: MetadataMap) -> Self
fn with_trailers(self, trailers: LazyTrailers) -> Self
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Result<RecordBatch>>>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.decode.FlightRecordBatchStream.md).


Decodes a [Stream] of [`FlightData`] back into
[`RecordBatch`]es. This can be used to decode the response from an
Arrow Flight server

# Note
To access the lower level Flight messages (e.g. to access
[`FlightData::app_metadata`]), you can call [`Self::into_inner`]
and use the [`FlightDataDecoder`] directly.

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

---
