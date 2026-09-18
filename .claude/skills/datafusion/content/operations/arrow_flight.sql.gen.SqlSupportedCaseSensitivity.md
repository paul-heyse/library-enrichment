# `arrow_flight::sql::gen::SqlSupportedCaseSensitivity`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlSupportedCaseSensitivity.json).

<a id="op-2f10db61562226e07743c78f"></a>
## SqlSupportedCaseSensitivity

`enum` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity` · arrow-flight 59.3.0

```rust
enum SqlSupportedCaseSensitivity
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1965`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05a7b588d2eaef2eaf9b8302"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 68], "end": [1963, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa3caae86686ccf97cacd51d"></a>
## SqlCaseSensitivityCaseInsensitive

`variant` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::SqlCaseSensitivityCaseInsensitive` · arrow-flight 59.3.0

```rust
SqlCaseSensitivityCaseInsensitive
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1967`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ef2c9065215ff3cae0f190b"></a>
## SqlCaseSensitivityLowercase

`variant` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::SqlCaseSensitivityLowercase` · arrow-flight 59.3.0

```rust
SqlCaseSensitivityLowercase
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1969`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c22e08b356aad2e60fb100c3"></a>
## SqlCaseSensitivityUnknown

`variant` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::SqlCaseSensitivityUnknown` · arrow-flight 59.3.0

```rust
SqlCaseSensitivityUnknown
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1966`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2068bd42aa922ed0888bc93"></a>
## SqlCaseSensitivityUppercase

`variant` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::SqlCaseSensitivityUppercase` · arrow-flight 59.3.0

```rust
SqlCaseSensitivityUppercase
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1968`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a451f12df5ad44b714029c"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1971, 1], "end": [1998, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1976`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-8fdc4f62f440a5aa496b5fd3"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlSupportedCaseSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 10], "end": [1963, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac23f131df0532bfabd75227"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlSupportedCaseSensitivity) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 63], "end": [1963, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ece73630c8fc7cb323e51c4f"></a>
## default

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlSupportedCaseSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 68], "end": [1963, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ced4f800a91ca59cc5267725"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlSupportedCaseSensitivity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 30], "end": [1963, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b053252d9c9bf7c98156d1ca"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 23], "end": [1963, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f1d3c00e33d73b1118f311e"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedCaseSensitivity>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 68], "end": [1963, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlSupportedCaseSensitivity`, or `None` if `value` is not a valid variant.

<a id="op-00ecb7993048c76d442e18d6"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1971, 1], "end": [1998, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1987`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-f250a815f68db86a74d20233"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 45], "end": [1963, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f258cc40d1bdd2225bedac46"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 68], "end": [1963, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlSupportedCaseSensitivity`.

<a id="op-09983a275553d69133ff7ae1"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlSupportedCaseSensitivity) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 51], "end": [1963, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5f3a3929423b3653bbdee7b"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedCaseSensitivity, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedCaseSensitivity", "path": "SqlSupportedCaseSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1963, 68], "end": [1963, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
