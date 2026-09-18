# `sqlparser::ast::CastFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CastFormat.json).

<a id="op-8aecc4d73b63967600728946"></a>
## CastFormat

`enum` · `sqlparser::ast::CastFormat` · sqlparser 0.62.0

```rust
enum CastFormat
```

Source: `src/ast/mod.rs:665`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options for `CAST` / `TRY_CAST`
BigQuery: <https://cloud.google.com/bigquery/docs/reference/standard-sql/format-elements#formatting_syntax>

<a id="op-aae777d7054350debd3e889c"></a>
## Value

`variant` · `sqlparser::ast::CastFormat::Value` · sqlparser 0.62.0

```rust
Value
```

Source: `src/ast/mod.rs:667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A simple cast format specified by a `Value`.

<a id="op-047fc1b3589a9cc34c753250"></a>
## ValueAtTimeZone

`variant` · `sqlparser::ast::CastFormat::ValueAtTimeZone` · sqlparser 0.62.0

```rust
ValueAtTimeZone
```

Source: `src/ast/mod.rs:669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A cast format with an explicit time zone: `(format, timezone)`.

<a id="op-105f257f3a6a6cc32b69a37d"></a>
## clone

`function` · `sqlparser::ast::CastFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CastFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 17], "end": [662, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc144ffcff86c2987c77a654"></a>
## cmp

`function` · `sqlparser::ast::CastFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CastFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 51], "end": [662, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c09459f240aa503a716de91f"></a>
## deserialize

`function` · `sqlparser::ast::CastFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 49], "end": [663, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7890d8c5e5d8324364155cd"></a>
## eq

`function` · `sqlparser::ast::CastFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CastFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 24], "end": [662, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0379c653c82fb5b003358f2e"></a>
## fmt

`function` · `sqlparser::ast::CastFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1708, 1], "end": [1715, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c3f552aea52e1d8d67dffcf"></a>
## fmt

`function` · `sqlparser::ast::CastFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 10], "end": [662, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-906944b35fcc67cc411b6c04"></a>
## hash

`function` · `sqlparser::ast::CastFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 56], "end": [662, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d669041ad4907393e2e8a31b"></a>
## partial_cmp

`function` · `sqlparser::ast::CastFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CastFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 35], "end": [662, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36481ce7385bb8f33bc46d2e"></a>
## serialize

`function` · `sqlparser::ast::CastFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 38], "end": [663, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-312d18ae3497aa3a4e185f68"></a>
## visit

`function` · `sqlparser::ast::CastFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 47], "end": [664, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0282db0c2cca2a5197e46f9"></a>
## visit

`function` · `sqlparser::ast::CastFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastFormat", "path": "CastFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 40], "end": [664, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
