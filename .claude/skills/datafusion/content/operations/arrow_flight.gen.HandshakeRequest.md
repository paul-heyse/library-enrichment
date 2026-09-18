# `arrow_flight::gen::HandshakeRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.HandshakeRequest.json).

<a id="op-2a6d2ff8443d88db6fc9cdff"></a>
## HandshakeRequest

`struct` · `arrow_flight::gen::HandshakeRequest` · arrow-flight 59.3.0

```rust
struct HandshakeRequest
```

Source: `src/arrow.flight.protocol.rs:7`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The request that a client provides to a server on handshake.

<a id="op-f46f937d9f4aa871205c4083"></a>
## clear

`function` · `arrow_flight::gen::HandshakeRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::HandshakeRequest", "path": "HandshakeRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 38], "end": [6, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:6`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00b723d4934c3da96709f247"></a>
## clone

`function` · `arrow_flight::gen::HandshakeRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> HandshakeRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::HandshakeRequest", "path": "HandshakeRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 10], "end": [6, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:6`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05df43ee984c9a1e1c81c910"></a>
## default

`function` · `arrow_flight::gen::HandshakeRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::HandshakeRequest", "path": "HandshakeRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 38], "end": [6, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:6`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a09853ed5c118b7ba22a043c"></a>
## encoded_len

`function` · `arrow_flight::gen::HandshakeRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::HandshakeRequest", "path": "HandshakeRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 38], "end": [6, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:6`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3550fe938af9c6b1bd653b7d"></a>
## eq

`function` · `arrow_flight::gen::HandshakeRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &HandshakeRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::HandshakeRequest", "path": "HandshakeRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 17], "end": [6, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:6`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc710fd056bcc812436b0632"></a>
## fmt

`function` · `arrow_flight::gen::HandshakeRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::HandshakeRequest", "path": "HandshakeRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 38], "end": [6, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:6`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b30148dacf0770d122c3c511"></a>
## hash

`function` · `arrow_flight::gen::HandshakeRequest::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::HandshakeRequest", "path": "HandshakeRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6, 32], "end": [6, 36], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:6`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5935183b233eb004fe57605"></a>
## payload

`struct_field` · `arrow_flight::gen::HandshakeRequest::payload` · arrow-flight 59.3.0

```rust
payload: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:15`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Arbitrary auth/handshake info.

<a id="op-3ab28d8b6527cfe9b844d742"></a>
## protocol_version

`struct_field` · `arrow_flight::gen::HandshakeRequest::protocol_version` · arrow-flight 59.3.0

```rust
protocol_version: u64
```

Source: `src/arrow.flight.protocol.rs:11`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A defined protocol version
