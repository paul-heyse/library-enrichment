# `sqlparser::ast::ddl::IdentityPropertyOrder`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IdentityPropertyOrder.json).

<a id="op-a734795eea4c10b91ea77b03"></a>
## IdentityPropertyOrder

`enum` · `sqlparser::ast::ddl::IdentityPropertyOrder` · sqlparser 0.62.0

```rust
enum IdentityPropertyOrder
```

Source: `src/ast/ddl.rs:1790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The identity column option specifies how values are generated for the auto-incremented column, either in increasing or decreasing order.
Syntax
```sql
ORDER | NOORDER
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-eb24b77ff5f9bcbad5f2b9ea"></a>
## NoOrder

`variant` · `sqlparser::ast::ddl::IdentityPropertyOrder::NoOrder` · sqlparser 0.62.0

```rust
NoOrder
```

Source: `src/ast/ddl.rs:1794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NOORDER` - do not enforce ordering for generated values.

<a id="op-519d7e9078579cf378776c0f"></a>
## Order

`variant` · `sqlparser::ast::ddl::IdentityPropertyOrder::Order` · sqlparser 0.62.0

```rust
Order
```

Source: `src/ast/ddl.rs:1792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ORDER` - preserve ordering for generated values (where supported).

<a id="op-2f15c3944fc35dd75d1e695d"></a>
## clone

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IdentityPropertyOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1787, 17], "end": [1787, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51b2eff879e2d1f588e0a12f"></a>
## cmp

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IdentityPropertyOrder) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1787, 57], "end": [1787, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee8f13d802061bfea0f50c64"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1788, 49], "end": [1788, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4da4bc7ebd8f77d8d3a4e06e"></a>
## eq

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IdentityPropertyOrder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1787, 30], "end": [1787, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99015df015a6ae4c8be04d96"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1797, 1], "end": [1804, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d79803d0997002a21870625a"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1787, 10], "end": [1787, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acf7225bd4af09f2d6a05fe8"></a>
## hash

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1787, 62], "end": [1787, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31826fcd16901087a6940cca"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IdentityPropertyOrder) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1787, 41], "end": [1787, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1071c0c4b95e28c6d7c9ac51"></a>
## serialize

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1788, 38], "end": [1788, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33937c554fe794cd45953f39"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 40], "end": [1789, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-365486770ca5d6b37e7e44aa"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityPropertyOrder::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyOrder", "path": "IdentityPropertyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1789, 47], "end": [1789, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
