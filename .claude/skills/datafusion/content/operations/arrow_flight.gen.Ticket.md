# `arrow_flight::gen::Ticket`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.Ticket.json).

<a id="op-284d5d60124d450a179a6175"></a>
## Ticket

`struct` · `arrow_flight::gen::Ticket` · arrow-flight 59.3.0

```rust
struct Ticket
```

Source: `src/arrow.flight.protocol.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


An opaque identifier that the service can use to retrieve a particular
portion of a stream.

Tickets are meant to be single use. It is an error/application-defined
behavior to reuse a ticket.

<a id="op-eac839e31dc2a23aa61426d4"></a>
## clear

`function` · `arrow_flight::gen::Ticket::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 38], "end": [336, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59a44610a764ae90b9dc3130"></a>
## clone

`function` · `arrow_flight::gen::Ticket::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> Ticket
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 10], "end": [336, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a68f4e3623a4f344e9d49895"></a>
## default

`function` · `arrow_flight::gen::Ticket::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 38], "end": [336, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2fbdbcf8edf2ccc9e2787d1"></a>
## encoded_len

`function` · `arrow_flight::gen::Ticket::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 38], "end": [336, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55279ce5eb7610e3d70ee1d4"></a>
## eq

`function` · `arrow_flight::gen::Ticket::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &Ticket) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 17], "end": [336, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52a1bbfa7af7ce463cbf43f7"></a>
## fmt

`function` · `arrow_flight::gen::Ticket::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 38], "end": [336, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbd7ed2429b6897bb85973f7"></a>
## fmt

`function` · `arrow_flight::gen::Ticket::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 1], "end": [362, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ece8469cc0f2e01437cb5ba"></a>
## hash

`function` · `arrow_flight::gen::Ticket::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [336, 32], "end": [336, 36], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-657a9360ae1339a42efd15f6"></a>
## new

`function` · `arrow_flight::gen::Ticket::new` · arrow-flight 59.3.0

```rust
fn new(ticket: impl Into<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::Ticket", "path": "Ticket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [756, 1], "end": [770, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:765`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new `Ticket`

# Example

```
# use arrow_flight::Ticket;
let ticket = Ticket::new("SELECT * from FOO");
```

<a id="op-23604b3cc4c043b5b218eac7"></a>
## ticket

`struct_field` · `arrow_flight::gen::Ticket::ticket` · arrow-flight 59.3.0

```rust
ticket: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:339`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
