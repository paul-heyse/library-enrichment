# `arrow_flight::gen::flight_service_server::FlightServiceServer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.flight_service_server.FlightServiceServer.json).

<a id="op-48b1bbbebd2a9708098b9006"></a>
## FlightServiceServer

`struct` · `arrow_flight::gen::flight_service_server::FlightServiceServer` · arrow-flight 59.3.0

```rust
struct FlightServiceServer<T>
```

Source: `src/arrow.flight.protocol.rs:1026`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A flight service is an endpoint for retrieving or storing Arrow data. A
flight service can expose one or more predefined endpoints that can be
accessed using the Arrow Flight Protocol. Additionally, a flight service
can expose a set of actions that are available.

<a id="op-03651cf73807679b2ac98ada"></a>
## Error

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightServiceServer::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_flight::gen::flight_service_server::FlightService", "path": "FlightService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "B"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "B"}, "trait": {"args": null, "id": "http_body::Body", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [1084, 5], "end": [1582, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "http::request::Request", "path": "Request"}}}], "constraints": []}}, "id": "tower_service::Service", "path": "Service"}, "trait_path": "tower_service::Service"}`

Source: `src/arrow.flight.protocol.rs:1091`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-620afbdaa51702f86324d51e"></a>
## Future

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightServiceServer::Future` · arrow-flight 59.3.0

```rust
Future
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_flight::gen::flight_service_server::FlightService", "path": "FlightService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "B"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "B"}, "trait": {"args": null, "id": "http_body::Body", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [1084, 5], "end": [1582, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "http::request::Request", "path": "Request"}}}], "constraints": []}}, "id": "tower_service::Service", "path": "Service"}, "trait_path": "tower_service::Service"}`

Source: `src/arrow.flight.protocol.rs:1092`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df6bdd5284dc02b4230b72de"></a>
## NAME

`assoc_const` · `arrow_flight::gen::flight_service_server::FlightServiceServer::NAME` · arrow-flight 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1597, 5], "end": [1599, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "tonic::server::NamedService", "path": "NamedService"}, "trait_path": "tonic::server::NamedService"}`

Source: `src/arrow.flight.protocol.rs:1598`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a12b9ba25258d4515160a258"></a>
## Response

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightServiceServer::Response` · arrow-flight 59.3.0

```rust
Response
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_flight::gen::flight_service_server::FlightService", "path": "FlightService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "B"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "B"}, "trait": {"args": null, "id": "http_body::Body", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [1084, 5], "end": [1582, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "http::request::Request", "path": "Request"}}}], "constraints": []}}, "id": "tower_service::Service", "path": "Service"}, "trait_path": "tower_service::Service"}`

Source: `src/arrow.flight.protocol.rs:1090`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3ba9e7840c75eff8930b42d"></a>
## accept_compressed

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::accept_compressed` · arrow-flight 59.3.0

```rust
fn accept_compressed(self, encoding: CompressionEncoding) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1033, 5], "end": [1083, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:1057`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Enable decompressing requests with the given encoding.

<a id="op-3ac24fe63ffdb76cfd2b453f"></a>
## call

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::call` · arrow-flight 59.3.0

```rust
fn call(&mut self, req: http::Request<B>) -> Self::Future
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_flight::gen::flight_service_server::FlightService", "path": "FlightService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "B"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "B"}, "trait": {"args": null, "id": "http_body::Body", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [1084, 5], "end": [1582, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "http::request::Request", "path": "Request"}}}], "constraints": []}}, "id": "tower_service::Service", "path": "Service"}, "trait_path": "tower_service::Service"}`

Source: `src/arrow.flight.protocol.rs:1099`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c93084627ea36f4473097a9"></a>
## clone

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1583, 5], "end": [1594, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:1584`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-474f835f9d941da9ddb47c12"></a>
## fmt

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1025, 14], "end": [1025, 19], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:1025`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70c283728aba09a93d671d6b"></a>
## from_arc

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::from_arc` · arrow-flight 59.3.0

```rust
fn from_arc(inner: Arc<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1033, 5], "end": [1083, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:1037`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a84b73486aa6737aeefbbdb"></a>
## max_decoding_message_size

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::max_decoding_message_size` · arrow-flight 59.3.0

```rust
fn max_decoding_message_size(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1033, 5], "end": [1083, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:1071`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Limits the maximum size of a decoded message.

Default: `4MB`

<a id="op-7a7863c628debd16c28364a5"></a>
## max_encoding_message_size

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::max_encoding_message_size` · arrow-flight 59.3.0

```rust
fn max_encoding_message_size(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1033, 5], "end": [1083, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:1079`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Limits the maximum size of an encoded message.

Default: `usize::MAX`

<a id="op-93310cd5e22d31c547dd4d22"></a>
## new

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::new` · arrow-flight 59.3.0

```rust
fn new(inner: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1033, 5], "end": [1083, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:1034`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36d9d80a1e50b339cdd5e634"></a>
## poll_ready

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::poll_ready` · arrow-flight 59.3.0

```rust
fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_flight::gen::flight_service_server::FlightService", "path": "FlightService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "B"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "std::marker::Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "B"}, "trait": {"args": null, "id": "http_body::Body", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [1084, 5], "end": [1582, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "http::request::Request", "path": "Request"}}}], "constraints": []}}, "id": "tower_service::Service", "path": "Service"}, "trait_path": "tower_service::Service"}`

Source: `src/arrow.flight.protocol.rs:1093`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ea2b06f028c08ab46c69ca2"></a>
## send_compressed

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::send_compressed` · arrow-flight 59.3.0

```rust
fn send_compressed(self, encoding: CompressionEncoding) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1033, 5], "end": [1083, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:1063`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Compress responses with the given encoding, if the client supports it.

<a id="op-3cf6543a72fd33b3fc165310"></a>
## with_interceptor

`function` · `arrow_flight::gen::flight_service_server::FlightServiceServer::with_interceptor` · arrow-flight 59.3.0

```rust
fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F> where F: tonic::service::Interceptor
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::gen::flight_service_server::FlightServiceServer", "path": "FlightServiceServer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1033, 5], "end": [1083, 6], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:1046`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
