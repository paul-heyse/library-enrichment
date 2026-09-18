# `sqlparser::ast::dml::MultiTableInsertWhenClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MultiTableInsertWhenClause.json).

<a id="op-bfa94cb95e9f81eff604a244"></a>
## MultiTableInsertWhenClause

`struct` · `sqlparser::ast::dml::MultiTableInsertWhenClause` · sqlparser 0.62.0

```rust
struct MultiTableInsertWhenClause
```

Source: `src/ast/dml.rs:815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A WHEN clause in a conditional multi-table INSERT.

Syntax:
```sql
WHEN n1 > 100 THEN
  INTO t1
  INTO t2 (c1, c2) VALUES (n1, n2)
```

<a id="op-6cae663f551260b30c59304e"></a>
## clone

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MultiTableInsertWhenClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 17], "end": [812, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-339468046f3e31de0ccdabac"></a>
## cmp

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MultiTableInsertWhenClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 51], "end": [812, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5dab02dee0cc2d564e66251"></a>
## condition

`struct_field` · `sqlparser::ast::dml::MultiTableInsertWhenClause::condition` · sqlparser 0.62.0

```rust
condition: super::Expr
```

Source: `src/ast/dml.rs:817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The condition for this WHEN clause

<a id="op-faa83a99c23df80f7322d22d"></a>
## deserialize

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [813, 49], "end": [813, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bba005e2e09ee9c1688b6c3b"></a>
## eq

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MultiTableInsertWhenClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 24], "end": [812, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-124ad77de429944b2b02379f"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [822, 1], "end": [831, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:823`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad9e55f1d6751da3083d6417"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 10], "end": [812, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da0f072129f28fa4a3babf68"></a>
## hash

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 56], "end": [812, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d82a5ff8ea51ddcb798ab033"></a>
## into_clauses

`struct_field` · `sqlparser::ast::dml::MultiTableInsertWhenClause::into_clauses` · sqlparser 0.62.0

```rust
into_clauses: Vec<MultiTableInsertIntoClause>
```

Source: `src/ast/dml.rs:819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The INTO clauses to execute when the condition is true

<a id="op-a9ff8acb7570470f586cb99b"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MultiTableInsertWhenClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 35], "end": [812, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3ccf8723d8487518416909d"></a>
## serialize

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [813, 38], "end": [813, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29e2079d3fda59681f691805"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 40], "end": [814, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ecf0c53d95296ffd0041f01"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertWhenClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertWhenClause", "path": "MultiTableInsertWhenClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 47], "end": [814, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
