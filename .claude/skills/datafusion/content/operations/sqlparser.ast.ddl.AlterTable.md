# `sqlparser::ast::ddl::AlterTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTable.json).

<a id="op-c6964c8b2595def5b9a107b3"></a>
## AlterTable

`struct` · `sqlparser::ast::ddl::AlterTable` · sqlparser 0.62.0

```rust
struct AlterTable
```

Source: `src/ast/ddl.rs:4664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ALTER TABLE statement

<a id="op-7cc56f094c2d6ba77f8b1f8e"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTable::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4661, 17], "end": [4661, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c48908904dd45208c23ced93"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTable::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTable) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4661, 51], "end": [4661, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc3cc22407a8875b97319df4"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTable::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4662, 49], "end": [4662, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04f8c515de4cc502e2ba7305"></a>
## end_token

`struct_field` · `sqlparser::ast::ddl::AlterTable::end_token` · sqlparser 0.62.0

```rust
end_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/ddl.rs:4683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token that represents the end of the statement (semicolon or EOF)

<a id="op-76acf68691f7ce96292071dc"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTable::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4661, 24], "end": [4661, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-370fb5bd536c76920679434c"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4686, 1], "end": [4711, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4687`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6f1853b527dc33e23cfe833"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4661, 10], "end": [4661, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9e70ec0f21701e8b249a5ec"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTable::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4661, 56], "end": [4661, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d1884c3bb89eacca130d7cc"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterTable::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:4669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified for the `ALTER TABLE`.

<a id="op-163268f93049772647397ced"></a>
## location

`struct_field` · `sqlparser::ast::ddl::AlterTable::location` · sqlparser 0.62.0

```rust
location: Option<ast::HiveSetLocation>
```

Source: `src/ast/ddl.rs:4675`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional Hive `SET LOCATION` clause for the alter operation.

<a id="op-2dfed2c3492512259ad42420"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterTable::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table name

<a id="op-c2f85244126060e0639a6676"></a>
## on_cluster

`struct_field` · `sqlparser::ast::ddl::AlterTable::on_cluster` · sqlparser 0.62.0

```rust
on_cluster: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:4679`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse dialect supports `ON CLUSTER` clause for ALTER TABLE
For example: `ALTER TABLE table_name ON CLUSTER cluster_name ADD COLUMN c UInt32`
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/update)

<a id="op-853f7b09c4101f888069a6c3"></a>
## only

`struct_field` · `sqlparser::ast::ddl::AlterTable::only` · sqlparser 0.62.0

```rust
only: bool
```

Source: `src/ast/ddl.rs:4671`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `ONLY` keyword was used (restrict scope to the named table).

<a id="op-9497516f67cb34a4212e1bcd"></a>
## operations

`struct_field` · `sqlparser::ast::ddl::AlterTable::operations` · sqlparser 0.62.0

```rust
operations: Vec<AlterTableOperation>
```

Source: `src/ast/ddl.rs:4673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of `ALTER TABLE` operations to apply.

<a id="op-2cf39c024e85eaca59916b1f"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTable::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4661, 35], "end": [4661, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10a8c834be31a8fe42475bf5"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTable::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4662, 38], "end": [4662, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df787c0a39dee152f5bb7ec8"></a>
## span

`function` · `sqlparser::ast::ddl::AlterTable::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "crate::ast::AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2484, 1], "end": [2493, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8c0feae65dec9e1a588481f"></a>
## table_type

`struct_field` · `sqlparser::ast::ddl::AlterTable::table_type` · sqlparser 0.62.0

```rust
table_type: Option<AlterTableType>
```

Source: `src/ast/ddl.rs:4681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table type: None for regular tables, Some(AlterTableType) for Iceberg or Dynamic tables

<a id="op-178b07aad2d157a7468614f5"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4663, 40], "end": [4663, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f782f0555c85470d9cfbf418"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4663, 47], "end": [4663, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
