# `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlSupportedResultSetConcurrency.json).

<a id="op-cfa3fcbb91d53e565bb1dccf"></a>
## SqlSupportedResultSetConcurrency

`enum` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency` · arrow-flight 59.3.0

```rust
enum SqlSupportedResultSetConcurrency
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2375`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0450cbf85845d238502010e7"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 68], "end": [2373, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-742d22e8945189aa5a5b544f"></a>
## SqlResultSetConcurrencyReadOnly

`variant` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::SqlResultSetConcurrencyReadOnly` · arrow-flight 59.3.0

```rust
SqlResultSetConcurrencyReadOnly
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2377`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9983779810e76d7d855d8d48"></a>
## SqlResultSetConcurrencyUnspecified

`variant` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::SqlResultSetConcurrencyUnspecified` · arrow-flight 59.3.0

```rust
SqlResultSetConcurrencyUnspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2376`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef8092a9aee1deaec3de897a"></a>
## SqlResultSetConcurrencyUpdatable

`variant` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::SqlResultSetConcurrencyUpdatable` · arrow-flight 59.3.0

```rust
SqlResultSetConcurrencyUpdatable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2378`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cef0020a0e48f9191463951"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2380, 1], "end": [2413, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2385`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-7d0fa266fa8ad2b0c06bd410"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlSupportedResultSetConcurrency
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 10], "end": [2373, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9b20556b2163043a576deb2"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlSupportedResultSetConcurrency) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 63], "end": [2373, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4818bf0db6b0265440d92bca"></a>
## default

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlSupportedResultSetConcurrency
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 68], "end": [2373, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-422c11895b0a7b04a0920d24"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlSupportedResultSetConcurrency) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 30], "end": [2373, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31ae6470e8b0b889e36bc5b8"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 23], "end": [2373, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcd9e127bd1d9ff816adc5f4"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedResultSetConcurrency>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 68], "end": [2373, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlSupportedResultSetConcurrency`, or `None` if `value` is not a valid variant.

<a id="op-3a46aef7e78fc9ba62a228c5"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2380, 1], "end": [2413, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2399`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-19667717a682df9791ea873a"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 45], "end": [2373, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9504b87ee8c3988dfb9d194"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 68], "end": [2373, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlSupportedResultSetConcurrency`.

<a id="op-38a57e3648b7c085454372dc"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlSupportedResultSetConcurrency) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 51], "end": [2373, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea4a0912180bb4d6f7b8f14b"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedResultSetConcurrency, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedResultSetConcurrency", "path": "SqlSupportedResultSetConcurrency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2373, 68], "end": [2373, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2373`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
