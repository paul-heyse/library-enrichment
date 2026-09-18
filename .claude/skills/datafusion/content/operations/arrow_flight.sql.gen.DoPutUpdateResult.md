# `arrow_flight::sql::gen::DoPutUpdateResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.DoPutUpdateResult.json).

<a id="op-c7303218225208a718e473b9"></a>
## DoPutUpdateResult

`struct` · `arrow_flight::sql::gen::DoPutUpdateResult` · arrow-flight 59.3.0

```rust
struct DoPutUpdateResult
```

Source: `src/sql/arrow.flight.protocol.sql.rs:922`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Returned from the RPC call DoPut when a CommandStatementUpdate,
CommandPreparedStatementUpdate, or CommandStatementIngest was
in the request, containing results from the update.

<a id="op-39501050ea6ecd7542c5043f"></a>
## as_any

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffa40fb28ce7dd7e48a9b715"></a>
## clear

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 44], "end": [921, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-061b657e78a1dc665092d045"></a>
## clone

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> DoPutUpdateResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 10], "end": [921, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-017d9f6b59cd76429b2cbb25"></a>
## default

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 44], "end": [921, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd4f9ab8e26dcc85f36587a9"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 44], "end": [921, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60beb8b2e2ce29ddcaac4456"></a>
## eq

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &DoPutUpdateResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 23], "end": [921, 32], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e8d4420d1f316823074e193"></a>
## fmt

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 44], "end": [921, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62fdbfcdb4c92b880ee1d369"></a>
## hash

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 38], "end": [921, 42], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7fb9d80c668906c43c781dd"></a>
## record_count

`struct_field` · `arrow_flight::sql::gen::DoPutUpdateResult::record_count` · arrow-flight 59.3.0

```rust
record_count: i64
```

Source: `src/sql/arrow.flight.protocol.sql.rs:926`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The number of records updated. A return value of -1 represents
an unknown updated record count.

<a id="op-029f747b2e378622748c39a5"></a>
## type_url

`function` · `arrow_flight::sql::gen::DoPutUpdateResult::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutUpdateResult", "path": "DoPutUpdateResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
