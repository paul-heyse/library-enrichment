# `arrow_flight::sql::gen::SqlSupportedTransactions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlSupportedTransactions.json).

<a id="op-f30a6c19ff535611871cd4b9"></a>
## SqlSupportedTransactions

`enum` · `arrow_flight::sql::gen::SqlSupportedTransactions` · arrow-flight 59.3.0

```rust
enum SqlSupportedTransactions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2302`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f941c187311ab2d65745e924"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlSupportedTransactions::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 68], "end": [2300, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-456c8adc7dd921ad597a3b11"></a>
## SqlDataDefinitionTransactions

`variant` · `arrow_flight::sql::gen::SqlSupportedTransactions::SqlDataDefinitionTransactions` · arrow-flight 59.3.0

```rust
SqlDataDefinitionTransactions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2304`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23013cadeda1f70d148ee140"></a>
## SqlDataManipulationTransactions

`variant` · `arrow_flight::sql::gen::SqlSupportedTransactions::SqlDataManipulationTransactions` · arrow-flight 59.3.0

```rust
SqlDataManipulationTransactions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2305`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5e7a1ab92c7a896e999c3dd"></a>
## SqlTransactionUnspecified

`variant` · `arrow_flight::sql::gen::SqlSupportedTransactions::SqlTransactionUnspecified` · arrow-flight 59.3.0

```rust
SqlTransactionUnspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2303`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea413fc030768eee6d8c21a9"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2307, 1], "end": [2332, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2312`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-a778ec6fa3d3861bfe211c4b"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlSupportedTransactions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 10], "end": [2300, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c799653b5b76d7e926e1699"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlSupportedTransactions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 63], "end": [2300, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25589471203e0c2ac363f7cd"></a>
## default

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlSupportedTransactions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 68], "end": [2300, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1d89606349686d7915b055d"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlSupportedTransactions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 30], "end": [2300, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e77de2899653c5ab64aa7da"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 23], "end": [2300, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9f3b7c4a0f4ebb1e502617d"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedTransactions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 68], "end": [2300, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlSupportedTransactions`, or `None` if `value` is not a valid variant.

<a id="op-013e7e475db17c8f9937a371"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2307, 1], "end": [2332, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2320`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-b6c74a376699a200401511d9"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 45], "end": [2300, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4adbb694ac3961456ee4dee2"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 68], "end": [2300, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlSupportedTransactions`.

<a id="op-56075f8af912c61a4f2943cd"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlSupportedTransactions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 51], "end": [2300, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34cccdc1ac16d4cfcb429c88"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlSupportedTransactions::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedTransactions, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransactions", "path": "SqlSupportedTransactions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2300, 68], "end": [2300, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2300`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
