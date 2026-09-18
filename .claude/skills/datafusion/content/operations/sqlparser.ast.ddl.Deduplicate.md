# `sqlparser::ast::ddl::Deduplicate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.Deduplicate.json).

<a id="op-594598578b09fadd705eb99e"></a>
## Deduplicate

`enum` · `sqlparser::ast::ddl::Deduplicate` · sqlparser 0.62.0

```rust
enum Deduplicate
```

Source: `src/ast/ddl.rs:2756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DEDUPLICATE statement used in OPTIMIZE TABLE et al. such as in ClickHouse SQL
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

<a id="op-a9202540f1ddf5dbdfd2dd69"></a>
## All

`variant` · `sqlparser::ast::ddl::Deduplicate::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/ddl.rs:2758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DEDUPLICATE ALL

<a id="op-5b81a8dcd6bef91f94dcb2a3"></a>
## ByExpression

`variant` · `sqlparser::ast::ddl::Deduplicate::ByExpression` · sqlparser 0.62.0

```rust
ByExpression
```

Source: `src/ast/ddl.rs:2760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DEDUPLICATE BY expr

<a id="op-4b8d10f3dfe73e0dc536942e"></a>
## clone

`function` · `sqlparser::ast::ddl::Deduplicate::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Deduplicate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2753, 17], "end": [2753, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efa7a8d6215953aed83a8258"></a>
## cmp

`function` · `sqlparser::ast::ddl::Deduplicate::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Deduplicate) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2753, 51], "end": [2753, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58b5f30baab9b2a6e7757f59"></a>
## deserialize

`function` · `sqlparser::ast::ddl::Deduplicate::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2754, 49], "end": [2754, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bbb7df7bfdbd62817fa8011"></a>
## eq

`function` · `sqlparser::ast::ddl::Deduplicate::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Deduplicate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2753, 24], "end": [2753, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49067c10ae8a067f70e4d063"></a>
## fmt

`function` · `sqlparser::ast::ddl::Deduplicate::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2763, 1], "end": [2770, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2764`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f430724f55f2c74a7e593ef4"></a>
## fmt

`function` · `sqlparser::ast::ddl::Deduplicate::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2753, 10], "end": [2753, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7daa2720fbd68f27dda7308"></a>
## hash

`function` · `sqlparser::ast::ddl::Deduplicate::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2753, 56], "end": [2753, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c06922abff249849c0cecd96"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::Deduplicate::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Deduplicate) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2753, 35], "end": [2753, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e14fdd009465666463e49707"></a>
## serialize

`function` · `sqlparser::ast::ddl::Deduplicate::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2754, 38], "end": [2754, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-858a401b537cc2457a81c64b"></a>
## visit

`function` · `sqlparser::ast::ddl::Deduplicate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2755, 47], "end": [2755, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb10babc1d319c9a49335184"></a>
## visit

`function` · `sqlparser::ast::ddl::Deduplicate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Deduplicate", "path": "Deduplicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2755, 40], "end": [2755, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
