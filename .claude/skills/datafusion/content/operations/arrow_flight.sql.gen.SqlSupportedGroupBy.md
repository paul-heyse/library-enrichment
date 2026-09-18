# `arrow_flight::sql::gen::SqlSupportedGroupBy`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlSupportedGroupBy.json).

<a id="op-bd9995e5231b010e098cdb96"></a>
## SqlSupportedGroupBy

`enum` · `arrow_flight::sql::gen::SqlSupportedGroupBy` · arrow-flight 59.3.0

```rust
enum SqlSupportedGroupBy
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2120`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67ac5204c6fd029ec13a72d7"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlSupportedGroupBy::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 68], "end": [2118, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-190a9db638fdd40cbad1c355"></a>
## SqlGroupByBeyondSelect

`variant` · `arrow_flight::sql::gen::SqlSupportedGroupBy::SqlGroupByBeyondSelect` · arrow-flight 59.3.0

```rust
SqlGroupByBeyondSelect
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2122`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-938f439dfd61e1eccc756a1b"></a>
## SqlGroupByUnrelated

`variant` · `arrow_flight::sql::gen::SqlSupportedGroupBy::SqlGroupByUnrelated` · arrow-flight 59.3.0

```rust
SqlGroupByUnrelated
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04cfc09a3cba954f9de4e984"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2124, 1], "end": [2143, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2129`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-a0714cccd94cb4ded332bf0a"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlSupportedGroupBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 10], "end": [2118, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aabdefaeecbc92d298c02006"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlSupportedGroupBy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 63], "end": [2118, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b764fa0498e8e795f41b2ed"></a>
## default

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlSupportedGroupBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 68], "end": [2118, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36cdfd3c7756d2db58503b06"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlSupportedGroupBy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 30], "end": [2118, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4253027dbc5343609a461a3"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 23], "end": [2118, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9d1089a03768582e45dd9d9"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedGroupBy>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 68], "end": [2118, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlSupportedGroupBy`, or `None` if `value` is not a valid variant.

<a id="op-a27ae75278f5ad2073d11814"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2124, 1], "end": [2143, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-44648a4c8392054045960c85"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 45], "end": [2118, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7b54c82d8d875a0df46dba2"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 68], "end": [2118, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlSupportedGroupBy`.

<a id="op-d97b144e30c1e60432945857"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlSupportedGroupBy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 51], "end": [2118, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-413f16b6cec3ada1c9c5753f"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlSupportedGroupBy::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedGroupBy, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedGroupBy", "path": "SqlSupportedGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2118, 68], "end": [2118, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2118`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
