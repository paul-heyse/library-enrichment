# `sqlparser::ast::query::Table`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Table.json).

<a id="op-d53115de167527d6b6bc726b"></a>
## Table

`struct` · `sqlparser::ast::query::Table` · sqlparser 0.62.0

```rust
struct Table
```

Source: `src/ast/query.rs:301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`TABLE` command]( https://www.postgresql.org/docs/current/sql-select.html#SQL-TABLE)
A (possibly schema-qualified) table reference used in `FROM` clauses.

<a id="op-9abf827f4328a78cce83283b"></a>
## clone

`function` · `sqlparser::ast::query::Table::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Table
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 17], "end": [296, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c94b2039b3d94aceb1e5ad8"></a>
## cmp

`function` · `sqlparser::ast::query::Table::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Table) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 51], "end": [296, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07ff2453b9e202017e339781"></a>
## deserialize

`function` · `sqlparser::ast::query::Table::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 49], "end": [297, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b87b55b742d5486a7869489"></a>
## eq

`function` · `sqlparser::ast::query::Table::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Table) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 24], "end": [296, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-184c5d3d584c6ed6f5ff93db"></a>
## fmt

`function` · `sqlparser::ast::query::Table::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [322, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54b646eab3058cf8bf11ce88"></a>
## fmt

`function` · `sqlparser::ast::query::Table::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 10], "end": [296, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-391b6bf78d7110c9a7ca4a44"></a>
## hash

`function` · `sqlparser::ast::query::Table::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 56], "end": [296, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-296cb410f80051786db2c250"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Table::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Table) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 35], "end": [296, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d9c3cbd3b35e2fe62ecce06"></a>
## schema_name

`struct_field` · `sqlparser::ast::query::Table::schema_name` · sqlparser 0.62.0

```rust
schema_name: Option<String>
```

Source: `src/ast/query.rs:305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional schema/catalog name qualifying the table.

<a id="op-067003c2bf2060e06a694a58"></a>
## serialize

`function` · `sqlparser::ast::query::Table::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 38], "end": [297, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fb7086d39ac260f7f91c5d5"></a>
## table_name

`struct_field` · `sqlparser::ast::query::Table::table_name` · sqlparser 0.62.0

```rust
table_name: Option<String>
```

Source: `src/ast/query.rs:303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional table name (absent for e.g. `TABLE` command without argument).

<a id="op-01697ddee274ae3d524bfde5"></a>
## visit

`function` · `sqlparser::ast::query::Table::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 47], "end": [299, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0a256e167f498ed939e3a0d"></a>
## visit

`function` · `sqlparser::ast::query::Table::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 40], "end": [299, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
