# `arrow_flight::sql::gen::Nullable`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.Nullable.json).

<a id="op-84c5d3b76c4193b930f9fe69"></a>
## Nullable

`enum` · `arrow_flight::sql::gen::Nullable` · arrow-flight 59.3.0

```rust
enum Nullable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2718`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fec82af7aa84d00e7aefe5b"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::Nullable::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 68], "end": [2716, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30fc82dd2b04e114cb39d0b5"></a>
## NullabilityNoNulls

`variant` · `arrow_flight::sql::gen::Nullable::NullabilityNoNulls` · arrow-flight 59.3.0

```rust
NullabilityNoNulls
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2721`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
Indicates that the fields does not allow the use of null values.

<a id="op-44c69ab6d8f3f9b22f35b352"></a>
## NullabilityNullable

`variant` · `arrow_flight::sql::gen::Nullable::NullabilityNullable` · arrow-flight 59.3.0

```rust
NullabilityNullable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2724`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
Indicates that the fields allow the use of null values.

<a id="op-ff14eebf770eccc0fa57c99b"></a>
## NullabilityUnknown

`variant` · `arrow_flight::sql::gen::Nullable::NullabilityUnknown` · arrow-flight 59.3.0

```rust
NullabilityUnknown
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2727`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
Indicates that nullability of the fields cannot be determined.

<a id="op-87f7d1f9767291b5348a212d"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::Nullable::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2729, 1], "end": [2750, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2734`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-d3683eada41ffdbc265dcb9f"></a>
## clone

`function` · `arrow_flight::sql::gen::Nullable::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> Nullable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 10], "end": [2716, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15f55d9b0f31c9b7932f8e70"></a>
## cmp

`function` · `arrow_flight::sql::gen::Nullable::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &Nullable) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 63], "end": [2716, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-397054d041cb5ea1375bba80"></a>
## default

`function` · `arrow_flight::sql::gen::Nullable::default` · arrow-flight 59.3.0

```rust
fn default() -> Nullable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 68], "end": [2716, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-931095a3ecaa241a9ce1f5ef"></a>
## eq

`function` · `arrow_flight::sql::gen::Nullable::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &Nullable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 30], "end": [2716, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43084bddf159fd6fc434208d"></a>
## fmt

`function` · `arrow_flight::sql::gen::Nullable::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 23], "end": [2716, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d078390ebe64b56613b20ca"></a>
## from_i32

`function` · `arrow_flight::sql::gen::Nullable::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<Nullable>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 68], "end": [2716, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `Nullable`, or `None` if `value` is not a valid variant.

<a id="op-bd061908448a1d6664057340"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::Nullable::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2729, 1], "end": [2750, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2742`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-d3a2ab1c8ba573b01bc5a551"></a>
## hash

`function` · `arrow_flight::sql::gen::Nullable::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 45], "end": [2716, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc2dd025d13ef3b6fa8581fa"></a>
## is_valid

`function` · `arrow_flight::sql::gen::Nullable::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 68], "end": [2716, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `Nullable`.

<a id="op-60aee8055ff4de7c3a1fc5e1"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::Nullable::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &Nullable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 51], "end": [2716, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db11e3657f77baace2c70d43"></a>
## try_from

`function` · `arrow_flight::sql::gen::Nullable::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<Nullable, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::Nullable", "path": "Nullable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2716, 68], "end": [2716, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2716`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
