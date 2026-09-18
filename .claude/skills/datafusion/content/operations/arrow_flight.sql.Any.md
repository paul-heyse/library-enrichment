# `arrow_flight::sql::Any`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.Any.json).

<a id="op-a83fa9be583ec3b019e51a4e"></a>
## Any

`struct` · `arrow_flight::sql::Any` · arrow-flight 59.3.0

```rust
struct Any
```

Source: `src/sql/mod.rs:271`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

An implementation of the protobuf [`Any`] message type

Encoded protobuf messages are not self-describing, nor contain any information
on the schema of the encoded payload. Consequently to decode a protobuf a client
must know the exact schema of the message.

This presents a problem for loosely typed APIs, where the exact message payloads
are not enumerable, and therefore cannot be enumerated as variants in a [oneof].

One solution is [`Any`] where the encoded payload is paired with a `type_url`
identifying the type of encoded message, and the resulting combination encoded.

Clients can then decode the outer [`Any`], inspect the `type_url` and if it is
a type they recognise, proceed to decode the embedded message `value`

[`Any`]: https://developers.google.com/protocol-buffers/docs/proto3#any
[oneof]: https://developers.google.com/protocol-buffers/docs/proto3#oneof

<a id="op-54679f3935e2c6178ecabac9"></a>
## clear

`function` · `arrow_flight::sql::Any::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 28], "end": [270, 44], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f021c9b8a545195dee19476"></a>
## clone

`function` · `arrow_flight::sql::Any::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 10], "end": [270, 15], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cd22c5ad12f4d0305e548b6"></a>
## default

`function` · `arrow_flight::sql::Any::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 28], "end": [270, 44], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-213187c309189987319969dd"></a>
## encoded_len

`function` · `arrow_flight::sql::Any::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 28], "end": [270, 44], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f8847b7a741a7883521250"></a>
## eq

`function` · `arrow_flight::sql::Any::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &Any) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 17], "end": [270, 26], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a88851d4c5cd1025346a84aa"></a>
## fmt

`function` · `arrow_flight::sql::Any::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 28], "end": [270, 44], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e9e52560f4a4c512d0b5116"></a>
## is

`function` · `arrow_flight::sql::Any::is` · arrow-flight 59.3.0

```rust
fn is<M: ProstMessageExt>(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [305, 2], "filename": "src/sql/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/mod.rs:287`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Checks whether the message is of type `M`

<a id="op-3e6f1b67cc183fa9d4e8267c"></a>
## pack

`function` · `arrow_flight::sql::Any::pack` · arrow-flight 59.3.0

```rust
fn pack<M: ProstMessageExt>(message: &M) -> Result<Any, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [305, 2], "filename": "src/sql/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/mod.rs:302`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Packs a message into an [`Any`](../operations/arrow_flight.sql.Any.md#op-a83fa9be583ec3b019e51a4e) message

<a id="op-6f296a3a0b25e26cf30d9512"></a>
## type_url

`struct_field` · `arrow_flight::sql::Any::type_url` · arrow-flight 59.3.0

```rust
type_url: String
```

Source: `src/sql/mod.rs:279`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A URL/resource name that uniquely identifies the type of the serialized
protocol buffer message. This string must contain at least
one "/" character. The last segment of the URL's path must represent
the fully qualified name of the type (as in
`path/google.protobuf.Duration`). The name should be in a canonical form
(e.g., leading "." is not accepted).

<a id="op-21ef0381b012ae157174fcb3"></a>
## unpack

`function` · `arrow_flight::sql::Any::unpack` · arrow-flight 59.3.0

```rust
fn unpack<M: ProstMessageExt>(&self) -> Result<Option<M>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [305, 2], "filename": "src/sql/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/mod.rs:292`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Unpacks the contents of the message if it is of type `M`

<a id="op-56d0c3a77dc9cfe9de1474c3"></a>
## value

`struct_field` · `arrow_flight::sql::Any::value` · arrow-flight 59.3.0

```rust
value: bytes::Bytes
```

Source: `src/sql/mod.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Must be a valid serialized protocol buffer of the above specified type.
