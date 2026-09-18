# `arrow_flight::encode`

Crate `arrow-flight` · 4 public items · structured records in [`model/arrow_flight.encode.json`](../model/arrow_flight.encode.json)

## GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES

`constant` · `arrow_flight::encode::GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES`

```rust
const GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES: usize = 2097152
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.encode.GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES.md).


Default target size for encoded [`FlightData`].

Note this value would normally be 4MB, but the size calculation is
somewhat inexact, so we set it to 2MB.

---

## DictionaryHandling

`enum` · `arrow_flight::encode::DictionaryHandling`

```rust
enum DictionaryHandling
```

**Variants**: `Hydrate`, `Resend`

**Derives**: Debug, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/arrow_flight.encode.DictionaryHandling.md).


Defines how a [`FlightDataEncoder`] encodes [`DictionaryArray`]s

[`DictionaryArray`]: arrow_array::DictionaryArray

In the arrow flight protocol dictionary values and keys are sent as two separate messages.
When a sender is encoding a [`RecordBatch`] containing ['DictionaryArray'] columns, it will
first send a dictionary batch (a batch with header `MessageHeader::DictionaryBatch`) containing
the dictionary values. The receiver is responsible for reading this batch and maintaining state that associates
those dictionary values with the corresponding array using the `dict_id` as a key.

After sending the dictionary batch the sender will send the array data in a batch with header `MessageHeader::RecordBatch`.
For any dictionary array batches in this message, the encoded flight message will only contain the dictionary keys. The receiver
is then responsible for rebuilding the `DictionaryArray` on the client side using the dictionary values from the DictionaryBatch message
and the keys from the RecordBatch message.

For example, if we have a batch with a `TypedDictionaryArray<'_, UInt32Type, Utf8Type>` (a dictionary array where they keys are `u32` and the
values are `String`), then the DictionaryBatch will contain a `StringArray` and the RecordBatch will contain a `UInt32Array`.

Note that since `dict_id` defined in the `Schema` is used as a key to associate dictionary values to their arrays it is required that each
`DictionaryArray` in a `RecordBatch` have a unique `dict_id`.

The current implementation does not support "delta" dictionaries so a new dictionary batch will be sent each time the encoder sees a
dictionary which is not pointer-equal to the previously observed dictionary for a given `dict_id`.

For clients which may not support `DictionaryEncoding`, the `DictionaryHandling::Hydrate` method will bypass the process defined above
and "hydrate" any `DictionaryArray` in the batch to their underlying value type (e.g. `TypedDictionaryArray<'_, UInt32Type, Utf8Type>` will
be sent as a `StringArray`). With this method all data will be sent in ``MessageHeader::RecordBatch` messages and the batch schema
will be adjusted so that all dictionary encoded fields are changed to fields of the dictionary value type.

---

## FlightDataEncoder

`struct` · `arrow_flight::encode::FlightDataEncoder`

```rust
struct FlightDataEncoder
```

**Implements**: `futures_core::stream::Stream`

**Methods** (1)

```rust
fn known_schema(&self) -> Option<SchemaRef>
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.encode.FlightDataEncoder.md).


Stream that encodes a stream of record batches to flight data.

See [`FlightDataEncoderBuilder`] for details and example.

---

## FlightDataEncoderBuilder

`struct` · `arrow_flight::encode::FlightDataEncoderBuilder`

```rust
struct FlightDataEncoderBuilder
```

**Derives**: Debug, Default

**Methods** (8)

```rust
fn build<S>(self, input: S) -> FlightDataEncoder where S: Stream<Item = Result<RecordBatch>> + Send + 'static
fn new() -> Self
fn with_dictionary_handling(self, dictionary_handling: DictionaryHandling) -> Self
fn with_flight_descriptor(self, descriptor: Option<FlightDescriptor>) -> Self
fn with_max_flight_data_size(self, max_flight_data_size: usize) -> Self
fn with_metadata(self, app_metadata: Bytes) -> Self
fn with_options(self, options: IpcWriteOptions) -> Self
fn with_schema(self, schema: SchemaRef) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.encode.FlightDataEncoderBuilder.md).


Creates a [`Stream`] of [`FlightData`]s from a
`Stream` of [`Result`]<[`RecordBatch`], [`FlightError`]>.

This can be used to implement [`FlightService::do_get`] in an
Arrow Flight implementation;

This structure encodes a stream of `Result`s rather than `RecordBatch`es  to
propagate errors from streaming execution, where the generation of the
`RecordBatch`es is incremental, and an error may occur even after
several have already been successfully produced.

# Caveats
1. When [`DictionaryHandling`] is [`DictionaryHandling::Hydrate`],
   [`DictionaryArray`]s are converted to their underlying types prior to
   transport.
   When [`DictionaryHandling`] is [`DictionaryHandling::Resend`], Dictionary [`FlightData`] is sent with every
   [`RecordBatch`] that contains a [`DictionaryArray`](arrow_array::array::DictionaryArray).
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

You can create a [`Stream`] to pass to [`Self::build`] from an existing
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

Encoding flight data may hydrate dictionaries, see [`DictionaryHandling`] for more information,
which changes the schema of the encoded data compared to the input record batches.
The fully hydrated schema can be accessed using the [`FlightDataEncoder::known_schema`] method
and explicitly informing the builder of the schema using [`FlightDataEncoderBuilder::with_schema`].

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

---
