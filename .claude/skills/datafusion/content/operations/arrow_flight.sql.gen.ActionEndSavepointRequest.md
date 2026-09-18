# `arrow_flight::sql::gen::ActionEndSavepointRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionEndSavepointRequest.json).

<a id="op-50e3c9af4da44d4caa4169b3"></a>
## ActionEndSavepointRequest

`struct` · `arrow_flight::sql::gen::ActionEndSavepointRequest` · arrow-flight 59.3.0

```rust
struct ActionEndSavepointRequest
```

Source: `src/sql/arrow.flight.protocol.sql.rs:613`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Request message for the "EndSavepoint" action.

Release (RELEASE) the savepoint or rollback (ROLLBACK) to the
savepoint.

Releasing a savepoint invalidates that savepoint.  Rolling back to
a savepoint does not invalidate the savepoint, but invalidates all
savepoints created after the current savepoint.

<a id="op-b4127bc984c8557aa6ec088e"></a>
## action

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::action` · arrow-flight 59.3.0

```rust
fn action(&self) -> action_end_savepoint_request::EndSavepoint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 38], "end": [612, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the enum value of `action`, or the default if the field is set to an invalid enum value.

<a id="op-d2b07925cf6b66608c92f7c7"></a>
## action

`struct_field` · `arrow_flight::sql::gen::ActionEndSavepointRequest::action` · arrow-flight 59.3.0

```rust
action: i32
```

Source: `src/sql/arrow.flight.protocol.sql.rs:619`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Whether to rollback/release the given savepoint.

<a id="op-d305a0b52e89c0126d5fd03b"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f799c36022296bceacfb8fe9"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 38], "end": [612, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-969c24426758a9bcc94d5e30"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionEndSavepointRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 10], "end": [612, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-851d5c76dc5ef7b31fd838d8"></a>
## default

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 38], "end": [612, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3097aed9543d427ade92f542"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 38], "end": [612, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-968db97563750cc5d991de51"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionEndSavepointRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 17], "end": [612, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bd5aeab0e2bd5b964f07228"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 38], "end": [612, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e81335fb972856024581931a"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 32], "end": [612, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6cbfae469cfa9a31c9c1cbc"></a>
## savepoint_id

`struct_field` · `arrow_flight::sql::gen::ActionEndSavepointRequest::savepoint_id` · arrow-flight 59.3.0

```rust
savepoint_id: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:616`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the savepoint on the server.

<a id="op-00817bb80dc3cff304ebd28d"></a>
## set_action

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::set_action` · arrow-flight 59.3.0

```rust
fn set_action(&mut self, value: action_end_savepoint_request::EndSavepoint)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 38], "end": [612, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Sets `action` to the provided enum value.

<a id="op-d4d22b7bbe6d98a7344641ae"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionEndSavepointRequest::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndSavepointRequest", "path": "ActionEndSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
