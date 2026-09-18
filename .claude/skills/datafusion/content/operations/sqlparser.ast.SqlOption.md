# `sqlparser::ast::SqlOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SqlOption.json).

<a id="op-c32f15c09b48ce1fa7b1af0d"></a>
## SqlOption

`enum` · `sqlparser::ast::SqlOption` · sqlparser 0.62.0

```rust
enum SqlOption
```

Source: `src/ast/mod.rs:8796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL option syntax used in table and server definitions.

<a id="op-8ecdbb7b67217a4126bd4d61"></a>
## Clustered

`variant` · `sqlparser::ast::SqlOption::Clustered` · sqlparser 0.62.0

```rust
Clustered
```

Source: `src/ast/mod.rs:8800`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Clustered represents the clustered version of table storage for MSSQL.

<https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-azure-sql-data-warehouse?view=aps-pdw-2016-au7#TableOptions>

<a id="op-d1cdb97708f8298c4408537e"></a>
## Comment

`variant` · `sqlparser::ast::SqlOption::Comment` · sqlparser 0.62.0

```rust
Comment
```

Source: `src/ast/mod.rs:8829`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Comment parameter (supports `=` and no `=` syntax)

<a id="op-b06cc82a08805db42f9db304"></a>
## Ident

`variant` · `sqlparser::ast::SqlOption::Ident` · sqlparser 0.62.0

```rust
Ident
```

Source: `src/ast/mod.rs:8804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single identifier options, e.g. `HEAP` for MSSQL.

<https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-azure-sql-data-warehouse?view=aps-pdw-2016-au7#TableOptions>

<a id="op-33ef76a6f873f11a8077ffa5"></a>
## KeyValue

`variant` · `sqlparser::ast::SqlOption::KeyValue` · sqlparser 0.62.0

```rust
KeyValue
```

Source: `src/ast/mod.rs:8808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Any option that consists of a key value pair where the value is an expression. e.g.

  WITH(DISTRIBUTION = ROUND_ROBIN)

<a id="op-3419573cb208eaf6b854d789"></a>
## NamedParenthesizedList

`variant` · `sqlparser::ast::SqlOption::NamedParenthesizedList` · sqlparser 0.62.0

```rust
NamedParenthesizedList
```

Source: `src/ast/mod.rs:8839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An option representing a key value pair, where the value is a parenthesized list and with an optional name
e.g.

  UNION  = (tbl_name\[,tbl_name\]...) <https://dev.mysql.com/doc/refman/8.4/en/create-table.html>
  ENGINE = ReplicatedMergeTree('/table_name','{replica}', ver) <https://clickhouse.com/docs/engines/table-engines/mergetree-family/replication>
  ENGINE = SummingMergeTree(\[columns\]) <https://clickhouse.com/docs/engines/table-engines/mergetree-family/summingmergetree>

<a id="op-bbffc4abb0989a435f99adff"></a>
## Partition

`variant` · `sqlparser::ast::SqlOption::Partition` · sqlparser 0.62.0

```rust
Partition
```

Source: `src/ast/mod.rs:8820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more table partitions and represents which partition the boundary values belong to,
e.g.

  PARTITION (id RANGE LEFT FOR VALUES (10, 20, 30, 40))

<https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-azure-sql-data-warehouse?view=aps-pdw-2016-au7#TablePartitionOptions>

<a id="op-a682b36a13251049a721fd86"></a>
## TableSpace

`variant` · `sqlparser::ast::SqlOption::TableSpace` · sqlparser 0.62.0

```rust
TableSpace
```

Source: `src/ast/mod.rs:8832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL TableSpace option
<https://dev.mysql.com/doc/refman/8.4/en/create-table.html>

<a id="op-76fb12e13143069010b83d33"></a>
## clone

`function` · `sqlparser::ast::SqlOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SqlOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8792, 17], "end": [8792, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dbf9e541a19c62ef38f7e58"></a>
## cmp

`function` · `sqlparser::ast::SqlOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SqlOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8792, 51], "end": [8792, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f6a87d7f6e5c09b3e57d400"></a>
## deserialize

`function` · `sqlparser::ast::SqlOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8793, 49], "end": [8793, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8793`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b74877ee62940ccd6c1302d"></a>
## eq

`function` · `sqlparser::ast::SqlOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SqlOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8792, 24], "end": [8792, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53799b19851229a1a4a6468a"></a>
## fmt

`function` · `sqlparser::ast::SqlOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8842, 1], "end": [8899, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8843`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cb75f6bdefe15a44aedf4a5"></a>
## fmt

`function` · `sqlparser::ast::SqlOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8792, 10], "end": [8792, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a72915340fca7732e6cfee4"></a>
## hash

`function` · `sqlparser::ast::SqlOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8792, 56], "end": [8792, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c38feafa8a1cb7855d3e1400"></a>
## partial_cmp

`function` · `sqlparser::ast::SqlOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SqlOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8792, 35], "end": [8792, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9a4dee91b3c0db550e94d09"></a>
## serialize

`function` · `sqlparser::ast::SqlOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8793, 38], "end": [8793, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8793`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed3a53e66dd075b1f7af1c94"></a>
## span

`function` · `sqlparser::ast::SqlOption::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "super::SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1025, 1], "end": [1048, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1026`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9013bb511bf7e986580d03fc"></a>
## visit

`function` · `sqlparser::ast::SqlOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8794, 47], "end": [8794, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1595a24e1bc0ca5ae33d703"></a>
## visit

`function` · `sqlparser::ast::SqlOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SqlOption", "path": "SqlOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8794, 40], "end": [8794, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
