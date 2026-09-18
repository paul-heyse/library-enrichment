# `arrow_flight::sql::gen::SqlTransactionIsolationLevel`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlTransactionIsolationLevel.json).

<a id="op-9eb2235b7e9e01062d03043c"></a>
## SqlTransactionIsolationLevel

`enum` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel` · arrow-flight 59.3.0

```rust
enum SqlTransactionIsolationLevel
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2265`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e96e5b7f7d26e775a02180ba"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 68], "end": [2263, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3590e7d8ba46c6bbdfffe4b8"></a>
## SqlTransactionNone

`variant` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::SqlTransactionNone` · arrow-flight 59.3.0

```rust
SqlTransactionNone
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2266`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ba4b4e7782f7bacc07a1675"></a>
## SqlTransactionReadCommitted

`variant` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::SqlTransactionReadCommitted` · arrow-flight 59.3.0

```rust
SqlTransactionReadCommitted
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2268`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27a9bce1e710698cddde9520"></a>
## SqlTransactionReadUncommitted

`variant` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::SqlTransactionReadUncommitted` · arrow-flight 59.3.0

```rust
SqlTransactionReadUncommitted
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2267`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35cd4287229ee58e82517f27"></a>
## SqlTransactionRepeatableRead

`variant` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::SqlTransactionRepeatableRead` · arrow-flight 59.3.0

```rust
SqlTransactionRepeatableRead
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2269`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-487a2956b309483baf62979d"></a>
## SqlTransactionSerializable

`variant` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::SqlTransactionSerializable` · arrow-flight 59.3.0

```rust
SqlTransactionSerializable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2270`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b9957af64c1fc2d94e90fd0"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 1], "end": [2299, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2277`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-5c30bade3da608de32d934cc"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlTransactionIsolationLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 10], "end": [2263, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f1ee1a525434b3e27b6ac2f"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlTransactionIsolationLevel) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 63], "end": [2263, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-775c929ae9081d6a1cdc3cb2"></a>
## default

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlTransactionIsolationLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 68], "end": [2263, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-466d963524e8bb33603d9256"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlTransactionIsolationLevel) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 30], "end": [2263, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecf884d592de596b4d626ad8"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 23], "end": [2263, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66c333e51e74aff1e2c0e633"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlTransactionIsolationLevel>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 68], "end": [2263, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlTransactionIsolationLevel`, or `None` if `value` is not a valid variant.

<a id="op-1894c804fe4b6df7070fc303"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 1], "end": [2299, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2287`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-fd07ddedefc57516c831df8c"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 45], "end": [2263, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64acdc617670d4e979f1064f"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 68], "end": [2263, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlTransactionIsolationLevel`.

<a id="op-fffb089d4cd74c29932957ff"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlTransactionIsolationLevel) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 51], "end": [2263, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e71881923c8908826ec5a60"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlTransactionIsolationLevel, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlTransactionIsolationLevel", "path": "SqlTransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2263, 68], "end": [2263, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
