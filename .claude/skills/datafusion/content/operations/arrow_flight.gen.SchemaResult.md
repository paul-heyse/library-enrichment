# `arrow_flight::gen::SchemaResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.SchemaResult.json).

<a id="op-77e2df8ada2705eab4f9041e"></a>
## SchemaResult

`struct` · `arrow_flight::gen::SchemaResult` · arrow-flight 59.3.0

```rust
struct SchemaResult
```

Source: `src/arrow.flight.protocol.rs:103`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Wrap the result of a getSchema call

<a id="op-f012c45d590d0602088821f2"></a>
## Error

`assoc_type` · `arrow_flight::gen::SchemaResult::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 1], "end": [398, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/lib.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1187d0440c8c68dce60b7d83"></a>
## clear

`function` · `arrow_flight::gen::SchemaResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 38], "end": [102, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56d5727e2456a7e01dfa3e60"></a>
## clone

`function` · `arrow_flight::gen::SchemaResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SchemaResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 10], "end": [102, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dd883705b2b92f36eed04f1"></a>
## default

`function` · `arrow_flight::gen::SchemaResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 38], "end": [102, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8f2267668c48b236a6f5818"></a>
## encoded_len

`function` · `arrow_flight::gen::SchemaResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 38], "end": [102, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77d9e07fac09703b71e08ec3"></a>
## eq

`function` · `arrow_flight::gen::SchemaResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SchemaResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 17], "end": [102, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19846a63beef7293826de5c4"></a>
## fmt

`function` · `arrow_flight::gen::SchemaResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 38], "end": [102, 54], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ffecc72c0df9fd10fdc29cd"></a>
## hash

`function` · `arrow_flight::gen::SchemaResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 32], "end": [102, 36], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/arrow.flight.protocol.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb283c13a904fd0fdfeef52f"></a>
## schema

`struct_field` · `arrow_flight::gen::SchemaResult::schema` · arrow-flight 59.3.0

```rust
schema: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The schema of the dataset in its IPC form:
   4 bytes - an optional IPC_CONTINUATION_TOKEN prefix
   4 bytes - the byte length of the payload
   a flatbuffer Message whose header is the Schema

<a id="op-1e881e6e56de0c0958a9b0fe"></a>
## try_from

`function` · `arrow_flight::gen::SchemaResult::try_from` · arrow-flight 59.3.0

```rust
fn try_from(schema_ipc: SchemaAsIpc<'_>) -> std::result::Result<Self, arrow_schema::ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::SchemaResult", "path": "SchemaResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 1], "end": [398, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/lib.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
