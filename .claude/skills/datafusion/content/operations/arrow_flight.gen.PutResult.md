# `arrow_flight::gen::PutResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.PutResult.json).

<a id="op-b90b812a48ac8a0f1c069fdf"></a>
## PutResult

`struct` · `arrow_flight::gen::PutResult` · arrow-flight 59.3.0

```rust
struct PutResult
```

Source: `src/arrow.flight.protocol.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
The response message associated with the submission of a DoPut.

<a id="op-b8f0539cf5e752660c760fad"></a>
## app_metadata

`struct_field` · `arrow_flight::gen::PutResult::app_metadata` · arrow-flight 59.3.0

```rust
app_metadata: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:371`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c58546c5ef1f37abf842a5b0"></a>
## clear

`function` · `arrow_flight::gen::PutResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 38], "end": [368, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93373dbc8587cd36f87f5e2c"></a>
## clone

`function` · `arrow_flight::gen::PutResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> PutResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 10], "end": [368, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09e436acec672ffd6fe25916"></a>
## default

`function` · `arrow_flight::gen::PutResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 38], "end": [368, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32232135b07ae4b64cbf7160"></a>
## encoded_len

`function` · `arrow_flight::gen::PutResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 38], "end": [368, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc2ac38fa926772b713114d3"></a>
## eq

`function` · `arrow_flight::gen::PutResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &PutResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 17], "end": [368, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c84de40a028ecddde0d278d"></a>
## fmt

`function` · `arrow_flight::gen::PutResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 38], "end": [368, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0c281dc6b09a4cb81dfc0b7"></a>
## hash

`function` · `arrow_flight::gen::PutResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 32], "end": [368, 36], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
