# `sqlparser::ast::query::XmlTableColumnOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.XmlTableColumnOption.json).

<a id="op-671e3a730ad9e37a46985a18"></a>
## XmlTableColumnOption

`enum` · `sqlparser::ast::query::XmlTableColumnOption` · sqlparser 0.62.0

```rust
enum XmlTableColumnOption
```

Source: `src/ast/query.rs:4198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Defines the options for an XmlTable column: Named or ForOrdinality

<a id="op-dc898eec1e01654f91015a60"></a>
## ForOrdinality

`variant` · `sqlparser::ast::query::XmlTableColumnOption::ForOrdinality` · sqlparser 0.62.0

```rust
ForOrdinality
```

Source: `src/ast/query.rs:4211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The FOR ORDINALITY marker

<a id="op-d35d45058247f4b9ac14046e"></a>
## NamedInfo

`variant` · `sqlparser::ast::query::XmlTableColumnOption::NamedInfo` · sqlparser 0.62.0

```rust
NamedInfo
```

Source: `src/ast/query.rs:4200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A named column with a type, optional path, and default value.

<a id="op-14272a13772f587eafa24fc4"></a>
## clone

`function` · `sqlparser::ast::query::XmlTableColumnOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> XmlTableColumnOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4195, 17], "end": [4195, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c2eb0aca8a4c2cce9e2faed"></a>
## cmp

`function` · `sqlparser::ast::query::XmlTableColumnOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &XmlTableColumnOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4195, 51], "end": [4195, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3770afa7a02659072d47b45"></a>
## deserialize

`function` · `sqlparser::ast::query::XmlTableColumnOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4197, 49], "end": [4197, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4197`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5b4f47db2868c7ff1173d41"></a>
## eq

`function` · `sqlparser::ast::query::XmlTableColumnOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &XmlTableColumnOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4195, 24], "end": [4195, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa79e8ab2586e12aabb538cc"></a>
## fmt

`function` · `sqlparser::ast::query::XmlTableColumnOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4195, 10], "end": [4195, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b24c1221ea747eedc6b288f4"></a>
## hash

`function` · `sqlparser::ast::query::XmlTableColumnOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4195, 56], "end": [4195, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4576a69dc652a0ab8b1442f"></a>
## partial_cmp

`function` · `sqlparser::ast::query::XmlTableColumnOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &XmlTableColumnOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4195, 35], "end": [4195, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47b427f5979bfdb7975a682e"></a>
## serialize

`function` · `sqlparser::ast::query::XmlTableColumnOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4197, 38], "end": [4197, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4197`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-032d8cc6be1934983bf3eb26"></a>
## visit

`function` · `sqlparser::ast::query::XmlTableColumnOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4196, 40], "end": [4196, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33ee44bde8562b2aac4cdcd7"></a>
## visit

`function` · `sqlparser::ast::query::XmlTableColumnOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumnOption", "path": "XmlTableColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4196, 47], "end": [4196, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
