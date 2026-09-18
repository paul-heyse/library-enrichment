# `arrow_flight::sql::gen::UpdateDeleteRules`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.UpdateDeleteRules.json).

<a id="op-1208c5b5640a20cb62652aa6"></a>
## UpdateDeleteRules

`enum` · `arrow_flight::sql::gen::UpdateDeleteRules` · arrow-flight 59.3.0

```rust
enum UpdateDeleteRules
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2798`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05db0577aaa41bd88f9b37b4"></a>
## Cascade

`variant` · `arrow_flight::sql::gen::UpdateDeleteRules::Cascade` · arrow-flight 59.3.0

```rust
Cascade
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2799`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b31d8243678a6a5e2c1c2887"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::UpdateDeleteRules::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 68], "end": [2796, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9381c31dbf62970467e3a662"></a>
## NoAction

`variant` · `arrow_flight::sql::gen::UpdateDeleteRules::NoAction` · arrow-flight 59.3.0

```rust
NoAction
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2802`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aaf8f61883825fea3b76af4c"></a>
## Restrict

`variant` · `arrow_flight::sql::gen::UpdateDeleteRules::Restrict` · arrow-flight 59.3.0

```rust
Restrict
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2800`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48ddaf59d243f61459725706"></a>
## SetDefault

`variant` · `arrow_flight::sql::gen::UpdateDeleteRules::SetDefault` · arrow-flight 59.3.0

```rust
SetDefault
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2803`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1643398f64c3e7486c597db"></a>
## SetNull

`variant` · `arrow_flight::sql::gen::UpdateDeleteRules::SetNull` · arrow-flight 59.3.0

```rust
SetNull
```

Source: `src/sql/arrow.flight.protocol.sql.rs:2801`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1052ccf0f41bf654dd6107b3"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2805, 1], "end": [2830, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2810`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-c79b0ecaffc4964c2add2dcf"></a>
## clone

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> UpdateDeleteRules
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 10], "end": [2796, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cb898c90740c977b59d8aa9"></a>
## cmp

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &UpdateDeleteRules) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 63], "end": [2796, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06ae55edfe79612004bbbfd3"></a>
## default

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::default` · arrow-flight 59.3.0

```rust
fn default() -> UpdateDeleteRules
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 68], "end": [2796, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fef5837b0efe0bec16d62445"></a>
## eq

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &UpdateDeleteRules) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 30], "end": [2796, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73dff57d6eff5a841fdcee28"></a>
## fmt

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 23], "end": [2796, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b55bf7aa065355f7a12a48d5"></a>
## from_i32

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<UpdateDeleteRules>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 68], "end": [2796, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `UpdateDeleteRules`, or `None` if `value` is not a valid variant.

<a id="op-fb1534ceb82adacc3d5ddf80"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2805, 1], "end": [2830, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2820`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-5d9fc1e603dbb0551d8762dd"></a>
## hash

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 45], "end": [2796, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b77317e69960d8239b12405"></a>
## is_valid

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 68], "end": [2796, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `UpdateDeleteRules`.

<a id="op-8989b0f699cd8c12a738b9cb"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &UpdateDeleteRules) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 51], "end": [2796, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffb5f478a528b6543a3d3756"></a>
## try_from

`function` · `arrow_flight::sql::gen::UpdateDeleteRules::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<UpdateDeleteRules, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::UpdateDeleteRules", "path": "UpdateDeleteRules"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2796, 68], "end": [2796, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:2796`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
