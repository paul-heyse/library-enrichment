# `arrow_flight::gen::RenewFlightEndpointRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.RenewFlightEndpointRequest.json).

<a id="op-7997e0667e8192b542de0785"></a>
## RenewFlightEndpointRequest

`struct` · `arrow_flight::gen::RenewFlightEndpointRequest` · arrow-flight 59.3.0

```rust
struct RenewFlightEndpointRequest
```

Source: `src/arrow.flight.protocol.rs:80`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The request of the RenewFlightEndpoint action.

The request should be stored in Action.body.

<a id="op-19b9ab9b9900441cfc5a1306"></a>
## clear

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 28], "end": [79, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea6937a64d104027fea1aac6"></a>
## clone

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> RenewFlightEndpointRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 10], "end": [79, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34282001554b10471d1e79bb"></a>
## default

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 28], "end": [79, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-037235e6f1cf387516b089fe"></a>
## encoded_len

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 28], "end": [79, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd610d2beec371c0d5c6e65f"></a>
## endpoint

`struct_field` · `arrow_flight::gen::RenewFlightEndpointRequest::endpoint` · arrow-flight 59.3.0

```rust
endpoint: ::core::option::Option<FlightEndpoint>
```

Source: `src/arrow.flight.protocol.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be1453f068972c694abcde16"></a>
## eq

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &RenewFlightEndpointRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 17], "end": [79, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fc796e53a53ebc01fa75e94"></a>
## fmt

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 1], "end": [346, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9b67f4620d259767d53fe0f"></a>
## fmt

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 28], "end": [79, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-324791cdb00bcb13bf85ec54"></a>
## new

`function` · `arrow_flight::gen::RenewFlightEndpointRequest::new` · arrow-flight 59.3.0

```rust
fn new(endpoint: FlightEndpoint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::RenewFlightEndpointRequest", "path": "RenewFlightEndpointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [729, 1], "end": [737, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:732`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new [`RenewFlightEndpointRequest`](../operations/arrow_flight.gen.RenewFlightEndpointRequest.md#op-7997e0667e8192b542de0785), providing the [`FlightEndpoint`](../operations/arrow_flight.gen.FlightEndpoint.md#op-7be6a6f1cb907b9ae3f73a23)
for which is being requested an extension of its expiration.
