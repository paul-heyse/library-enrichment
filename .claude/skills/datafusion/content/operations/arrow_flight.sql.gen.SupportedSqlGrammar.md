# `arrow_flight::sql::gen::SupportedSqlGrammar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SupportedSqlGrammar.json).

<a id="op-f6a41dd177a8c8c673dfc453"></a>
## SupportedSqlGrammar

`enum` · `arrow_flight::sql::gen::SupportedSqlGrammar` · arrow-flight 59.3.0

```rust
enum SupportedSqlGrammar
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2033`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f037b877ea18cbc3c961989"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SupportedSqlGrammar::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 68], "end": [2031, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-047a62e52ef45e2555da220a"></a>
## SqlCoreGrammar

`variant` · `arrow_flight::sql::gen::SupportedSqlGrammar::SqlCoreGrammar` · arrow-flight 59.3.0

```rust
SqlCoreGrammar
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2035`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6b1574c75cdf3dac0a6fddb"></a>
## SqlExtendedGrammar

`variant` · `arrow_flight::sql::gen::SupportedSqlGrammar::SqlExtendedGrammar` · arrow-flight 59.3.0

```rust
SqlExtendedGrammar
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2036`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-650f107a86b00619fce8a718"></a>
## SqlMinimumGrammar

`variant` · `arrow_flight::sql::gen::SupportedSqlGrammar::SqlMinimumGrammar` · arrow-flight 59.3.0

```rust
SqlMinimumGrammar
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2034`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcab4c944986217fe287cffe"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2038, 1], "end": [2059, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2043`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-7b92650a27b654df845e8ccd"></a>
## clone

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SupportedSqlGrammar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 10], "end": [2031, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a7108ab9da2b6f47f1488b3"></a>
## cmp

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SupportedSqlGrammar) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 63], "end": [2031, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07cb7601e8e5383efc2b4b31"></a>
## default

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::default` · arrow-flight 59.3.0

```rust
fn default() -> SupportedSqlGrammar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 68], "end": [2031, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7443a4297faae41d05d17399"></a>
## eq

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SupportedSqlGrammar) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 30], "end": [2031, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d84a25fbabc792ce0331c04f"></a>
## fmt

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 23], "end": [2031, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a471b718d66a517dd190979b"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SupportedSqlGrammar>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 68], "end": [2031, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SupportedSqlGrammar`, or `None` if `value` is not a valid variant.

<a id="op-e01f5b3761765847af149bf5"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2038, 1], "end": [2059, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2051`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-41df9e93c9c27f0ae9ac6ffb"></a>
## hash

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 45], "end": [2031, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16aea28ef0745c138e59eac3"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 68], "end": [2031, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SupportedSqlGrammar`.

<a id="op-e696de4090900755921e020a"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SupportedSqlGrammar) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 51], "end": [2031, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac341461a02aa0ccfe24c315"></a>
## try_from

`function` · `arrow_flight::sql::gen::SupportedSqlGrammar::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SupportedSqlGrammar, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SupportedSqlGrammar", "path": "SupportedSqlGrammar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2031, 68], "end": [2031, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2031`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
