# `sqlparser::ast::query::CteAsMaterialized`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.CteAsMaterialized.json).

<a id="op-98f7583ca3a1c9edff930e7d"></a>
## CteAsMaterialized

`enum` · `sqlparser::ast::query::CteAsMaterialized` · sqlparser 0.62.0

```rust
enum CteAsMaterialized
```

Source: `src/ast/query.rs:777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates whether a CTE is materialized or not.

<a id="op-27960d051d9df40af7e16413"></a>
## Materialized

`variant` · `sqlparser::ast::query::CteAsMaterialized::Materialized` · sqlparser 0.62.0

```rust
Materialized
```

Source: `src/ast/query.rs:779`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `WITH` statement specifies `AS MATERIALIZED` behavior

<a id="op-599e1b124eb4af50907dfefc"></a>
## NotMaterialized

`variant` · `sqlparser::ast::query::CteAsMaterialized::NotMaterialized` · sqlparser 0.62.0

```rust
NotMaterialized
```

Source: `src/ast/query.rs:781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `WITH` statement specifies `AS NOT MATERIALIZED` behavior

<a id="op-9137413568b956e1721af6b5"></a>
## clone

`function` · `sqlparser::ast::query::CteAsMaterialized::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CteAsMaterialized
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [773, 17], "end": [773, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c76608f3e990f28d44c09669"></a>
## cmp

`function` · `sqlparser::ast::query::CteAsMaterialized::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CteAsMaterialized) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [773, 57], "end": [773, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c9a2564fd018becd689c1b7"></a>
## deserialize

`function` · `sqlparser::ast::query::CteAsMaterialized::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [774, 49], "end": [774, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46804d327c19c039ee8f7f45"></a>
## eq

`function` · `sqlparser::ast::query::CteAsMaterialized::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CteAsMaterialized) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [773, 30], "end": [773, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fc3e192b738da7727b430fa"></a>
## fmt

`function` · `sqlparser::ast::query::CteAsMaterialized::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [773, 10], "end": [773, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9805bdaf6b56096c2e252ca5"></a>
## fmt

`function` · `sqlparser::ast::query::CteAsMaterialized::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [784, 1], "end": [796, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cabc688a69de4379eb4f62c"></a>
## hash

`function` · `sqlparser::ast::query::CteAsMaterialized::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [773, 62], "end": [773, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6e2ce0446decfa0f52abe57"></a>
## partial_cmp

`function` · `sqlparser::ast::query::CteAsMaterialized::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CteAsMaterialized) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [773, 41], "end": [773, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0530bb3caf0a1c3d7f365b26"></a>
## serialize

`function` · `sqlparser::ast::query::CteAsMaterialized::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [774, 38], "end": [774, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ce3f32058fcc69563fbdfea"></a>
## visit

`function` · `sqlparser::ast::query::CteAsMaterialized::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [775, 47], "end": [775, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fe4b948e0cb6aa280e6c74e"></a>
## visit

`function` · `sqlparser::ast::query::CteAsMaterialized::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::CteAsMaterialized", "path": "CteAsMaterialized"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [775, 40], "end": [775, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
