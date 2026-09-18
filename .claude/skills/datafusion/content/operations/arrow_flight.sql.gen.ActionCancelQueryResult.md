# `arrow_flight::sql::gen::ActionCancelQueryResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionCancelQueryResult.json).

<a id="op-6c919ca13402f4af9908aed8"></a>
## ActionCancelQueryResult

`struct` · `arrow_flight::sql::gen::ActionCancelQueryResult` · arrow-flight 59.3.0

```rust
struct ActionCancelQueryResult
```

Source: `src/sql/arrow.flight.protocol.sql.rs:979`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The result of cancelling a query.

The result should be wrapped in a google.protobuf.Any message.

This command is deprecated since 13.0.0. Use the "CancelFlightInfo"
action with DoAction instead.

<a id="op-899ec1bc96897af3aea9c607"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9cdbea90ae4587b37a8bc2e"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 44], "end": [978, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e9ad5852cdb7fd571cc6509"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionCancelQueryResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 10], "end": [978, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5e5de1a58031d0f9b70aec1"></a>
## default

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 44], "end": [978, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-890a836148c93e3fbdc81062"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 44], "end": [978, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bb145ab2f2a96fadc915807"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionCancelQueryResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 23], "end": [978, 32], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ceb9a565e9a8d7cf2ce8f835"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 44], "end": [978, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b784fda133222e5cc60289e"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 38], "end": [978, 42], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f8c705e75347355da89d727"></a>
## result

`struct_field` · `arrow_flight::sql::gen::ActionCancelQueryResult::result` · arrow-flight 59.3.0

```rust
result: i32
```

Source: `src/sql/arrow.flight.protocol.sql.rs:981`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b889b0ca2851001fc234946e"></a>
## result

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::result` · arrow-flight 59.3.0

```rust
fn result(&self) -> action_cancel_query_result::CancelResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 44], "end": [978, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the enum value of `result`, or the default if the field is set to an invalid enum value.

<a id="op-0c4a31f54c2757ba19d0ed47"></a>
## set_result

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::set_result` · arrow-flight 59.3.0

```rust
fn set_result(&mut self, value: action_cancel_query_result::CancelResult)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [978, 44], "end": [978, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:978`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Sets `result` to the provided enum value.

<a id="op-b14e3989dedca28ff198516b"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionCancelQueryResult::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCancelQueryResult", "path": "ActionCancelQueryResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
