# `arrow_flight::sql::gen::ActionCancelQueryRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionCancelQueryRequest.json).

<a id="op-dd952a47af4299200b581108"></a>
## ActionCancelQueryRequest

`struct` · `arrow_flight::sql::gen::ActionCancelQueryRequest` · arrow-flight 59.3.0

```rust
struct ActionCancelQueryRequest
```

Source: `src/sql/arrow.flight.protocol.sql.rs:963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Request message for the "CancelQuery" action.

Explicitly cancel a running query.

This lets a single client explicitly cancel work, no matter how many clients
are involved/whether the query is distributed or not, given server support.
The transaction/statement is not rolled back; it is the application's job to
commit or rollback as appropriate. This only indicates the client no longer
wishes to read the remainder of the query results or continue submitting
data.

This command is idempotent.

This command is deprecated since 13.0.0. Use the "CancelFlightInfo"
action with DoAction instead.

<a id="op-8439042780ecc77c2915c3a5"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a9a58071a79f6f049900519"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 38], "end": [962, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54556e5b64ad65433cdc3667"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionCancelQueryRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 10], "end": [962, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-269d01611f0b88a936923b1d"></a>
## default

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 38], "end": [962, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d579f5c6a168d3b75021d3e"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 38], "end": [962, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-889e69c2350f67f8612e8f68"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionCancelQueryRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 17], "end": [962, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-683fc022c7753303d1e2dc84"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 38], "end": [962, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f4e2f96a778b0063812fbdb"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [962, 32], "end": [962, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1dabbec4069ad8b69e2bb67"></a>
## info

`struct_field` · `arrow_flight::sql::gen::ActionCancelQueryRequest::info` · arrow-flight 59.3.0

```rust
info: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:969`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The result of the GetFlightInfo RPC that initiated the query.
XXX(ARROW-16902): this must be a serialized FlightInfo, but is
rendered as bytes because Protobuf does not really support one
DLL using Protobuf definitions from another DLL.

<a id="op-2a0a1fac943378e5da981c44"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionCancelQueryRequest::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryRequest", "path": "ActionCancelQueryRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
