# `arrow_flight::gen::FlightEndpoint`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.FlightEndpoint.json).

<a id="op-7be6a6f1cb907b9ae3f73a23"></a>
## FlightEndpoint

`struct` · `arrow_flight::gen::FlightEndpoint` · arrow-flight 59.3.0

```rust
struct FlightEndpoint
```

Source: `src/arrow.flight.protocol.rs:283`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A particular stream or split associated with a flight.

<a id="op-3795e3e37af5984eaa12a3f4"></a>
## app_metadata

`struct_field` · `arrow_flight::gen::FlightEndpoint::app_metadata` · arrow-flight 59.3.0

```rust
app_metadata: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Application-defined metadata.

There is no inherent or required relationship between this
and the app_metadata fields in the FlightInfo or resulting
FlightData messages. Since this metadata is application-defined,
a given application could define there to be a relationship,
but there is none required by the spec.

<a id="op-0121859e73abf154aff597f1"></a>
## clear

`function` · `arrow_flight::gen::FlightEndpoint::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 28], "end": [282, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-232fe1f9226dcd0912fda658"></a>
## clone

`function` · `arrow_flight::gen::FlightEndpoint::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> FlightEndpoint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 10], "end": [282, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf5654a55506eb5439145699"></a>
## default

`function` · `arrow_flight::gen::FlightEndpoint::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 28], "end": [282, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24af43de794dd013707a7e32"></a>
## encoded_len

`function` · `arrow_flight::gen::FlightEndpoint::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 28], "end": [282, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7485076ee6065ad7a92ecb7"></a>
## eq

`function` · `arrow_flight::gen::FlightEndpoint::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &FlightEndpoint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 17], "end": [282, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60ff5fe36d40af7fbabba0e4"></a>
## expiration_time

`struct_field` · `arrow_flight::gen::FlightEndpoint::expiration_time` · arrow-flight 59.3.0

```rust
expiration_time: ::core::option::Option<::prost_types::Timestamp>
```

Source: `src/arrow.flight.protocol.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Expiration time of this stream. If present, clients may assume
they can retry DoGet requests. Otherwise, it is
application-defined whether DoGet requests may be retried.

<a id="op-07ba451ef8e4de57002a11aa"></a>
## fmt

`function` · `arrow_flight::gen::FlightEndpoint::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 28], "end": [282, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23dd7445e0ad3f10ed40a1e6"></a>
## fmt

`function` · `arrow_flight::gen::FlightEndpoint::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [261, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:238`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f540744d963e635a828a6da2"></a>
## location

`struct_field` · `arrow_flight::gen::FlightEndpoint::location` · arrow-flight 59.3.0

```rust
location: ::prost::alloc::vec::Vec<Location>
```

Source: `src/arrow.flight.protocol.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A list of URIs where this ticket can be redeemed via DoGet().

If the list is empty, the expectation is that the ticket can only
be redeemed on the current service where the ticket was
generated.

If the list is not empty, the expectation is that the ticket can
be redeemed at any of the locations, and that the data returned
will be equivalent. In this case, the ticket may only be redeemed
at one of the given locations, and not (necessarily) on the
current service.

In other words, an application can use multiple locations to
represent redundant and/or load balanced services.

<a id="op-c9aae0f615ca51210ff62ebf"></a>
## new

`function` · `arrow_flight::gen::FlightEndpoint::new` · arrow-flight 59.3.0

```rust
fn new() -> FlightEndpoint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [828, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:791`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new, empty `FlightEndpoint` that represents a location
to retrieve Flight results.

# Example
```
# use arrow_flight::{FlightEndpoint, Ticket};
#
// Specify the client should fetch results from this server
let endpoint = FlightEndpoint::new()
  .with_ticket(Ticket::new("the ticket"));

// Specify the client should fetch results from either
// `http://example.com` or `https://example.com`
let endpoint = FlightEndpoint::new()
  .with_ticket(Ticket::new("the ticket"))
  .with_location("http://example.com")
  .with_location("https://example.com");
```

<a id="op-3ef1662bb2dfd4ae52939ca7"></a>
## ticket

`struct_field` · `arrow_flight::gen::FlightEndpoint::ticket` · arrow-flight 59.3.0

```rust
ticket: ::core::option::Option<Ticket>
```

Source: `src/arrow.flight.protocol.rs:287`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Token used to retrieve this stream.

<a id="op-30c066ab3d28232c31f2b06e"></a>
## with_app_metadata

`function` · `arrow_flight::gen::FlightEndpoint::with_app_metadata` · arrow-flight 59.3.0

```rust
fn with_app_metadata(self, app_metadata: impl Into<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [828, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:824`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add optional application specific metadata to the message

<a id="op-23afb9bfcfedb8d54feba4a8"></a>
## with_expiration_time

`function` · `arrow_flight::gen::FlightEndpoint::with_expiration_time` · arrow-flight 59.3.0

```rust
fn with_expiration_time(self, expiration_time: Timestamp) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [828, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:818`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specify expiration time for this stream

<a id="op-b53751d3593d9cdeb0fbf2a0"></a>
## with_location

`function` · `arrow_flight::gen::FlightEndpoint::with_location` · arrow-flight 59.3.0

```rust
fn with_location(self, uri: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [828, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:812`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add a location `uri` to this endpoint. Note each endpoint can
have multiple locations.

If no `uri` is specified, the [Flight Spec] says:

```text
* If the list is empty, the expectation is that the ticket can only
* be redeemed on the current service where the ticket was
* generated.
```
[Flight Spec]: https://github.com/apache/arrow-rs/blob/17ca4d51d0490f9c65f5adde144f677dbc8300e7/format/Flight.proto#L307C2-L312

<a id="op-19fcf345135287cc4175f847"></a>
## with_ticket

`function` · `arrow_flight::gen::FlightEndpoint::with_ticket` · arrow-flight 59.3.0

```rust
fn with_ticket(self, ticket: Ticket) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightEndpoint", "path": "FlightEndpoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [772, 1], "end": [828, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set the [`Ticket`](../operations/arrow_flight.gen.Ticket.md#op-284d5d60124d450a179a6175) used to retrieve data from the endpoint
