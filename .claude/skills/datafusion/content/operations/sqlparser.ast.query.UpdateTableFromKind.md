# `sqlparser::ast::query::UpdateTableFromKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.UpdateTableFromKind.json).

<a id="op-5bdf0be019a875d833ddf5c2"></a>
## UpdateTableFromKind

`enum` · `sqlparser::ast::query::UpdateTableFromKind` · sqlparser 0.62.0

```rust
enum UpdateTableFromKind
```

Source: `src/ast/query.rs:4185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `FROM` clause of an `UPDATE TABLE` statement

<a id="op-98a30e543b8cfad4899a8532"></a>
## AfterSet

`variant` · `sqlparser::ast::query::UpdateTableFromKind::AfterSet` · sqlparser 0.62.0

```rust
AfterSet
```

Source: `src/ast/query.rs:4191`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Update Statement where the 'FROM' clause is after the 'SET' keyword (Which is the standard way)
For Example: `UPDATE SET t1.name='aaa' FROM t1`

<a id="op-bf1a1e74ab84efd42794b6c3"></a>
## BeforeSet

`variant` · `sqlparser::ast::query::UpdateTableFromKind::BeforeSet` · sqlparser 0.62.0

```rust
BeforeSet
```

Source: `src/ast/query.rs:4188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Update Statement where the 'FROM' clause is before the 'SET' keyword (Supported by Snowflake)
For Example: `UPDATE FROM t1 SET t1.name='aaa'`

<a id="op-bdeb8328f4cbe2df5c67997c"></a>
## clone

`function` · `sqlparser::ast::query::UpdateTableFromKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UpdateTableFromKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 17], "end": [4182, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa266d55fae4566cde4ef111"></a>
## cmp

`function` · `sqlparser::ast::query::UpdateTableFromKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UpdateTableFromKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 51], "end": [4182, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e037e92b3abdf23c4747112a"></a>
## deserialize

`function` · `sqlparser::ast::query::UpdateTableFromKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4183, 49], "end": [4183, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65427664c14054303dfb05a6"></a>
## eq

`function` · `sqlparser::ast::query::UpdateTableFromKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UpdateTableFromKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 24], "end": [4182, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50adae47a7d0fa2c9ba8add0"></a>
## fmt

`function` · `sqlparser::ast::query::UpdateTableFromKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 10], "end": [4182, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-621e25956d0525a896bfeadd"></a>
## hash

`function` · `sqlparser::ast::query::UpdateTableFromKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 56], "end": [4182, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e71f3da78cd7d2413009f54"></a>
## partial_cmp

`function` · `sqlparser::ast::query::UpdateTableFromKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UpdateTableFromKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 35], "end": [4182, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31f7dee146ddd0376f41a8ba"></a>
## serialize

`function` · `sqlparser::ast::query::UpdateTableFromKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4183, 38], "end": [4183, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bda8391d2e5ee8f8e461c459"></a>
## span

`function` · `sqlparser::ast::query::UpdateTableFromKind::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "super::UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2395, 1], "end": [2403, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c365d7495335ee08e441c1c"></a>
## visit

`function` · `sqlparser::ast::query::UpdateTableFromKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4184, 47], "end": [4184, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37f828da4ce38d81ef1c8124"></a>
## visit

`function` · `sqlparser::ast::query::UpdateTableFromKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::UpdateTableFromKind", "path": "UpdateTableFromKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4184, 40], "end": [4184, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
