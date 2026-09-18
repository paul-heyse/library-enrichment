# `arrow_flight::sql::gen::SqlSupportedSubqueries`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlSupportedSubqueries.json).

<a id="op-1f654569cf5b34f472c7f45b"></a>
## SqlSupportedSubqueries

`enum` · `arrow_flight::sql::gen::SqlSupportedSubqueries` · arrow-flight 59.3.0

```rust
enum SqlSupportedSubqueries
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2207`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30642b962c7cd3ce3e1d75ee"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlSupportedSubqueries::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 68], "end": [2205, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d42689d28ef16622e85e426"></a>
## SqlSubqueriesInComparisons

`variant` · `arrow_flight::sql::gen::SqlSupportedSubqueries::SqlSubqueriesInComparisons` · arrow-flight 59.3.0

```rust
SqlSubqueriesInComparisons
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2208`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-332d8fd87684ef651bab62b8"></a>
## SqlSubqueriesInExists

`variant` · `arrow_flight::sql::gen::SqlSupportedSubqueries::SqlSubqueriesInExists` · arrow-flight 59.3.0

```rust
SqlSubqueriesInExists
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2209`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b44ed5a041c4639027a58ba"></a>
## SqlSubqueriesInIns

`variant` · `arrow_flight::sql::gen::SqlSupportedSubqueries::SqlSubqueriesInIns` · arrow-flight 59.3.0

```rust
SqlSubqueriesInIns
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2210`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee60687cd3ad4778ddca21aa"></a>
## SqlSubqueriesInQuantifieds

`variant` · `arrow_flight::sql::gen::SqlSupportedSubqueries::SqlSubqueriesInQuantifieds` · arrow-flight 59.3.0

```rust
SqlSubqueriesInQuantifieds
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2211`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abb61c47b18fc8a5f3988f6c"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2213, 1], "end": [2236, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2218`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-f4aaa9a5b0f1765a90854464"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlSupportedSubqueries
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 10], "end": [2205, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-341c60197c606f66d320ccdd"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlSupportedSubqueries) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 63], "end": [2205, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb4bf720c7310cac720d5fe7"></a>
## default

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlSupportedSubqueries
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 68], "end": [2205, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-576bf5aa67f5c3238a5c46a5"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlSupportedSubqueries) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 30], "end": [2205, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6084e1338ffcffcc4b0f5236"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 23], "end": [2205, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d87c492e22cbe3fb4f215e64"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedSubqueries>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 68], "end": [2205, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlSupportedSubqueries`, or `None` if `value` is not a valid variant.

<a id="op-37640e902ae69d63f575aa08"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2213, 1], "end": [2236, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2227`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-e8c5ea8571411645633ae22a"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 45], "end": [2205, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f9017a50b1585bf8b781d99"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 68], "end": [2205, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlSupportedSubqueries`.

<a id="op-21e1563d5e0724ec81f41d06"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlSupportedSubqueries) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 51], "end": [2205, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6f880f4dc2fac732734d699"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlSupportedSubqueries::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedSubqueries, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedSubqueries", "path": "SqlSupportedSubqueries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 68], "end": [2205, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
