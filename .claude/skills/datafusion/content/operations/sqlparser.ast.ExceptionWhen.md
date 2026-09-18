# `sqlparser::ast::ExceptionWhen`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ExceptionWhen.json).

<a id="op-da0bc7c594dd13e9d92c4813"></a>
## ExceptionWhen

`struct` · `sqlparser::ast::ExceptionWhen` · sqlparser 0.62.0

```rust
struct ExceptionWhen
```

Source: `src/ast/mod.rs:3442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A representation of a `WHEN` arm with all the identifiers catched and the statements to execute
for the arm.

Snowflake: <https://docs.snowflake.com/en/sql-reference/snowflake-scripting/exception>
BigQuery: <https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#beginexceptionend>

<a id="op-7bc7b5cefd6cf3e8efe0e738"></a>
## clone

`function` · `sqlparser::ast::ExceptionWhen::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExceptionWhen
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3439, 17], "end": [3439, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56c0ed09da628d33b55644fc"></a>
## cmp

`function` · `sqlparser::ast::ExceptionWhen::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExceptionWhen) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3439, 51], "end": [3439, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-053dcbf12751bf13e6adef4c"></a>
## deserialize

`function` · `sqlparser::ast::ExceptionWhen::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3440, 49], "end": [3440, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3440`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0dedd71a9dc0365bb7f3071"></a>
## eq

`function` · `sqlparser::ast::ExceptionWhen::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExceptionWhen) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3439, 24], "end": [3439, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39b0599f0296458f14985fe9"></a>
## fmt

`function` · `sqlparser::ast::ExceptionWhen::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3439, 10], "end": [3439, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-789b4748d3afe792ca4cff5d"></a>
## fmt

`function` · `sqlparser::ast::ExceptionWhen::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3449, 1], "end": [3464, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:3450`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-565d133810bab3db61e93bcc"></a>
## hash

`function` · `sqlparser::ast::ExceptionWhen::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3439, 56], "end": [3439, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcb3671abb949640eb2c5b4c"></a>
## idents

`struct_field` · `sqlparser::ast::ExceptionWhen::idents` · sqlparser 0.62.0

```rust
idents: Vec<Ident>
```

Source: `src/ast/mod.rs:3444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identifiers that trigger this branch (error conditions).

<a id="op-4adb83a30e4835e3820e7a55"></a>
## partial_cmp

`function` · `sqlparser::ast::ExceptionWhen::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExceptionWhen) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3439, 35], "end": [3439, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-978d766434041ff2ca636dfd"></a>
## serialize

`function` · `sqlparser::ast::ExceptionWhen::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3440, 38], "end": [3440, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3440`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae1c192095ddd765ec351df1"></a>
## statements

`struct_field` · `sqlparser::ast::ExceptionWhen::statements` · sqlparser 0.62.0

```rust
statements: Vec<Statement>
```

Source: `src/ast/mod.rs:3446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Statements to execute when the condition matches.

<a id="op-8bc066392e221a2c7a4ea2a7"></a>
## visit

`function` · `sqlparser::ast::ExceptionWhen::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3441, 47], "end": [3441, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da5d1534d5efa2f246ec9a19"></a>
## visit

`function` · `sqlparser::ast::ExceptionWhen::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExceptionWhen", "path": "ExceptionWhen"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3441, 40], "end": [3441, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
