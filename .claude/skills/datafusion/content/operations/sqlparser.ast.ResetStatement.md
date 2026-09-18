# `sqlparser::ast::ResetStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ResetStatement.json).

<a id="op-e86f0f67e6ef832a8a3c1dce"></a>
## ResetStatement

`struct` · `sqlparser::ast::ResetStatement` · sqlparser 0.62.0

```rust
struct ResetStatement
```

Source: `src/ast/mod.rs:11970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Resets a session parameter to its default value.
```sql
RESET { ALL | <configuration_parameter> }
```

<a id="op-94a90b61330a5098752a5c0d"></a>
## clone

`function` · `sqlparser::ast::ResetStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ResetStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11967, 17], "end": [11967, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aaf74afb2704dc2d5dc49b73"></a>
## cmp

`function` · `sqlparser::ast::ResetStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ResetStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11967, 51], "end": [11967, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b4b2a5ddd0aec93b7ff8799"></a>
## deserialize

`function` · `sqlparser::ast::ResetStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11968, 49], "end": [11968, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c3d7f674609783c98ec1d99"></a>
## eq

`function` · `sqlparser::ast::ResetStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ResetStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11967, 24], "end": [11967, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9860ab3d46d4b4c5d00452c1"></a>
## fmt

`function` · `sqlparser::ast::ResetStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12036, 1], "end": [12043, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:12037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1a2482482accc43dadf0f83"></a>
## fmt

`function` · `sqlparser::ast::ResetStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11967, 10], "end": [11967, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d902a2a4c27f5de181ed4b56"></a>
## hash

`function` · `sqlparser::ast::ResetStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11967, 56], "end": [11967, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-921f7c7111b054303d88924c"></a>
## partial_cmp

`function` · `sqlparser::ast::ResetStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ResetStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11967, 35], "end": [11967, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-404c318c50f44684992e9728"></a>
## reset

`struct_field` · `sqlparser::ast::ResetStatement::reset` · sqlparser 0.62.0

```rust
reset: Reset
```

Source: `src/ast/mod.rs:11972`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The reset action to perform (either `ALL` or a specific configuration parameter).

<a id="op-f163120ce444e5a3ffb78344"></a>
## serialize

`function` · `sqlparser::ast::ResetStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11968, 38], "end": [11968, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-436618de1b40750c70a1e760"></a>
## visit

`function` · `sqlparser::ast::ResetStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11969, 40], "end": [11969, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2e3a6067a826a4acf4807b4"></a>
## visit

`function` · `sqlparser::ast::ResetStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11969, 47], "end": [11969, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
