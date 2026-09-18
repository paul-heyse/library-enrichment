# `arrow_flight::sql::gen::ActionEndTransactionRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionEndTransactionRequest.json).

<a id="op-37a83b1dbf8d2da210a4f07c"></a>
## ActionEndTransactionRequest

`struct` · `arrow_flight::sql::gen::ActionEndTransactionRequest` · arrow-flight 59.3.0

```rust
struct ActionEndTransactionRequest
```

Source: `src/sql/arrow.flight.protocol.sql.rs:551`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Request message for the "EndTransaction" action.

Commit (COMMIT) or rollback (ROLLBACK) the transaction.

If the action completes successfully, the transaction handle is
invalidated, as are all associated savepoints.

<a id="op-5a052288aad10b31b88dc230"></a>
## action

`struct_field` · `arrow_flight::sql::gen::ActionEndTransactionRequest::action` · arrow-flight 59.3.0

```rust
action: i32
```

Source: `src/sql/arrow.flight.protocol.sql.rs:557`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Whether to commit/rollback the given transaction.

<a id="op-8dd6b77f244a78599cb54c30"></a>
## action

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::action` · arrow-flight 59.3.0

```rust
fn action(&self) -> action_end_transaction_request::EndTransaction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the enum value of `action`, or the default if the field is set to an invalid enum value.

<a id="op-186523b92af6dcb3cadcfc2d"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68cbb16a0e72eaa27d911c87"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cb761efa15ba0ae841e79cc"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionEndTransactionRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 10], "end": [550, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4533d586e99c01067e50883a"></a>
## default

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef1daadc150dbc9f5a43c69a"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6db5c0360765ebd141542d4"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionEndTransactionRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 17], "end": [550, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad09b33dfe1fa4192c0e944d"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53c2bb133f054a78a4f878aa"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 32], "end": [550, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09acffd9b71a1bf8c9857d9c"></a>
## set_action

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::set_action` · arrow-flight 59.3.0

```rust
fn set_action(&mut self, value: action_end_transaction_request::EndTransaction)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Sets `action` to the provided enum value.

<a id="op-f2f9b7c2cd6b042b89033c51"></a>
## transaction_id

`struct_field` · `arrow_flight::sql::gen::ActionEndTransactionRequest::transaction_id` · arrow-flight 59.3.0

```rust
transaction_id: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:554`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the transaction on the server.

<a id="op-8fab7a79a49b828d31e46c8f"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionEndTransactionRequest::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionEndTransactionRequest", "path": "ActionEndTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
