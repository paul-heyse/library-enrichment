# `sqlparser::ast::ddl::IdentityParameters`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IdentityParameters.json).

<a id="op-7580bf8c8f91bd6dd4ee1454"></a>
## IdentityParameters

`struct` · `sqlparser::ast::ddl::IdentityParameters` · sqlparser 0.62.0

```rust
struct IdentityParameters
```

Source: `src/ast/ddl.rs:1774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parameters specifying seed and increment for identity columns.

<a id="op-a10c05cd25dd95ffdb4aca75"></a>
## clone

`function` · `sqlparser::ast::ddl::IdentityParameters::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IdentityParameters
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 17], "end": [1771, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e58ee00ac6e41345b43dc031"></a>
## cmp

`function` · `sqlparser::ast::ddl::IdentityParameters::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IdentityParameters) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 51], "end": [1771, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec3cb347424438112ddfa253"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IdentityParameters::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1772, 49], "end": [1772, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1772`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd8920e80c08a3ec7e96514c"></a>
## eq

`function` · `sqlparser::ast::ddl::IdentityParameters::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IdentityParameters) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 24], "end": [1771, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69d3a0266a52ad3bfead374b"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityParameters::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 10], "end": [1771, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a79f2066622247c3d02ddd1a"></a>
## hash

`function` · `sqlparser::ast::ddl::IdentityParameters::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 56], "end": [1771, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-126d3383ba00bb20ba297f0d"></a>
## increment

`struct_field` · `sqlparser::ast::ddl::IdentityParameters::increment` · sqlparser 0.62.0

```rust
increment: ast::Expr
```

Source: `src/ast/ddl.rs:1778`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The increment expression for the identity column.

<a id="op-d81d760bf44c22670077cc57"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IdentityParameters::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IdentityParameters) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 35], "end": [1771, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bf18e66936a64e29ce356aa"></a>
## seed

`struct_field` · `sqlparser::ast::ddl::IdentityParameters::seed` · sqlparser 0.62.0

```rust
seed: ast::Expr
```

Source: `src/ast/ddl.rs:1776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The initial seed expression for the identity column.

<a id="op-558b1977d23796ff22ffe6d2"></a>
## serialize

`function` · `sqlparser::ast::ddl::IdentityParameters::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1772, 38], "end": [1772, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1772`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59cb586d728085eefecd25b4"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityParameters::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1773, 47], "end": [1773, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86d88d4ab5a3746734e08d04"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityParameters::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityParameters", "path": "IdentityParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1773, 40], "end": [1773, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
