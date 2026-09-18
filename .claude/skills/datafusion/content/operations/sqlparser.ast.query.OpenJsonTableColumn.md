# `sqlparser::ast::query::OpenJsonTableColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.OpenJsonTableColumn.json).

<a id="op-28ea55f1d0fca766ffb2109c"></a>
## OpenJsonTableColumn

`struct` · `sqlparser::ast::query::OpenJsonTableColumn` · sqlparser 0.62.0

```rust
struct OpenJsonTableColumn
```

Source: `src/ast/query.rs:4125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single column definition in MSSQL's `OPENJSON WITH` clause.

```sql
colName type [ column_path ] [ AS JSON ]
```

Reference: <https://learn.microsoft.com/en-us/sql/t-sql/functions/openjson-transact-sql?view=sql-server-ver16#syntax>

<a id="op-8296534b4b13d82ab38e937b"></a>
## as_json

`struct_field` · `sqlparser::ast::query::OpenJsonTableColumn::as_json` · sqlparser 0.62.0

```rust
as_json: bool
```

Source: `src/ast/query.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `AS JSON` option.

<a id="op-e9b49c8c36b2fab467a1d4ec"></a>
## clone

`function` · `sqlparser::ast::query::OpenJsonTableColumn::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OpenJsonTableColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4122, 17], "end": [4122, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72b2f90ca1db728115c0fa29"></a>
## cmp

`function` · `sqlparser::ast::query::OpenJsonTableColumn::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OpenJsonTableColumn) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4122, 51], "end": [4122, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b322ed3be1e3dfe7c90e88b9"></a>
## deserialize

`function` · `sqlparser::ast::query::OpenJsonTableColumn::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4124, 49], "end": [4124, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32b2947880ba50ed4744c59d"></a>
## eq

`function` · `sqlparser::ast::query::OpenJsonTableColumn::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OpenJsonTableColumn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4122, 24], "end": [4122, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b1fe038e394dbb02bbf446c"></a>
## fmt

`function` · `sqlparser::ast::query::OpenJsonTableColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4122, 10], "end": [4122, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe8db2b389de7a29e74c2e98"></a>
## fmt

`function` · `sqlparser::ast::query::OpenJsonTableColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4136, 1], "end": [4147, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7adaccd23a56d7238b1df865"></a>
## hash

`function` · `sqlparser::ast::query::OpenJsonTableColumn::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4122, 56], "end": [4122, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ddd62977166278bb172ed8b"></a>
## name

`struct_field` · `sqlparser::ast::query::OpenJsonTableColumn::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/query.rs:4127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the column to be extracted.

<a id="op-9c8c7b2fb24ae828dfc0bbc7"></a>
## partial_cmp

`function` · `sqlparser::ast::query::OpenJsonTableColumn::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OpenJsonTableColumn) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4122, 35], "end": [4122, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02497bfefea76fe078a4dc9e"></a>
## path

`struct_field` · `sqlparser::ast::query::OpenJsonTableColumn::path` · sqlparser 0.62.0

```rust
path: Option<String>
```

Source: `src/ast/query.rs:4131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The path to the column to be extracted. Must be a literal string.

<a id="op-8c46c2469edb23486083c84b"></a>
## serialize

`function` · `sqlparser::ast::query::OpenJsonTableColumn::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4124, 38], "end": [4124, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ebe92d7c4849bd8877e01ff"></a>
## type

`struct_field` · `sqlparser::ast::query::OpenJsonTableColumn::type` · sqlparser 0.62.0

```rust
type: DataType
```

Source: `src/ast/query.rs:4129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of the column to be extracted.

<a id="op-86722cfb5241c2fca1bc188e"></a>
## visit

`function` · `sqlparser::ast::query::OpenJsonTableColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4123, 40], "end": [4123, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0cbb30da24e50cd59f320e4"></a>
## visit

`function` · `sqlparser::ast::query::OpenJsonTableColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OpenJsonTableColumn", "path": "OpenJsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4123, 47], "end": [4123, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
