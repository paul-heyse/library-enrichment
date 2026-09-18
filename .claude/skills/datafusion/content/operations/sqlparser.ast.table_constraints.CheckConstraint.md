# `sqlparser::ast::table_constraints::CheckConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.CheckConstraint.json).

<a id="op-c895f28e005040054bb9f9d2"></a>
## CheckConstraint

`struct` · `sqlparser::ast::table_constraints::CheckConstraint` · sqlparser 0.62.0

```rust
struct CheckConstraint
```

Source: `src/ast/table_constraints.rs:177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `CHECK` constraint (`[ CONSTRAINT <name> ] CHECK (<expr>) [[NOT] ENFORCED]`).

<a id="op-164b8a66009e324839cf54ad"></a>
## clone

`function` · `sqlparser::ast::table_constraints::CheckConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CheckConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 17], "end": [173, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47305418776e3cf8d523369a"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::CheckConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CheckConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 51], "end": [173, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94aa3312aa89f2aa96b06812"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::CheckConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 49], "end": [174, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-533778278fb31d43ade90129"></a>
## enforced

`struct_field` · `sqlparser::ast::table_constraints::CheckConstraint::enforced` · sqlparser 0.62.0

```rust
enforced: Option<bool>
```

Source: `src/ast/table_constraints.rs:184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific `ENFORCED` / `NOT ENFORCED` flag.
<https://dev.mysql.com/doc/refman/8.4/en/create-table.html>

<a id="op-d9ee93b92d9b8689710d49d4"></a>
## eq

`function` · `sqlparser::ast::table_constraints::CheckConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CheckConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 24], "end": [173, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38eb9da63f7ead7b6e32d7b9"></a>
## expr

`struct_field` · `sqlparser::ast::table_constraints::CheckConstraint::expr` · sqlparser 0.62.0

```rust
expr: Box<ast::Expr>
```

Source: `src/ast/table_constraints.rs:181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The boolean expression the CHECK constraint enforces.

<a id="op-641b0f4d3d31bffd25ffa5ed"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::CheckConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [202, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/table_constraints.rs:188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f83e505765d311932061d3ce"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::CheckConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 10], "end": [173, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aa9b4b4d2ce0b316c9c9ec3"></a>
## hash

`function` · `sqlparser::ast::table_constraints::CheckConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 56], "end": [173, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-671727d2e10544791ab801fa"></a>
## name

`struct_field` · `sqlparser::ast::table_constraints::CheckConstraint::name` · sqlparser 0.62.0

```rust
name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional constraint name.

<a id="op-b7f3e92ff023c4231b3fe741"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::CheckConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CheckConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 35], "end": [173, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88bd974c44d89f5a8895b326"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::CheckConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 38], "end": [174, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d60e41d68eb948f2d1846470"></a>
## span

`function` · `sqlparser::ast::table_constraints::CheckConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [210, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/table_constraints.rs:205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-545cdc06d73468285d9dec28"></a>
## visit

`function` · `sqlparser::ast::table_constraints::CheckConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 40], "end": [175, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d5d536a1e99595f90a9810d"></a>
## visit

`function` · `sqlparser::ast::table_constraints::CheckConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 47], "end": [175, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
