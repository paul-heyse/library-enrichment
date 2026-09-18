# `sqlparser::ast::query::TopQuantity`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TopQuantity.json).

<a id="op-91799e4eca2d0c31f22deabe"></a>
## TopQuantity

`enum` · `sqlparser::ast::query::TopQuantity` · sqlparser 0.62.0

```rust
enum TopQuantity
```

Source: `src/ast/query.rs:3625`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Quantity used in a `TOP` clause: either an expression or a constant.

<a id="op-e6e1d0f0ed38a38ef34e3f99"></a>
## Constant

`variant` · `sqlparser::ast::query::TopQuantity::Constant` · sqlparser 0.62.0

```rust
Constant
```

Source: `src/ast/query.rs:3629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unparenthesized integer constant: `TOP 10`.

<a id="op-f70c8585b04b0e68954079d0"></a>
## Expr

`variant` · `sqlparser::ast::query::TopQuantity::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/query.rs:3627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A parenthesized expression (MSSQL syntax: `TOP (expr)`).

<a id="op-e029866cce3c3cd25fd9f508"></a>
## clone

`function` · `sqlparser::ast::query::TopQuantity::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TopQuantity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3621, 17], "end": [3621, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef464c9ffc0cb9147524fe26"></a>
## cmp

`function` · `sqlparser::ast::query::TopQuantity::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TopQuantity) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3621, 51], "end": [3621, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b066ac6963073b7b45378491"></a>
## deserialize

`function` · `sqlparser::ast::query::TopQuantity::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3622, 49], "end": [3622, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f14e3836cc6306897b4d675"></a>
## eq

`function` · `sqlparser::ast::query::TopQuantity::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TopQuantity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3621, 24], "end": [3621, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32c61e382ea2860e0ad36164"></a>
## fmt

`function` · `sqlparser::ast::query::TopQuantity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3621, 10], "end": [3621, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03940f014288f7c2996605dc"></a>
## hash

`function` · `sqlparser::ast::query::TopQuantity::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3621, 56], "end": [3621, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe55702c366e59f4025b094f"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TopQuantity::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TopQuantity) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3621, 35], "end": [3621, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e7971f09e61115575e3957c"></a>
## serialize

`function` · `sqlparser::ast::query::TopQuantity::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3622, 38], "end": [3622, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ef0e836f354f92a16bedc0f"></a>
## visit

`function` · `sqlparser::ast::query::TopQuantity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3623, 47], "end": [3623, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3623`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7b1eb6db73f7aad77d18668"></a>
## visit

`function` · `sqlparser::ast::query::TopQuantity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TopQuantity", "path": "TopQuantity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3623, 40], "end": [3623, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3623`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
