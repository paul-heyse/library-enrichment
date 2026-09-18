# `arrow_flight::gen::flight_service_client::FlightServiceClient`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.flight_service_client.FlightServiceClient.json).

<a id="op-8e3fc5d811e76f6be8a947c7"></a>
## FlightServiceClient

`struct` · `arrow_flight::gen::flight_service_client::FlightServiceClient` · arrow-flight 59.3.0

```rust
struct FlightServiceClient<T>
```

Source: `src/arrow.flight.protocol.rs:435`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A flight service is an endpoint for retrieving or storing Arrow data. A
flight service can expose one or more predefined endpoints that can be
accessed using the Arrow Flight Protocol. Additionally, a flight service
can expose a set of actions that are available.

<a id="op-1edfcb647f0222ce044b2c7d"></a>
## accept_compressed

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::accept_compressed` · arrow-flight 59.3.0

```rust
fn accept_compressed(self, encoding: CompressionEncoding) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:483`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Enable decompressing responses.

<a id="op-8c3782cc9f2d3b695a09ad53"></a>
## clone

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> FlightServiceClient<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 21], "end": [434, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc255326c5457657d99bab38"></a>
## do_action

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::do_action` · arrow-flight 59.3.0

```rust
async fn do_action(&mut self, request: impl tonic::IntoRequest<super::Action>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::Result>>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:780`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Flight services can support an arbitrary number of simple actions in
addition to the possible ListFlights, GetFlightInfo, DoGet, DoPut
operations that are potentially available. DoAction allows a flight client
to do a specific action against a flight service. An action includes
opaque request and response objects that are specific to the type action
being undertaken.

