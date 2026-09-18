# `sqlparser::ast::query::ExprWithAliasAndOrderBy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ExprWithAliasAndOrderBy.json).

<a id="op-8767418f70ab92a953f0e95e"></a>
## ExprWithAliasAndOrderBy

`struct` · `sqlparser::ast::query::ExprWithAliasAndOrderBy` · sqlparser 0.62.0

```rust
struct ExprWithAliasAndOrderBy
```

Source: `src/ast/query.rs:1340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression optionally followed by an alias and order by options.

Example:
```sql
42 AS myint ASC
```

<a id="op-2587ee3b662c059eb39358f6"></a>
## clone

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExprWithAliasAndOrderBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1337, 17], "end": [1337, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bc747669c496ee2d09ac2f1"></a>
## cmp

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExprWithAliasAndOrderBy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1337, 51], "end": [1337, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f338625804a9692803784ca7"></a>
## deserialize

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 49], "end": [1338, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1338`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-559d575ef273ccba9789aeec"></a>
## eq

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExprWithAliasAndOrderBy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1337, 24], "end": [1337, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20f0273523ced1d647793d4e"></a>
## expr

`struct_field` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::expr` · sqlparser 0.62.0

```rust
expr: ExprWithAlias
```

Source: `src/ast/query.rs:1342`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression with optional alias.

<a id="op-97f6c39d1b71de0d09375cd7"></a>
## fmt

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1337, 10], "end": [1337, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fad5a6259ed750f44314e6b7"></a>
## fmt

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1347, 1], "end": [1351, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10b9a0cf18eaeb5e63a110af"></a>
## hash

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1337, 56], "end": [1337, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2043fabe4d039d00b4562cf4"></a>
## order_by

`struct_field` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::order_by` · sqlparser 0.62.0

```rust
order_by: OrderByOptions
```

Source: `src/ast/query.rs:1344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Ordering options applied to the expression.

<a id="op-282e5b4ba87ef1f1c7334385"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExprWithAliasAndOrderBy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1337, 35], "end": [1337, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf8e863fdb9162e38513834c"></a>
## serialize

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1338, 38], "end": [1338, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1338`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd13efd4693dfb0e6cc77a32"></a>
## visit

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1339, 47], "end": [1339, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1339`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e646d2db5223bb360cf648a4"></a>
## visit

`function` · `sqlparser::ast::query::ExprWithAliasAndOrderBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAliasAndOrderBy", "path": "ExprWithAliasAndOrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1339, 40], "end": [1339, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1339`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
