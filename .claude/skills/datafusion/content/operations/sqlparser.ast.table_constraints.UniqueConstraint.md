# `sqlparser::ast::table_constraints::UniqueConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.UniqueConstraint.json).

<a id="op-d85a5286c7d6f64c64971f91"></a>
## UniqueConstraint

`struct` · `sqlparser::ast::table_constraints::UniqueConstraint` · sqlparser 0.62.0

```rust
struct UniqueConstraint
```

Source: `src/ast/table_constraints.rs:494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unique constraint definition.

<a id="op-68996531e2c6077dc25c400c"></a>
## characteristics

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::characteristics` · sqlparser 0.62.0

```rust
characteristics: Option<ast::ConstraintCharacteristics>
```

Source: `src/ast/table_constraints.rs:512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional characteristics like `DEFERRABLE`.

<a id="op-26da3ba4c83c163772d5537d"></a>
## clone

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UniqueConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 17], "end": [490, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce549eee49abc4dcf522fca5"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UniqueConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 51], "end": [490, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfa2b34c631387420b60a009"></a>
## columns

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::columns` · sqlparser 0.62.0

```rust
columns: Vec<ast::IndexColumn>
```

Source: `src/ast/table_constraints.rs:508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identifiers of the columns that are unique.

<a id="op-828363037664097b014201c2"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 49], "end": [491, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0b6164a285fbebf172f3b79"></a>
## eq

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UniqueConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 24], "end": [490, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d3e624f673d3b4e8645ad4d"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 10], "end": [490, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-583c873633362d563dd3cd06"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [517, 1], "end": [538, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/table_constraints.rs:518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b2e6d8d5a6ee3413f04ead9"></a>
## hash

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 56], "end": [490, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27ab16e22f1d9f439efd497f"></a>
## index_name

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::index_name` · sqlparser 0.62.0

```rust
index_name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index name

<a id="op-2ea66e992e2623f72394b48f"></a>
## index_options

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::index_options` · sqlparser 0.62.0

```rust
index_options: Vec<ast::IndexOption>
```

Source: `src/ast/table_constraints.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional index options such as `USING`.

<a id="op-bc40ae5e4dd0ddd3b45a8cb2"></a>
## index_type

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::index_type` · sqlparser 0.62.0

```rust
index_type: Option<ast::IndexType>
```

Source: `src/ast/table_constraints.rs:506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `USING` of [index type][1] statement before columns.

[1]: IndexType

<a id="op-75f76fabaf2aeabeab7ea265"></a>
## index_type_display

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::index_type_display` · sqlparser 0.62.0

```rust
index_type_display: ast::KeyOrIndexDisplay
```

Source: `src/ast/table_constraints.rs:502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the type is followed by the keyword `KEY`, `INDEX`, or no keyword at all.

<a id="op-cd606b5e40ee6631617b2317"></a>
## name

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::name` · sqlparser 0.62.0

```rust
name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Constraint name.

Can be not the same as `index_name`

<a id="op-11eeb8bcf8d5bee535237e74"></a>
## nulls_distinct

`struct_field` · `sqlparser::ast::table_constraints::UniqueConstraint::nulls_distinct` · sqlparser 0.62.0

```rust
nulls_distinct: ast::NullsDistinctOption
```

Source: `src/ast/table_constraints.rs:514`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional Postgres nulls handling: `[ NULLS [ NOT ] DISTINCT ]`

<a id="op-7f1eafc6f51c1615f51ef140"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UniqueConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 35], "end": [490, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8807434c0ce344225ac85202"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 38], "end": [491, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-177de35cf105acc7daf29d95"></a>
## span

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [555, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/table_constraints.rs:541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89ca3f5a20238626131256f3"></a>
## visit

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [492, 47], "end": [492, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9673c4553317563bd26baf8c"></a>
## visit

`function` · `sqlparser::ast::table_constraints::UniqueConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [492, 40], "end": [492, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
