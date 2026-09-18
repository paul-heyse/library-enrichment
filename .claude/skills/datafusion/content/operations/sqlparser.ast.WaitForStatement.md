# `sqlparser::ast::WaitForStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WaitForStatement.json).

<a id="op-6c5103bb70b5202f19890373"></a>
## WaitForStatement

`struct` · `sqlparser::ast::WaitForStatement` · sqlparser 0.62.0

```rust
struct WaitForStatement
```

Source: `src/ast/mod.rs:11326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MSSQL `WAITFOR` statement.

See: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/waitfor-transact-sql>

<a id="op-5c4a7a60d7bfa2e02003d1c6"></a>
## clone

`function` · `sqlparser::ast::WaitForStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WaitForStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11323, 17], "end": [11323, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a50bd55bec978ec1e2faa706"></a>
## cmp

`function` · `sqlparser::ast::WaitForStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WaitForStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11323, 51], "end": [11323, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-586f6fa45399651b6d17df48"></a>
## deserialize

`function` · `sqlparser::ast::WaitForStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11324, 49], "end": [11324, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9a5b89970fff7d446099de2"></a>
## eq

`function` · `sqlparser::ast::WaitForStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WaitForStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11323, 24], "end": [11323, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df1c55f3f6b7c3f14f3141f9"></a>
## expr

`struct_field` · `sqlparser::ast::WaitForStatement::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/mod.rs:11330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The time expression.

<a id="op-3a2ba6eb13332a6ba28cc3fc"></a>
## fmt

`function` · `sqlparser::ast::WaitForStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11333, 1], "end": [11337, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b094ee64da30c69bb96b03b"></a>
## fmt

`function` · `sqlparser::ast::WaitForStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11323, 10], "end": [11323, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6fd854ec5430f07bf4ae39a"></a>
## hash

`function` · `sqlparser::ast::WaitForStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11323, 56], "end": [11323, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25b9980980bc32aab141956c"></a>
## partial_cmp

`function` · `sqlparser::ast::WaitForStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WaitForStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11323, 35], "end": [11323, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-706a7ac47f8ba4ffa98ef997"></a>
## serialize

`function` · `sqlparser::ast::WaitForStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11324, 38], "end": [11324, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05221804dae5e673d3b9960c"></a>
## visit

`function` · `sqlparser::ast::WaitForStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11325, 40], "end": [11325, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-533b4e95da1a38d68a786648"></a>
## visit

`function` · `sqlparser::ast::WaitForStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForStatement", "path": "WaitForStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11325, 47], "end": [11325, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f2a6afc230f3c24ca236b53"></a>
## wait_type

`struct_field` · `sqlparser::ast::WaitForStatement::wait_type` · sqlparser 0.62.0

```rust
wait_type: WaitForType
```

Source: `src/ast/mod.rs:11328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DELAY` or `TIME`.