<a id="op-e56377e6718320fc1d49aff0"></a>
## do_exchange

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::do_exchange` · arrow-flight 59.3.0

```rust
async fn do_exchange(&mut self, request: impl tonic::IntoStreamingRequest<Message = super::FlightData>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::FlightData>>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:747`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Open a bidirectional data channel for a given descriptor. This
allows clients to send and receive arbitrary Arrow data and
application-specific metadata in a single logical stream. In
contrast to DoGet/DoPut, this is more suited for clients
offloading computation (rather than storage) to a Flight service.

<a id="op-f765ef5cce7fe4297b123f51"></a>
## do_get

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::do_get` · arrow-flight 59.3.0

```rust
async fn do_get(&mut self, request: impl tonic::IntoRequest<super::Ticket>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::FlightData>>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:686`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieve a single stream associated with a particular descriptor
associated with the referenced ticket. A Flight can be composed of one or
more streams where each stream can be retrieved using a separate opaque
ticket that the flight service uses for managing a collection of streams.

<a id="op-87b33e629eaa1b482c4966fe"></a>
## do_put

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::do_put` · arrow-flight 59.3.0

```rust
async fn do_put(&mut self, request: impl tonic::IntoStreamingRequest<Message = super::FlightData>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::PutResult>>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:717`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Push a stream to the flight service associated with a particular
flight stream. This allows a client of a flight service to upload a stream
of data. Depending on the particular flight service, a client consumer
could be allowed to upload a single stream per descriptor or an unlimited
number. In the latter, the service might implement a 'seal' action that
can be applied to a descriptor once all streams are uploaded.

<a id="op-ec61504eef38fd381925ef5b"></a>
## fmt

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 14], "end": [434, 19], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ae5a14fca2482c21d336842"></a>
## get_flight_info

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::get_flight_info` · arrow-flight 59.3.0

```rust
async fn get_flight_info(&mut self, request: impl tonic::IntoRequest<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::FlightInfo>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:578`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


For a given FlightDescriptor, get information about how the flight can be
consumed. This is a useful interface if the consumer of the interface
already can identify the specific flight to consume. This interface can
also allow a consumer to generate a flight stream through a specified
descriptor. For example, a flight descriptor might be something that
includes a SQL statement or a Pickled Python operation that will be
executed. In those cases, the descriptor will not be previously available
within the list of available streams provided by ListFlights but will be
available for consumption for the duration defined by the specific flight
service.

<a id="op-9edd6af407797ea49886390e"></a>
## get_schema

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::get_schema` · arrow-flight 59.3.0

```rust
async fn get_schema(&mut self, request: impl tonic::IntoRequest<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::SchemaResult>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:658`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


For a given FlightDescriptor, get the Schema as described in Schema.fbs::Schema
This is used when a consumer needs the Schema of flight stream. Similar to
GetFlightInfo this interface may generate a new flight that was not previously
available in ListFlights.

<a id="op-9b0d2dc8d815724df65e7a49"></a>
## handshake

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::handshake` · arrow-flight 59.3.0

```rust
async fn handshake(&mut self, request: impl tonic::IntoStreamingRequest<Message = super::HandshakeRequest>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::HandshakeResponse>>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:508`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Handshake between client and server. Depending on the server, the
handshake may be required to determine the token that should be used for
future operations. Both request and response are streams to allow multiple
round-trips depending on auth mechanism.

<a id="op-9e783a9fa3ac1b8ff25aad89"></a>
## list_actions

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::list_actions` · arrow-flight 59.3.0

```rust
async fn list_actions(&mut self, request: impl tonic::IntoRequest<super::Empty>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::ActionType>>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:810`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A flight service exposes all of the available action types that it has
along with descriptions. This allows different flight consumers to
understand the capabilities of the flight service.

<a id="op-1f12b9265983a90e34c51b3d"></a>
## list_flights

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::list_flights` · arrow-flight 59.3.0

```rust
async fn list_flights(&mut self, request: impl tonic::IntoRequest<super::Criteria>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::FlightInfo>>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:541`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Get a list of available streams given a particular criteria. Most flight
services will expose one or more streams that are readily available for
retrieval. This api allows listing the streams available for
consumption. A user can also provide a criteria. The criteria can limit
the subset of streams that can be listed via this interface. Each flight
service allows its own definition of how to consume criteria.

<a id="op-d66df50af14b09a30928f29e"></a>
## max_decoding_message_size

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::max_decoding_message_size` · arrow-flight 59.3.0

```rust
fn max_decoding_message_size(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:491`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Limits the maximum size of a decoded message.

Default: `4MB`

<a id="op-d98f03a1478339ccd8c4d66c"></a>
## max_encoding_message_size

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::max_encoding_message_size` · arrow-flight 59.3.0

```rust
fn max_encoding_message_size(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:499`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Limits the maximum size of an encoded message.

Default: `usize::MAX`

<a id="op-8a8e9da3915720517e0a29a6"></a>
## new

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::new` · arrow-flight 59.3.0

```rust
fn new(inner: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:445`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-594f9d8a34d7919417e26739"></a>
## poll_flight_info

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::poll_flight_info` · arrow-flight 59.3.0

```rust
async fn poll_flight_info(&mut self, request: impl tonic::IntoRequest<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::PollInfo>, tonic::Status>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:627`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


For a given FlightDescriptor, start a query and get information
to poll its execution status. This is a useful interface if the
query may be a long-running query. The first PollFlightInfo call
should return as quickly as possible. (GetFlightInfo doesn't
return until the query is complete.)

A client can consume any available results before
the query is completed. See PollInfo.info for details.

A client can poll the updated query status by calling
PollFlightInfo() with PollInfo.flight_descriptor. A server
should not respond until the result would be different from last
time. That way, the client can "long poll" for updates
without constantly making requests. Clients can set a short timeout
to avoid blocking calls if desired.

A client can't use PollInfo.flight_descriptor after
PollInfo.expiration_time passes. A server might not accept the
retry descriptor anymore and the query may be cancelled.

A client may use the CancelFlightInfo action with
PollInfo.info to cancel the running query.

<a id="op-417e66d9b65a883f1dc93a28"></a>
## send_compressed

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::send_compressed` · arrow-flight 59.3.0

```rust
fn send_compressed(self, encoding: CompressionEncoding) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:477`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Compress requests with the given encoding.

This requires the server to support it otherwise it might respond with an
error.

<a id="op-969a5e6670f7b86ccde70ab0"></a>
## with_interceptor

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::with_interceptor` · arrow-flight 59.3.0

```rust
fn with_interceptor<F>(inner: T, interceptor: F) -> FlightServiceClient<InterceptedService<T, F>> where F: tonic::service::Interceptor, T::ResponseBody: Default, T: tonic::codegen::Service<http::Request<tonic::body::Body>, Response = http::Response<<T as tonic::client::GrpcService>::ResponseBody>>, <T as tonic::codegen::Service>::Error: Into<StdError> + std::marker::Send + std::marker::Sync
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:453`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bec414ef634606e5f0a0438"></a>
## with_origin

`function` · `arrow_flight::gen::flight_service_client::FlightServiceClient::with_origin` · arrow-flight 59.3.0

```rust
fn with_origin(inner: T, origin: Uri) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_client::FlightServiceClient", "path": "FlightServiceClient"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [438, 5], "end": [836, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
