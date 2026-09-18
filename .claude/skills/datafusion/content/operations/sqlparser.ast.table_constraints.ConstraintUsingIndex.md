# `sqlparser::ast::table_constraints::ConstraintUsingIndex`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.ConstraintUsingIndex.json).

<a id="op-7a7a1c6cbf4a3ed9d1cfc10e"></a>
## ConstraintUsingIndex

`struct` · `sqlparser::ast::table_constraints::ConstraintUsingIndex` · sqlparser 0.62.0

```rust
struct ConstraintUsingIndex
```

Source: `src/ast/table_constraints.rs:566`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL constraint that promotes an existing unique index to a table constraint.

`[ CONSTRAINT constraint_name ] { UNIQUE | PRIMARY KEY } USING INDEX index_name
  [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]`

See <https://www.postgresql.org/docs/current/sql-altertable.html>

<a id="op-7c6076babdad4e0342e88436"></a>
## characteristics

`struct_field` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::characteristics` · sqlparser 0.62.0

```rust
characteristics: Option<ast::ConstraintCharacteristics>
```

Source: `src/ast/table_constraints.rs:572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional characteristics like `DEFERRABLE`.

<a id="op-3d3fa3df5aa9d80831a58bb3"></a>
## clone

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ConstraintUsingIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 17], "end": [563, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed595adf79f7d66f832aaa43"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ConstraintUsingIndex) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 51], "end": [563, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a67b1564904beedc7f6cce60"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 49], "end": [564, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d59befb3cdf72003e1f2cfbb"></a>
## eq

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ConstraintUsingIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 24], "end": [563, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-183ad05a149cddfa6d0b0c87"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 10], "end": [563, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85504fa70c97518e924d6f84"></a>
## fmt_with_keyword

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::fmt_with_keyword` · sqlparser 0.62.0

```rust
fn fmt_with_keyword(&self, f: &mut fmt::Formatter<'_>, keyword: &str) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [575, 1], "end": [589, 2], "filename": "src/ast/table_constraints.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/table_constraints.rs:577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Format as `[CONSTRAINT name] <keyword> USING INDEX index_name [characteristics]`.

<a id="op-0420dd53bacf87b622ff9f08"></a>
## hash

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 56], "end": [563, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ded78c471307a4a2016d3794"></a>
## index_name

`struct_field` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::index_name` · sqlparser 0.62.0

```rust
index_name: ast::Ident
```

Source: `src/ast/table_constraints.rs:570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the existing unique index to promote.

<a id="op-c975f769e50b345a4d99c453"></a>
## name

`struct_field` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::name` · sqlparser 0.62.0

```rust
name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:568`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional constraint name.

<a id="op-2afe93b586c7b5824db2b98a"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ConstraintUsingIndex) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 35], "end": [563, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9715f2a051d90eeab19fe6f4"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 38], "end": [564, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2c82ca8d1e5d0b396de7c56"></a>
## span

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [605, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/table_constraints.rs:592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89d672947c2be9143f75b509"></a>
## visit

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 40], "end": [565, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f34adcf2f9252b77e7b21af3"></a>
## visit

`function` · `sqlparser::ast::table_constraints::ConstraintUsingIndex::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ConstraintUsingIndex", "path": "ConstraintUsingIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 47], "end": [565, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
