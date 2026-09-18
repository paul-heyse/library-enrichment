# `sqlparser::ast::JsonPath`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.JsonPath.json).

<a id="op-33b880ac3ae2526719f291d6"></a>
## JsonPath

`struct` · `sqlparser::ast::JsonPath` · sqlparser 0.62.0

```rust
struct JsonPath
```

Source: `src/ast/mod.rs:711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A JSON path.

See <https://docs.snowflake.com/en/user-guide/querying-semistructured>.
See <https://docs.databricks.com/en/sql/language-manual/sql-ref-json-path-expression.html>.

<a id="op-9319927314bd9e52b0b4f896"></a>
## clone

`function` · `sqlparser::ast::JsonPath::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonPath
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 17], "end": [708, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:708`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98596c3d3bae6feba4ebbf6a"></a>
## cmp

`function` · `sqlparser::ast::JsonPath::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonPath) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 51], "end": [708, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:708`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c43596b43c9584f9128e946a"></a>
## deserialize

`function` · `sqlparser::ast::JsonPath::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 49], "end": [709, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9fd7076d12fc56a2f995745"></a>
## eq

`function` · `sqlparser::ast::JsonPath::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonPath) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 24], "end": [708, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:708`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68876adc3fb495fd7cf91194"></a>
## fmt

`function` · `sqlparser::ast::JsonPath::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [716, 1], "end": [743, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:717`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5f37ece3d4d5d605d4d178b"></a>
## fmt

`function` · `sqlparser::ast::JsonPath::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 10], "end": [708, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:708`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eb10d082251e3f2773da011"></a>
## hash

`function` · `sqlparser::ast::JsonPath::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 56], "end": [708, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:708`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcd1ce7639e14b7cf0855f92"></a>
## partial_cmp

`function` · `sqlparser::ast::JsonPath::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonPath) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 35], "end": [708, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:708`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36db67e6c8ca68f9c88f13be"></a>
## path

`struct_field` · `sqlparser::ast::JsonPath::path` · sqlparser 0.62.0

```rust
path: Vec<JsonPathElem>
```

Source: `src/ast/mod.rs:713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sequence of path elements that form the JSON path.

<a id="op-fab0616cbb893cf17ca14037"></a>
## serialize

`function` · `sqlparser::ast::JsonPath::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 38], "end": [709, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-608ae8a0e34f18450efdf8e7"></a>
## span

`function` · `sqlparser::ast::JsonPath::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "super::JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1804, 1], "end": [1810, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22ae9ccd7078e5ed1cde096d"></a>
## visit

`function` · `sqlparser::ast::JsonPath::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [710, 40], "end": [710, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:710`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-674eda1035806961d32b13ad"></a>
## visit

`function` · `sqlparser::ast::JsonPath::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPath", "path": "JsonPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [710, 47], "end": [710, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:710`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
