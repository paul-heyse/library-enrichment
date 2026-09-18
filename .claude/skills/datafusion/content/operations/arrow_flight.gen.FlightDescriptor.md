# `arrow_flight::gen::FlightDescriptor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.FlightDescriptor.json).

<a id="op-87317d059af665e5e2fe8326"></a>
## FlightDescriptor

`struct` · `arrow_flight::gen::FlightDescriptor` · arrow-flight 59.3.0

```rust
struct FlightDescriptor
```

Source: `src/arrow.flight.protocol.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The name or tag for a Flight. May be used as a way to retrieve or generate
a flight or be used to expose a set of previously defined flights.

<a id="op-ec8dc18baca5c5dbc0389365"></a>
## clear

`function` · `arrow_flight::gen::FlightDescriptor::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 38], "end": [114, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2caa4818ede0e65b570d668a"></a>
## clone

`function` · `arrow_flight::gen::FlightDescriptor::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> FlightDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 10], "end": [114, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d9b20f1e98dcea9b69d0fd4"></a>
## cmd

`struct_field` · `arrow_flight::gen::FlightDescriptor::cmd` · arrow-flight 59.3.0

```rust
cmd: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Opaque value used to express a command. Should only be defined when
type = CMD.

<a id="op-6cd0d66e42f5800f9607f261"></a>
## default

`function` · `arrow_flight::gen::FlightDescriptor::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 38], "end": [114, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83b2a94d406c71fbbedd7e08"></a>
## encoded_len

`function` · `arrow_flight::gen::FlightDescriptor::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 38], "end": [114, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15f41c552961a1508eb455d3"></a>
## eq

`function` · `arrow_flight::gen::FlightDescriptor::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &FlightDescriptor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 17], "end": [114, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-204639bf7cd75b9affdb8c1d"></a>
## fmt

`function` · `arrow_flight::gen::FlightDescriptor::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [235, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:212`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cb1725560e91bcf78a2ead7"></a>
## fmt

`function` · `arrow_flight::gen::FlightDescriptor::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 38], "end": [114, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e559319b55e451e24f58895"></a>
## hash

`function` · `arrow_flight::gen::FlightDescriptor::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 32], "end": [114, 36], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f9eba3b8c08fe0b2a86502c"></a>
## new_cmd

`function` · `arrow_flight::gen::FlightDescriptor::new_cmd` · arrow-flight 59.3.0

```rust
fn new_cmd(cmd: impl Into<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [538, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:520`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new opaque command [`CMD`] `FlightDescriptor` to generate a dataset.

[`CMD`]: https://github.com/apache/arrow/blob/6bd31f37ae66bd35594b077cb2f830be57e08acd/format/Flight.proto#L224-L227

<a id="op-da3f8b11b90cdff33d3aa037"></a>
## new_path

`function` · `arrow_flight::gen::FlightDescriptor::new_path` · arrow-flight 59.3.0

```rust
fn new_path(path: Vec<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [538, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:531`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new named path [`PATH`] `FlightDescriptor` that identifies a dataset

[`PATH`]: https://github.com/apache/arrow/blob/6bd31f37ae66bd35594b077cb2f830be57e08acd/format/Flight.proto#L217-L222

<a id="op-522f4a369bec89e74ec72754"></a>
## path

`struct_field` · `arrow_flight::gen::FlightDescriptor::path` · arrow-flight 59.3.0

```rust
path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>
```

Source: `src/arrow.flight.protocol.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


List of strings identifying a particular dataset. Should only be defined
when type = PATH.

<a id="op-d8e7723582e9b1b0308457ba"></a>
## set_type

`function` · `arrow_flight::gen::FlightDescriptor::set_type` · arrow-flight 59.3.0

```rust
fn set_type(&mut self, value: flight_descriptor::DescriptorType)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 38], "end": [114, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Sets `type` to the provided enum value.

<a id="op-5cf67c17588e2dead37d96c4"></a>
## type

`struct_field` · `arrow_flight::gen::FlightDescriptor::type` · arrow-flight 59.3.0

```rust
type: i32
```

Source: `src/arrow.flight.protocol.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2ea8a998cc9ca3932b95026"></a>
## type

`function` · `arrow_flight::gen::FlightDescriptor::type` · arrow-flight 59.3.0

```rust
fn type(&self) -> flight_descriptor::DescriptorType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightDescriptor", "path": "FlightDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 38], "end": [114, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the enum value of `type`, or the default if the field is set to an invalid enum value.
