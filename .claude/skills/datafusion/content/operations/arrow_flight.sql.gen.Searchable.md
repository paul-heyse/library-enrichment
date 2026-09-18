# `arrow_flight::sql::gen::Searchable`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.Searchable.json).

<a id="op-bef3badc071bdcd0ccfcb4f7"></a>
## Searchable

`enum` · `arrow_flight::sql::gen::Searchable` · arrow-flight 59.3.0

```rust
enum Searchable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2753`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7096e8e8a2817d80a805f1c"></a>
## Basic

`variant` · `arrow_flight::sql::gen::Searchable::Basic` · arrow-flight 59.3.0

```rust
Basic
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
Indicates that the column can be used In a WHERE clause with any
operator other than LIKE.

- Allowed operators: comparison, quantified comparison, BETWEEN,
                      DISTINCT, IN, MATCH, and UNIQUE.

<a id="op-9d4a95a528eb116d98dd7f68"></a>
## Char

`variant` · `arrow_flight::sql::gen::Searchable::Char` · arrow-flight 59.3.0

```rust
Char
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2760`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
Indicates that the column can be used in a WHERE clause if it is using a
LIKE operator.

<a id="op-bd00970284813e11bdfe891a"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::Searchable::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 68], "end": [2751, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03eb3771871fdfd43ff40093"></a>
## Full

`variant` · `arrow_flight::sql::gen::Searchable::Full` · arrow-flight 59.3.0

```rust
Full
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2770`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
Indicates that the column can be used in a WHERE clause using any operator.

<a id="op-d3562e2f6a8ac3d40702963a"></a>
## None

`variant` · `arrow_flight::sql::gen::Searchable::None` · arrow-flight 59.3.0

```rust
None
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2756`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
Indicates that column cannot be used in a WHERE clause.

<a id="op-4561a607960b3d23077d6d1c"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::Searchable::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2772, 1], "end": [2795, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-ced4ac177afbf8f0d0906e39"></a>
## clone

`function` · `arrow_flight::sql::gen::Searchable::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> Searchable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 10], "end": [2751, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26bb52592b3c394557e3b0e9"></a>
## cmp

`function` · `arrow_flight::sql::gen::Searchable::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &Searchable) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 63], "end": [2751, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99e31b8afb9fc7cbcb21baa4"></a>
## default

`function` · `arrow_flight::sql::gen::Searchable::default` · arrow-flight 59.3.0

```rust
fn default() -> Searchable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 68], "end": [2751, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c49c81126a32cf12b28682"></a>
## eq

`function` · `arrow_flight::sql::gen::Searchable::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &Searchable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 30], "end": [2751, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da171d88c61a4ebd8ec3a570"></a>
## fmt

`function` · `arrow_flight::sql::gen::Searchable::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 23], "end": [2751, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a259eb69627c95350a5f1ab"></a>
## from_i32

`function` · `arrow_flight::sql::gen::Searchable::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<Searchable>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 68], "end": [2751, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `Searchable`, or `None` if `value` is not a valid variant.

<a id="op-40f62a5358b20aefaf1d75cd"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::Searchable::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2772, 1], "end": [2795, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2786`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-fc85923eb41a9397b5738798"></a>
## hash

`function` · `arrow_flight::sql::gen::Searchable::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 45], "end": [2751, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2113f0c73cac51eed5106938"></a>
## is_valid

`function` · `arrow_flight::sql::gen::Searchable::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 68], "end": [2751, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `Searchable`.

<a id="op-233b8dcba064518423d381ba"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::Searchable::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &Searchable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 51], "end": [2751, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6ced690e86a607c2c0b371d"></a>
## try_from

`function` · `arrow_flight::sql::gen::Searchable::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<Searchable, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Searchable", "path": "Searchable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2751, 68], "end": [2751, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2751`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
