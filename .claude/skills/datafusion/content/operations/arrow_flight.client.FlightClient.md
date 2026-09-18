# `arrow_flight::client::FlightClient`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.client.FlightClient.json).

<a id="op-fc031911df2305bdb403fbc0"></a>
## FlightClient

`struct` · `arrow_flight::client::FlightClient` · arrow-flight 59.3.0

```rust
struct FlightClient<T = tonic::transport::Channel>
```

Source: `src/client.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A "Mid level" [Apache Arrow Flight](https://arrow.apache.org/docs/format/Flight.html) client.

[`FlightClient`](../operations/arrow_flight.client.FlightClient.md#op-fc031911df2305bdb403fbc0) is intended as a convenience for interactions
with Arrow Flight servers. For more direct control, such as access
to the response headers, use  [`FlightServiceClient`](../operations/arrow_flight.gen.flight_service_client.FlightServiceClient.md#op-8e3fc5d811e76f6be8a947c7) directly
via methods such as [`Self::inner`](../operations/arrow_flight.client.FlightClient.md#op-7988a4cd019ff4ff3b475fc4) or [`Self::into_inner`](../operations/arrow_flight.client.FlightClient.md#op-044bd85e2e9647ee7ac762cb).

# Example:
```no_run
# async fn run() {
# use arrow_flight::FlightClient;
# use bytes::Bytes;
use tonic::transport::Channel;
let channel = Channel::from_static("http://localhost:1234")
  .connect()
  .await
  .expect("error connecting");

let mut client = FlightClient::new(channel);

// Send 'Hi' bytes as the handshake request to the server
let response = client
  .handshake(Bytes::from("Hi"))
  .await
  .expect("error handshaking");

