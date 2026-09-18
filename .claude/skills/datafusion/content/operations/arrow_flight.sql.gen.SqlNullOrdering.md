# `arrow_flight::sql::gen::SqlNullOrdering`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlNullOrdering.json).

<a id="op-221e07e27a5b59bc3c6652c9"></a>
## SqlNullOrdering

`enum` · `arrow_flight::sql::gen::SqlNullOrdering` · arrow-flight 59.3.0

```rust
enum SqlNullOrdering
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2001`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90e63d7cb1ee7adf6193955c"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlNullOrdering::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 68], "end": [1999, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f5b625f5d8ed5f1ede81edc"></a>
## SqlNullsSortedAtEnd

`variant` · `arrow_flight::sql::gen::SqlNullOrdering::SqlNullsSortedAtEnd` · arrow-flight 59.3.0

```rust
SqlNullsSortedAtEnd
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2005`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8383ac7d380a2aebe106ab50"></a>
## SqlNullsSortedAtStart

`variant` · `arrow_flight::sql::gen::SqlNullOrdering::SqlNullsSortedAtStart` · arrow-flight 59.3.0

```rust
SqlNullsSortedAtStart
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2004`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20c0b6f9e0b3f3b9d94f68df"></a>
## SqlNullsSortedHigh

`variant` · `arrow_flight::sql::gen::SqlNullOrdering::SqlNullsSortedHigh` · arrow-flight 59.3.0

```rust
SqlNullsSortedHigh
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2002`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86801c66484f9eb5a8fcb9ba"></a>
## SqlNullsSortedLow

`variant` · `arrow_flight::sql::gen::SqlNullOrdering::SqlNullsSortedLow` · arrow-flight 59.3.0

```rust
SqlNullsSortedLow
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2003`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f874072f8588ead4df19fc2e"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlNullOrdering::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2007, 1], "end": [2030, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2012`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-5fc53be4c8e653bb7f8feb2c"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlNullOrdering::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlNullOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 10], "end": [1999, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60b773a7796cdaa84add1498"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlNullOrdering::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlNullOrdering) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 63], "end": [1999, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62c28f79ce42f3f3fb79c1f0"></a>
## default

`function` · `arrow_flight::sql::gen::SqlNullOrdering::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlNullOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 68], "end": [1999, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-112aa9ce3e57f200e344e89e"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlNullOrdering::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlNullOrdering) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 30], "end": [1999, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18c3f726f2ac166cc8ae6167"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlNullOrdering::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 23], "end": [1999, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8df72494273afa848577e396"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlNullOrdering::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlNullOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 68], "end": [1999, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlNullOrdering`, or `None` if `value` is not a valid variant.

<a id="op-c58b0d27b60fcf6faf68a432"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlNullOrdering::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2007, 1], "end": [2030, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2021`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-bc8f5418687fa92a89387b95"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlNullOrdering::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 45], "end": [1999, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5981f371da50b5fdacf0a90d"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlNullOrdering::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 68], "end": [1999, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlNullOrdering`.

<a id="op-e61b522705e024360c8ff484"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlNullOrdering::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlNullOrdering) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 51], "end": [1999, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d58f5b83bdfc8e2330e9d89"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlNullOrdering::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlNullOrdering, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlNullOrdering", "path": "SqlNullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1999, 68], "end": [1999, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1999`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
