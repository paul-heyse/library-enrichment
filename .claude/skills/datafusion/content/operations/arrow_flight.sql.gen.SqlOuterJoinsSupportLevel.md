# `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlOuterJoinsSupportLevel.json).

<a id="op-42b5527ccb5b37888d281c99"></a>
## SqlOuterJoinsSupportLevel

`enum` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel` · arrow-flight 59.3.0

```rust
enum SqlOuterJoinsSupportLevel
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2091`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d81812d623e437a65d39f3b"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 68], "end": [2089, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba81367ee591e08254afe1f2"></a>
## SqlFullOuterJoins

`variant` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::SqlFullOuterJoins` · arrow-flight 59.3.0

```rust
SqlFullOuterJoins
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2094`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-605a0fffbd25d1b9cb543238"></a>
## SqlJoinsUnsupported

`variant` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::SqlJoinsUnsupported` · arrow-flight 59.3.0

```rust
SqlJoinsUnsupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2092`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee12f5e371b7ff1534e2f06a"></a>
## SqlLimitedOuterJoins

`variant` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::SqlLimitedOuterJoins` · arrow-flight 59.3.0

```rust
SqlLimitedOuterJoins
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2093`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cdaceeab59c09399d2e06d8"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2096, 1], "end": [2117, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2101`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-7e3cc51bb9da69bd962e8b4a"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlOuterJoinsSupportLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 10], "end": [2089, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42abf7efb97406e41eb1f0c5"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlOuterJoinsSupportLevel) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 63], "end": [2089, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f2ddc40f0d27fbaf5177e35"></a>
## default

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlOuterJoinsSupportLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 68], "end": [2089, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1007e523089e45320b5f81e7"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlOuterJoinsSupportLevel) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 30], "end": [2089, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39dba4d53633a97131b6863a"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 23], "end": [2089, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62121f867514436aae6c0ad0"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlOuterJoinsSupportLevel>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 68], "end": [2089, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlOuterJoinsSupportLevel`, or `None` if `value` is not a valid variant.

<a id="op-98985b380454102b1b4cbc70"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2096, 1], "end": [2117, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2109`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-47d9b1ca987ced7f7f014dfe"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 45], "end": [2089, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6866289fd48df8ff3d9e909"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 68], "end": [2089, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlOuterJoinsSupportLevel`.

<a id="op-d24158988ad9a7e44fcb75d4"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlOuterJoinsSupportLevel) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 51], "end": [2089, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-929278f8d92c7dd559d95b8c"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlOuterJoinsSupportLevel, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlOuterJoinsSupportLevel", "path": "SqlOuterJoinsSupportLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2089, 68], "end": [2089, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2089`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
