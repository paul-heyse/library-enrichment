# `sqlparser::ast::TruncateTableTarget`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TruncateTableTarget.json).

<a id="op-47f70182f2bba8b653c919b0"></a>
## TruncateTableTarget

`struct` · `sqlparser::ast::TruncateTableTarget` · sqlparser 0.62.0

```rust
struct TruncateTableTarget
```

Source: `src/ast/mod.rs:6489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target of a `TRUNCATE TABLE` command

Note this is its own struct because `visit_relation` requires an `ObjectName` (not a `Vec<ObjectName>`)

<a id="op-80bb365e701c4342e0ebc5c0"></a>
## clone

`function` · `sqlparser::ast::TruncateTableTarget::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TruncateTableTarget
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6486, 17], "end": [6486, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3d6957b3aa826cb02e72f96"></a>
## cmp

`function` · `sqlparser::ast::TruncateTableTarget::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TruncateTableTarget) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6486, 51], "end": [6486, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6fe718f2b6e032b9fdae41c"></a>
## deserialize

`function` · `sqlparser::ast::TruncateTableTarget::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6488, 49], "end": [6488, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6488`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99ceddf74018b8aed756d999"></a>
## eq

`function` · `sqlparser::ast::TruncateTableTarget::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TruncateTableTarget) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6486, 24], "end": [6486, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4730e4d45306de795b48b0a"></a>
## fmt

`function` · `sqlparser::ast::TruncateTableTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6507, 1], "end": [6518, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f202ada3bb839d8ed3c3c7a2"></a>
## fmt

`function` · `sqlparser::ast::TruncateTableTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6486, 10], "end": [6486, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c60803bdf367afc2ed980ae8"></a>
## has_asterisk

`struct_field` · `sqlparser::ast::TruncateTableTarget::has_asterisk` · sqlparser 0.62.0

```rust
has_asterisk: bool
```

Source: `src/ast/mod.rs:6504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres-specific option: asterisk after table name to explicitly indicate descendants
```sql
TRUNCATE TABLE name [ * ]
```
<https://www.postgresql.org/docs/current/sql-truncate.html>

<a id="op-dc9bd0c3428e6627f61f18f8"></a>
## hash

`function` · `sqlparser::ast::TruncateTableTarget::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6486, 56], "end": [6486, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ce2f14f4f529135f2f537d8"></a>
## name

`struct_field` · `sqlparser::ast::TruncateTableTarget::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:6492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

name of the table being truncated

<a id="op-f181cbb76af35a09ce5e741a"></a>
## only

`struct_field` · `sqlparser::ast::TruncateTableTarget::only` · sqlparser 0.62.0

```rust
only: bool
```

Source: `src/ast/mod.rs:6498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres-specific option: explicitly exclude descendants (also default without ONLY)
```sql
TRUNCATE TABLE ONLY name
```
<https://www.postgresql.org/docs/current/sql-truncate.html>

<a id="op-16290e371918c58b7ff009bb"></a>
## partial_cmp

`function` · `sqlparser::ast::TruncateTableTarget::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TruncateTableTarget) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6486, 35], "end": [6486, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3fbf74c89df8b76d8ba8ac0"></a>
## serialize

`function` · `sqlparser::ast::TruncateTableTarget::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6488, 38], "end": [6488, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6488`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22bd6c66744fd72f092a9bca"></a>
## visit

`function` · `sqlparser::ast::TruncateTableTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6487, 47], "end": [6487, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3ab869365afd5c902efece1"></a>
## visit

`function` · `sqlparser::ast::TruncateTableTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TruncateTableTarget", "path": "TruncateTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6487, 40], "end": [6487, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
