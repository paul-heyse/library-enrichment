# `arrow_flight::sql::gen::SqlSupportedResultSetType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlSupportedResultSetType.json).

<a id="op-15e7adac1fd3292f8a2266cf"></a>
## SqlSupportedResultSetType

`enum` · `arrow_flight::sql::gen::SqlSupportedResultSetType` · arrow-flight 59.3.0

```rust
enum SqlSupportedResultSetType
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2335`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-354333f9796cc5300b953ee2"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlSupportedResultSetType::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 68], "end": [2333, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4bfcf4da999ad3b74e14d7e"></a>
## SqlResultSetTypeForwardOnly

`variant` · `arrow_flight::sql::gen::SqlSupportedResultSetType::SqlResultSetTypeForwardOnly` · arrow-flight 59.3.0

```rust
SqlResultSetTypeForwardOnly
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a73a96dfc13f26fe5084c8d2"></a>
## SqlResultSetTypeScrollInsensitive

`variant` · `arrow_flight::sql::gen::SqlSupportedResultSetType::SqlResultSetTypeScrollInsensitive` · arrow-flight 59.3.0

```rust
SqlResultSetTypeScrollInsensitive
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2338`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-905a51793300b8b371f2e987"></a>
## SqlResultSetTypeScrollSensitive

`variant` · `arrow_flight::sql::gen::SqlSupportedResultSetType::SqlResultSetTypeScrollSensitive` · arrow-flight 59.3.0

```rust
SqlResultSetTypeScrollSensitive
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2339`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aadce2d102dbbd3257adda54"></a>
## SqlResultSetTypeUnspecified

`variant` · `arrow_flight::sql::gen::SqlSupportedResultSetType::SqlResultSetTypeUnspecified` · arrow-flight 59.3.0

```rust
SqlResultSetTypeUnspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2336`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c504676335a91ab3beb23cbc"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 1], "end": [2372, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2346`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-0f00b64054bad3cd665fac35"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlSupportedResultSetType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 10], "end": [2333, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-546baff215e9d8e8da53a6aa"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlSupportedResultSetType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 63], "end": [2333, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f383e4924bebb0e9458deb9"></a>
## default

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlSupportedResultSetType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 68], "end": [2333, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e01031e2c48f10db2d77678"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlSupportedResultSetType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 30], "end": [2333, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b776beaf92babcde242a742"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 23], "end": [2333, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5444918e62eb6804cd0c6810"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedResultSetType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 68], "end": [2333, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlSupportedResultSetType`, or `None` if `value` is not a valid variant.

<a id="op-7c190259d62fcdd3c756a120"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 1], "end": [2372, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2359`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-099ab8b954e0a9a7857ff4b3"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 45], "end": [2333, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8515025f92a6ce0f57c65714"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 68], "end": [2333, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlSupportedResultSetType`.

<a id="op-1c9e9d9850872d110bcc3cec"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlSupportedResultSetType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 51], "end": [2333, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8345a0c6e3e62d5ece35d18f"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetType::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedResultSetType, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetType", "path": "SqlSupportedResultSetType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 68], "end": [2333, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2333`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