// Expect the server responded with 'Ho'
assert_eq!(response, Bytes::from("Ho"));
# }
```

<a id="op-296b0b28296201552eb9d670"></a>
## add_header

`function` · `arrow_flight::client::FlightClient::add_header` · arrow-flight 59.3.0

```rust
fn add_header(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add the specified header with value to all subsequent
requests. See [`Self::metadata_mut`](../operations/arrow_flight.client.FlightClient.md#op-cc94e7b8a05ff08989ce250f) for fine grained control.

<a id="op-a26d63c05c7af2a1172c5715"></a>
## cancel_flight_info

`function` · `arrow_flight::client::FlightClient::cancel_flight_info` · arrow-flight 59.3.0

```rust
async fn cancel_flight_info(&mut self, request: CancelFlightInfoRequest) -> Result<CancelFlightInfoResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:622`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `CancelFlightInfo` call to the server and return
a [`CancelFlightInfoResult`](../operations/arrow_flight.gen.CancelFlightInfoResult.md#op-3b9cf25276fb4bb5ded28371).

# Example:
```no_run
# async fn run() {
# use arrow_flight::{CancelFlightInfoRequest, FlightClient, FlightDescriptor};
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// Send a 'CMD' request to the server
let request = FlightDescriptor::new_cmd(b"MOAR DATA".to_vec());
let flight_info = client
  .get_flight_info(request)
  .await
  .expect("error handshaking");

// Cancel the query
let request = CancelFlightInfoRequest::new(flight_info);
let result = client
  .cancel_flight_info(request)
  .await
  .expect("error cancelling");
# }
```

<a id="op-319c3a296f2440b9ad3de5f2"></a>
## do_action

`function` · `arrow_flight::client::FlightClient::do_action` · arrow-flight 59.3.0

```rust
async fn do_action(&mut self, action: Action) -> Result<BoxStream<'static, Result<Bytes>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:577`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `DoAction` call to the server and returning a
[`Stream`] of opaque [`Bytes`].

# Example:
```no_run
# async fn run() {
# use bytes::Bytes;
# use futures::TryStreamExt;
# use arrow_flight::{Action, FlightClient};
# use arrow_schema::Schema;
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

let request = Action::new("my_action", "the body");

// Make a request to run the action on the server
let results: Vec<Bytes> = client
  .do_action(request)
  .await
  .expect("error executing acton")
  .try_collect() // use TryStreamExt to collect stream
  .await
  .expect("error gathering action results");
# }
```

Unresolved upstream links (retained, not inferred): ``Bytes``, ``Stream``.

<a id="op-4e97cdae0c3e2648313c0e36"></a>
## do_exchange

`function` · `arrow_flight::client::FlightClient::do_exchange` · arrow-flight 59.3.0

```rust
async fn do_exchange<S: Stream<Item = Result<FlightData>> + Send + 'static>(&mut self, request: S) -> Result<FlightRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:422`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `DoExchange` call to the server with the provided
[`Stream`] of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) and returning a
stream of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5).

# Example:
```no_run
# async fn run() {
# use futures::{TryStreamExt, StreamExt};
# use std::sync::Arc;
# use arrow_array::UInt64Array;
# use arrow_array::RecordBatch;
# use arrow_flight::{FlightClient, FlightDescriptor, PutResult};
# use arrow_flight::encode::FlightDataEncoderBuilder;
# let batch = RecordBatch::try_from_iter(vec![
#  ("col2", Arc::new(UInt64Array::from_iter([10, 23, 33])) as _)
# ]).unwrap();
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// encode the batch as a stream of `FlightData`
let flight_data_stream = FlightDataEncoderBuilder::new()
  .build(futures::stream::iter(vec![Ok(batch)]));

// send the stream and get the results as `RecordBatches`
let response: Vec<RecordBatch> = client
  .do_exchange(flight_data_stream)
  .await
  .unwrap()
  .try_collect() // use TryStreamExt to collect stream
  .await
  .expect("error calling do_exchange");
# }
```

Unresolved upstream links (retained, not inferred): ``Stream``.

<a id="op-f6a796a1c248c2da88a515e0"></a>
## do_get

`function` · `arrow_flight::client::FlightClient::do_get` · arrow-flight 59.3.0

```rust
async fn do_get(&mut self, ticket: Ticket) -> Result<FlightRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:213`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `DoGet` call to the server with the provided ticket,
returning a [`FlightRecordBatchStream`](../operations/arrow_flight.decode.FlightRecordBatchStream.md#op-21b308cb3e3a13c8cb4fc4c0) for reading
[`RecordBatch`](arrow_array::RecordBatch)es.

# Note

To access the returned [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) use
[`FlightRecordBatchStream::into_inner()`](../operations/arrow_flight.decode.FlightRecordBatchStream.md#op-b3dd3be088628416debb9120)

# Example:
```no_run
# async fn run() {
# use bytes::Bytes;
# use arrow_flight::FlightClient;
# use arrow_flight::Ticket;
# use arrow_array::RecordBatch;
# use futures::stream::TryStreamExt;
# let channel: tonic::transport::Channel = unimplemented!();
# let ticket = Ticket { ticket: Bytes::from("foo") };
let mut client = FlightClient::new(channel);

// Invoke a do_get request on the server with a previously
// received Ticket

let response = client
   .do_get(ticket)
   .await
   .expect("error invoking do_get");

// Use try_collect to get the RecordBatches from the server
let batches: Vec<RecordBatch> = response
   .try_collect()
   .await
   .expect("no stream errors");
# }
```

<a id="op-8543b9551f0791f8c8707137"></a>
## do_put

`function` · `arrow_flight::client::FlightClient::do_put` · arrow-flight 59.3.0

```rust
async fn do_put<S: Stream<Item = Result<FlightData>> + Send + 'static>(&mut self, request: S) -> Result<BoxStream<'static, Result<PutResult>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `DoPut` call to the server with the provided
[`Stream`] of [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) and returning a
stream of [`PutResult`](../operations/arrow_flight.gen.PutResult.md#op-b90b812a48ac8a0f1c069fdf).

# Note

The input stream is [`Result`](../operations/arrow_flight.error.Result.md#op-946b2547adca3a863c890c3c) so that this can be connected
to a streaming data source, such as [`FlightDataEncoder`](crate::encode::FlightDataEncoder),
without having to buffer. If the input stream returns an error
that error will not be sent to the server, instead it will be
placed into the result stream and the server connection
terminated.

# Example:
```no_run
# async fn run() {
# use futures::{TryStreamExt, StreamExt};
# use std::sync::Arc;
# use arrow_array::UInt64Array;
# use arrow_array::RecordBatch;
# use arrow_flight::{FlightClient, FlightDescriptor, PutResult};
# use arrow_flight::encode::FlightDataEncoderBuilder;
# let batch = RecordBatch::try_from_iter(vec![
#  ("col2", Arc::new(UInt64Array::from_iter([10, 23, 33])) as _)
# ]).unwrap();
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// encode the batch as a stream of `FlightData`
let flight_data_stream = FlightDataEncoderBuilder::new()
  .build(futures::stream::iter(vec![Ok(batch)]));

// send the stream and get the results as `PutResult`
let response: Vec<PutResult>= client
  .do_put(flight_data_stream)
  .await
  .unwrap()
  .try_collect() // use TryStreamExt to collect stream
  .await
  .expect("error calling do_put");
# }
```

Unresolved upstream links (retained, not inferred): ``Stream``.

<a id="op-705daa13f4ce72032bceaa77"></a>
## fmt

`function` · `arrow_flight::client::FlightClient::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 10], "end": [70, 15], "filename": "src/client.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7821e11bfd6da715bbbb1b04"></a>
## get_flight_info

`function` · `arrow_flight::client::FlightClient::get_flight_info` · arrow-flight 59.3.0

```rust
async fn get_flight_info(&mut self, descriptor: FlightDescriptor) -> Result<FlightInfo>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `GetFlightInfo` call to the server with the provided
[`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326) and return the [`FlightInfo`](../operations/arrow_flight.gen.FlightInfo.md#op-422a7589713733d24ed1403b) from the
server. The [`FlightInfo`](../operations/arrow_flight.gen.FlightInfo.md#op-422a7589713733d24ed1403b) can be used with [`Self::do_get`](../operations/arrow_flight.client.FlightClient.md#op-f6a796a1c248c2da88a515e0)
to retrieve the requested batches.

# Example:
```no_run
# async fn run() {
# use arrow_flight::FlightClient;
# use arrow_flight::FlightDescriptor;
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// Send a 'CMD' request to the server
let request = FlightDescriptor::new_cmd(b"MOAR DATA".to_vec());
let flight_info = client
  .get_flight_info(request)
  .await
  .expect("error handshaking");

// retrieve the first endpoint from the returned flight info
let ticket = flight_info
  .endpoint[0]
  // Extract the ticket
  .ticket
  .clone()
  .expect("expected ticket");

// Retrieve the corresponding RecordBatch stream with do_get
let data = client
  .do_get(ticket)
  .await
  .expect("error fetching data");
# }
```

<a id="op-018e7e123df0c5a889de917f"></a>
## get_schema

`function` · `arrow_flight::client::FlightClient::get_schema` · arrow-flight 59.3.0

```rust
async fn get_schema(&mut self, flight_descriptor: FlightDescriptor) -> Result<Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `GetSchema` call to the server with the provided
[`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326) and returning the associated [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050).

# Example:
```no_run
# async fn run() {
# use bytes::Bytes;
# use arrow_flight::{FlightDescriptor, FlightClient};
# use arrow_schema::Schema;
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// Request the schema result of a 'CMD' request to the server
let request = FlightDescriptor::new_cmd(b"MOAR DATA".to_vec());

let schema: Schema = client
  .get_schema(request)
  .await
  .expect("error making request");
# }
```

<a id="op-33916eafcd0f7186ae257a52"></a>
## handshake

`function` · `arrow_flight::client::FlightClient::handshake` · arrow-flight 59.3.0

```rust
async fn handshake(&mut self, payload: impl Into<Bytes>) -> Result<Bytes>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:152`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Perform an Arrow Flight handshake with the server, sending
`payload` as the [`HandshakeRequest`](../operations/arrow_flight.gen.HandshakeRequest.md#op-2a6d2ff8443d88db6fc9cdff) payload and returning
the [`HandshakeResponse`](crate::HandshakeResponse)
bytes returned from the server

See [`FlightClient`](../operations/arrow_flight.client.FlightClient.md#op-fc031911df2305bdb403fbc0) docs for an example.

<a id="op-7988a4cd019ff4ff3b475fc4"></a>
## inner

`function` · `arrow_flight::client::FlightClient::inner` · arrow-flight 59.3.0

```rust
fn inner(&self) -> &FlightServiceClient<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return a reference to the underlying tonic
[`FlightServiceClient`](../operations/arrow_flight.gen.flight_service_client.FlightServiceClient.md#op-8e3fc5d811e76f6be8a947c7)

<a id="op-b35d1db6d157981a59b6411c"></a>
## inner_mut

`function` · `arrow_flight::client::FlightClient::inner_mut` · arrow-flight 59.3.0

```rust
fn inner_mut(&mut self) -> &mut FlightServiceClient<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return a mutable reference to the underlying tonic
[`FlightServiceClient`](../operations/arrow_flight.gen.flight_service_client.FlightServiceClient.md#op-8e3fc5d811e76f6be8a947c7)

<a id="op-044bd85e2e9647ee7ac762cb"></a>
## into_inner

`function` · `arrow_flight::client::FlightClient::into_inner` · arrow-flight 59.3.0

```rust
fn into_inner(self) -> FlightServiceClient<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Consume this client and return the underlying tonic
[`FlightServiceClient`](../operations/arrow_flight.gen.flight_service_client.FlightServiceClient.md#op-8e3fc5d811e76f6be8a947c7)

<a id="op-9bd4ebc6ba6dbdc5c39d269e"></a>
## list_actions

`function` · `arrow_flight::client::FlightClient::list_actions` · arrow-flight 59.3.0

```rust
async fn list_actions(&mut self) -> Result<BoxStream<'static, Result<ActionType>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:539`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `ListActions` call to the server and returning a
[`Stream`] of [`ActionType`](../operations/arrow_flight.gen.ActionType.md#op-bd90750c7c512fab437de12f).

# Example:
```no_run
# async fn run() {
# use futures::TryStreamExt;
# use arrow_flight::{ActionType, FlightClient};
# use arrow_schema::Schema;
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// List available actions on the server:
let actions: Vec<ActionType> = client
  .list_actions()
  .await
  .expect("error listing actions")
  .try_collect() // use TryStreamExt to collect stream
  .await
  .expect("error gathering actions");
# }
```

Unresolved upstream links (retained, not inferred): ``Stream``.

<a id="op-72f0f4a18531837218132727"></a>
## list_flights

`function` · `arrow_flight::client::FlightClient::list_flights` · arrow-flight 59.3.0

```rust
async fn list_flights(&mut self, expression: impl Into<Bytes>) -> Result<BoxStream<'static, Result<FlightInfo>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:465`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `ListFlights` call to the server with the provided
criteria and returning a [`Stream`] of [`FlightInfo`](../operations/arrow_flight.gen.FlightInfo.md#op-422a7589713733d24ed1403b).

# Example:
```no_run
# async fn run() {
# use futures::TryStreamExt;
# use bytes::Bytes;
# use arrow_flight::{FlightInfo, FlightClient};
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// Send 'Name=Foo' bytes as the "expression" to the server
// and gather the returned FlightInfo
let responses: Vec<FlightInfo> = client
  .list_flights(Bytes::from("Name=Foo"))
  .await
  .expect("error listing flights")
  .try_collect() // use TryStreamExt to collect stream
  .await
  .expect("error gathering flights");
# }
```

Unresolved upstream links (retained, not inferred): ``Stream``.

<a id="op-419625315592d1d810862915"></a>
## metadata

`function` · `arrow_flight::client::FlightClient::metadata` · arrow-flight 59.3.0

```rust
fn metadata(&self) -> &MetadataMap
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return a reference to gRPC metadata included with each request

<a id="op-cc94e7b8a05ff08989ce250f"></a>
## metadata_mut

`function` · `arrow_flight::client::FlightClient::metadata_mut` · arrow-flight 59.3.0

```rust
fn metadata_mut(&mut self) -> &mut MetadataMap
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:108`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return a reference to gRPC metadata included with each request

These headers can be used, for example, to include
authorization or other application specific headers.

<a id="op-000c8f995f61b675c4169c69"></a>
## new

`function` · `arrow_flight::client::FlightClient::new` · arrow-flight 59.3.0

```rust
fn new(inner: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates a client with the provided transport

<a id="op-5977e2f25cc459a97b42a98d"></a>
## new_from_inner

`function` · `arrow_flight::client::FlightClient::new_from_inner` · arrow-flight 59.3.0

```rust
fn new_from_inner(inner: FlightServiceClient<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates a new higher level client with the provided lower level client

<a id="op-7d288a5c4577ea1dacb87dc5"></a>
## poll_flight_info

`function` · `arrow_flight::client::FlightClient::poll_flight_info` · arrow-flight 59.3.0

```rust
async fn poll_flight_info(&mut self, descriptor: FlightDescriptor) -> Result<PollInfo>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:319`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `PollFlightInfo` call to the server with the provided
[`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326) and return the [`PollInfo`](../operations/arrow_flight.gen.PollInfo.md#op-a5f3704ac23735382e4c2af6) from the
server.

The `info` field of the [`PollInfo`](../operations/arrow_flight.gen.PollInfo.md#op-a5f3704ac23735382e4c2af6) can be used with
[`Self::do_get`](../operations/arrow_flight.client.FlightClient.md#op-f6a796a1c248c2da88a515e0) to retrieve the requested batches.

If the `flight_descriptor` field of the [`PollInfo`](../operations/arrow_flight.gen.PollInfo.md#op-a5f3704ac23735382e4c2af6) is
`None` then the `info` field represents the complete results.

If the `flight_descriptor` field is some [`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326)
then the `info` field has incomplete results, and the client
should call this method again with the new `flight_descriptor`
to get the updated status.

The `expiration_time`, if set, represents the expiration time
of the `flight_descriptor`, after which the server may not accept
this retry descriptor and may cancel the query.

# Example:
```no_run
# async fn run() {
# use arrow_flight::FlightClient;
# use arrow_flight::FlightDescriptor;
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// Send a 'CMD' request to the server
let request = FlightDescriptor::new_cmd(b"MOAR DATA".to_vec());
let poll_info = client
  .poll_flight_info(request)
  .await
  .expect("error handshaking");

// retrieve the first endpoint from the returned poll info
let ticket = poll_info
  .info
  .expect("expected flight info")
  .endpoint[0]
  // Extract the ticket
  .ticket
  .clone()
  .expect("expected ticket");

// Retrieve the corresponding RecordBatch stream with do_get
let data = client
  .do_get(ticket)
  .await
  .expect("error fetching data");
# }
```

<a id="op-70867e9d73c446877c8c3495"></a>
## renew_flight_endpoint

`function` · `arrow_flight::client::FlightClient::renew_flight_endpoint` · arrow-flight 59.3.0

```rust
async fn renew_flight_endpoint(&mut self, request: RenewFlightEndpointRequest) -> Result<FlightEndpoint>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::client::FlightClient", "path": "FlightClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [79, 1], "end": [680, 2], "filename": "src/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/client.rs:661`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Make a `RenewFlightEndpoint` call to the server and return
the renewed [`FlightEndpoint`](../operations/arrow_flight.gen.FlightEndpoint.md#op-7be6a6f1cb907b9ae3f73a23).

# Example:
```no_run
# async fn run() {
# use arrow_flight::{FlightClient, FlightDescriptor, RenewFlightEndpointRequest};
# let channel: tonic::transport::Channel = unimplemented!();
let mut client = FlightClient::new(channel);

// Send a 'CMD' request to the server
let request = FlightDescriptor::new_cmd(b"MOAR DATA".to_vec());
let flight_endpoint = client
  .get_flight_info(request)
  .await
  .expect("error handshaking")
  .endpoint[0];

// Renew the endpoint
let request = RenewFlightEndpointRequest::new(flight_endpoint);
let flight_endpoint = client
  .renew_flight_endpoint(request)
  .await
  .expect("error renewing");
# }
```
