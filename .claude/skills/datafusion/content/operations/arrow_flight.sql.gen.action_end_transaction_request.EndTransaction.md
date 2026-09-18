# `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.action_end_transaction_request.EndTransaction.json).

<a id="op-a42058ca20a39cb61d4922e8"></a>
## EndTransaction

`enum` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction` · arrow-flight 59.3.0

```rust
enum EndTransaction
```

Source: `src/sql/arrow.flight.protocol.sql.rs:573`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24b2825a96b0c16a52598e51"></a>
## Commit

`variant` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::Commit` · arrow-flight 59.3.0

```rust
Commit
```

Source: `src/sql/arrow.flight.protocol.sql.rs:576`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Commit the transaction.

<a id="op-feb068bd3e6e35989bfc0f58"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 9], "end": [570, 29], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fa2759767f860db821c9462"></a>
## Rollback

`variant` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::Rollback` · arrow-flight 59.3.0

```rust
Rollback
```

Source: `src/sql/arrow.flight.protocol.sql.rs:578`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Roll back the transaction.

<a id="op-e6b9a247889470d48570856d"></a>
## Unspecified

`variant` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::Unspecified` · arrow-flight 59.3.0

```rust
Unspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:574`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a5ad2e9caba29db5e9c94bb"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 5], "end": [601, 6], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:585`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-fcfd7da33768258bc0cdab35"></a>
## clone

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> EndTransaction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 9], "end": [562, 14], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4567085314b578d72c56588c"></a>
## cmp

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &EndTransaction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [569, 9], "end": [569, 12], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:569`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fed81adc2feb9a112a503dcc"></a>
## default

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::default` · arrow-flight 59.3.0

```rust
fn default() -> EndTransaction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 9], "end": [570, 29], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afebe682bae815437c6affb0"></a>
## eq

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &EndTransaction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 9], "end": [565, 18], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:565`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa5a277495af6ccb6b54e8b1"></a>
## fmt

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 9], "end": [564, 14], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:564`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb300267d46d7702d3cd3f85"></a>
## from_i32

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<EndTransaction>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 9], "end": [570, 29], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `EndTransaction`, or `None` if `value` is not a valid variant.

<a id="op-11fa5eec24dd2769956b3570"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 5], "end": [601, 6], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:593`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-5302f8777c1effa147b012d8"></a>
## hash

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 9], "end": [567, 13], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:567`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a87edbabf1b4088fae6d6bfc"></a>
## is_valid

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 9], "end": [570, 29], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `EndTransaction`.

<a id="op-422819ba80afcaed3187fe19"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &EndTransaction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [568, 9], "end": [568, 19], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:568`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6620496b6b3819c6c97cc9ef"></a>
## try_from

`function` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<EndTransaction, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::action_end_transaction_request::EndTransaction", "path": "EndTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 9], "end": [570, 29], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
