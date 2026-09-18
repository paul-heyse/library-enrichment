# `sqlparser::ast::table_constraints::PrimaryKeyConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.PrimaryKeyConstraint.json).

<a id="op-a476172bd847c18631c65d5f"></a>
## PrimaryKeyConstraint

`struct` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint` · sqlparser 0.62.0

```rust
struct PrimaryKeyConstraint
```

Source: `src/ast/table_constraints.rs:433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL [definition][1] for `PRIMARY KEY` constraints statements:
* `[CONSTRAINT [<name>]] PRIMARY KEY [index_name] [index_type] (<columns>) <index_options>`

Actually the specification have no `[index_name]` but the next query will complete successfully:
```sql
CREATE TABLE unspec_table (
  xid INT NOT NULL,
  CONSTRAINT p_name PRIMARY KEY index_name USING BTREE (xid)
);
```

where:
* [index_type][2] is `USING {BTREE | HASH}`
* [index_options][3] is `{index_type | COMMENT 'string' | ... %currently unsupported stmts% } ...`

[1]: https://dev.mysql.com/doc/refman/8.3/en/create-table.html
[2]: IndexType
[3]: IndexOption

<a id="op-fa0f88d7c142b59335635d2a"></a>
## characteristics

`struct_field` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::characteristics` · sqlparser 0.62.0

```rust
characteristics: Option<ast::ConstraintCharacteristics>
```

Source: `src/ast/table_constraints.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional characteristics like `DEFERRABLE`.

<a id="op-91ec0cc4b85aa7584a33cd06"></a>
## clone

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> PrimaryKeyConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 17], "end": [430, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32e488c4fd7162bfff1cb9fc"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &PrimaryKeyConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 51], "end": [430, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0950c65772acf04d1c6a70e"></a>
## columns

`struct_field` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::columns` · sqlparser 0.62.0

```rust
columns: Vec<ast::IndexColumn>
```

Source: `src/ast/table_constraints.rs:445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identifiers of the columns that form the primary key.

<a id="op-9f1c3815bc01547451f64fb2"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [431, 49], "end": [431, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:431`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2f1bba8b3e0945d20994719"></a>
## eq

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &PrimaryKeyConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 24], "end": [430, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03e378fbd2771574ec5d679d"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 1], "end": [471, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/table_constraints.rs:453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7462999b434df6c1e874d8c6"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 10], "end": [430, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d69e5d1c5c9216d2268877e4"></a>
## hash

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 56], "end": [430, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0a44bc692966495fd0e12dc"></a>
## index_name

`struct_field` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::index_name` · sqlparser 0.62.0

```rust
index_name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index name

<a id="op-9a7f6108a92954bf62c355b9"></a>
## index_options

`struct_field` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::index_options` · sqlparser 0.62.0

```rust
index_options: Vec<ast::IndexOption>
```

Source: `src/ast/table_constraints.rs:447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional index options such as `USING`.

<a id="op-2ae44a83a8285cd10e6b5fce"></a>
## index_type

`struct_field` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::index_type` · sqlparser 0.62.0

```rust
index_type: Option<ast::IndexType>
```

Source: `src/ast/table_constraints.rs:443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `USING` of [index type][1] statement before columns.

[1]: IndexType

<a id="op-ea1f89c3de84c527eac4447e"></a>
## name

`struct_field` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::name` · sqlparser 0.62.0

```rust
name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Constraint name.

Can be not the same as `index_name`

<a id="op-406afb4939479acd2dbf8ea7"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &PrimaryKeyConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 35], "end": [430, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4b032d0c1332f9adc38d9dc"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [431, 38], "end": [431, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:431`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9be81b44f4049b06d3bbdc7c"></a>
## span

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [473, 1], "end": [488, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/table_constraints.rs:474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-697e2380f1b568b1c7d0e506"></a>
## visit

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 40], "end": [432, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e349f48bc5957ea5db320204"></a>
## visit

`function` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 47], "end": [432, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
