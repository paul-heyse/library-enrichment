# `sqlparser::ast::table_constraints::ForeignKeyConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.ForeignKeyConstraint.json).

<a id="op-8239c0aefe9acdb739d8cc2c"></a>
## ForeignKeyConstraint

`struct` · `sqlparser::ast::table_constraints::ForeignKeyConstraint` · sqlparser 0.62.0

```rust
struct ForeignKeyConstraint
```

Source: `src/ast/table_constraints.rs:220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
REFERENCES <foreign_table> (<referred_columns>) [ MATCH { FULL | PARTIAL | SIMPLE } ]
{ [ON DELETE <referential_action>] [ON UPDATE <referential_action>] |
  [ON UPDATE <referential_action>] [ON DELETE <referential_action>]
}`).

<a id="op-1fc47d066d692ced05f250a2"></a>
## characteristics

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::characteristics` · sqlparser 0.62.0

```rust
characteristics: Option<ast::ConstraintCharacteristics>
```

Source: `src/ast/table_constraints.rs:239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional characteristics (e.g., `DEFERRABLE`).

<a id="op-af862697a4e9497f54e95013"></a>
## clone

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ForeignKeyConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 17], "end": [217, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-023708163652189ef15b9bb7"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ForeignKeyConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 51], "end": [217, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c5d748cd631f7837ab7089b"></a>
## columns

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::columns` · sqlparser 0.62.0

```rust
columns: Vec<ast::Ident>
```

Source: `src/ast/table_constraints.rs:227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns in the local table that participate in the foreign key.

<a id="op-bb15c3617430b261b7cbaebf"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 49], "end": [218, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3830fd86a8be837112b264f3"></a>
## eq

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ForeignKeyConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 24], "end": [217, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ac8e093764c9efa21e403e4"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [270, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/table_constraints.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5de19cf8d2273abc87ba206c"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 10], "end": [217, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ef830d0bb87b79156694e3c"></a>
## foreign_table

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::foreign_table` · sqlparser 0.62.0

```rust
foreign_table: ast::ObjectName
```

Source: `src/ast/table_constraints.rs:229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Referenced foreign table name.

<a id="op-36f8d87392eadeef8abf1edb"></a>
## hash

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 56], "end": [217, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2426ab2d44461d94032d10e4"></a>
## index_name

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::index_name` · sqlparser 0.62.0

```rust
index_name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific index name associated with the foreign key.
<https://dev.mysql.com/doc/refman/8.4/en/create-table-foreign-keys.html>

<a id="op-bfcb9602d3fd848fe71ac563"></a>
## match_kind

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::match_kind` · sqlparser 0.62.0

```rust
match_kind: Option<ast::ConstraintReferenceMatchKind>
```

Source: `src/ast/table_constraints.rs:237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `MATCH` kind (FULL | PARTIAL | SIMPLE).

<a id="op-109c5d72546f5b0c14631063"></a>
## name

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::name` · sqlparser 0.62.0

```rust
name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:222`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional constraint name.

<a id="op-33c557dfc63e56a1a0e7b0ef"></a>
## on_delete

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::on_delete` · sqlparser 0.62.0

```rust
on_delete: Option<ast::ReferentialAction>
```

Source: `src/ast/table_constraints.rs:233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Action to perform `ON DELETE`.

<a id="op-0997f85eef3b8eb72b4d05ce"></a>
## on_update

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::on_update` · sqlparser 0.62.0

```rust
on_update: Option<ast::ReferentialAction>
```

Source: `src/ast/table_constraints.rs:235`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Action to perform `ON UPDATE`.

<a id="op-6df45b4338b0bd2c75d43378"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ForeignKeyConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 35], "end": [217, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-032d1f780584eca600921566"></a>
## referred_columns

`struct_field` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::referred_columns` · sqlparser 0.62.0

```rust
referred_columns: Vec<ast::Ident>
```

Source: `src/ast/table_constraints.rs:231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns in the referenced table.

<a id="op-d1d954c03541d3c754d5966f"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 38], "end": [218, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d00517b8abfc1078474bcbbe"></a>
## span

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 1], "end": [291, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/table_constraints.rs:273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87b8549b00b9e3484f019a7a"></a>
## visit

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 47], "end": [219, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a2b9345bbe4f780ee976bd0"></a>
## visit

`function` · `sqlparser::ast::table_constraints::ForeignKeyConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 40], "end": [219, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
