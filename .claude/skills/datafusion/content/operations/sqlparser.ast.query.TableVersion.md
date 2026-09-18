# `sqlparser::ast::query::TableVersion`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableVersion.json).

<a id="op-c8a52ed166ef9eae8a085a4d"></a>
## TableVersion

`enum` · `sqlparser::ast::query::TableVersion` · sqlparser 0.62.0

```rust
enum TableVersion
```

Source: `src/ast/query.rs:2596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies a table version selection, e.g. `FOR SYSTEM_TIME AS OF` or `AT(...)`.

<a id="op-68007ac0f87255a3a9601d84"></a>
## Changes

`variant` · `sqlparser::ast::query::TableVersion::Changes` · sqlparser 0.62.0

```rust
Changes
```

Source: `src/ast/query.rs:2620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `CHANGES` clause for change tracking queries.
For example:
```sql
SELECT * FROM t
  CHANGES(INFORMATION => DEFAULT)
  AT(TIMESTAMP => TO_TIMESTAMP_TZ('...'))
  END(TIMESTAMP => TO_TIMESTAMP_TZ('...'))
```
<https://docs.snowflake.com/en/sql-reference/constructs/changes>

<a id="op-4f73065eac509baba668d9c4"></a>
## ForSystemTimeAsOf

`variant` · `sqlparser::ast::query::TableVersion::ForSystemTimeAsOf` · sqlparser 0.62.0

```rust
ForSystemTimeAsOf
```

Source: `src/ast/query.rs:2599`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

When the table version is defined using `FOR SYSTEM_TIME AS OF`.
For example: `SELECT * FROM tbl FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 1 HOUR)`

<a id="op-effe927d2ff60e11f1f91dcb"></a>
## Function

`variant` · `sqlparser::ast::query::TableVersion::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/query.rs:2610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

When the table version is defined using a function.
For example: `SELECT * FROM tbl AT(TIMESTAMP => '2020-08-14 09:30:00')`

<a id="op-ffb0b2764d9595d9667eef66"></a>
## TimestampAsOf

`variant` · `sqlparser::ast::query::TableVersion::TimestampAsOf` · sqlparser 0.62.0

```rust
TimestampAsOf
```

Source: `src/ast/query.rs:2603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

When the table version is defined using `TIMESTAMP AS OF`.
Databricks supports this syntax.
For example: `SELECT * FROM tbl TIMESTAMP AS OF CURRENT_TIMESTAMP() - INTERVAL 1 HOUR`

<a id="op-3932bb37a5fd95276cbadf7c"></a>
## VersionAsOf

`variant` · `sqlparser::ast::query::TableVersion::VersionAsOf` · sqlparser 0.62.0

```rust
VersionAsOf
```

Source: `src/ast/query.rs:2607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

When the table version is defined using `VERSION AS OF`.
Databricks supports this syntax.
For example: `SELECT * FROM tbl VERSION AS OF 2`

<a id="op-f7d4d3a21d0a98d75251230e"></a>
## clone

`function` · `sqlparser::ast::query::TableVersion::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableVersion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2592, 17], "end": [2592, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afd79b98c2613b1a789b0b3a"></a>
## cmp

`function` · `sqlparser::ast::query::TableVersion::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableVersion) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2592, 51], "end": [2592, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e97e2884880cad1c8b4d3423"></a>
## deserialize

`function` · `sqlparser::ast::query::TableVersion::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2593, 49], "end": [2593, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5a887f24cf5166b39282a4e"></a>
## eq

`function` · `sqlparser::ast::query::TableVersion::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableVersion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2592, 24], "end": [2592, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cbc648e7b9c9c21cb4625dc"></a>
## fmt

`function` · `sqlparser::ast::query::TableVersion::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2630, 1], "end": [2646, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2631`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6df02784c9222a5b1760b9da"></a>
## fmt

`function` · `sqlparser::ast::query::TableVersion::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2592, 10], "end": [2592, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91e0b94f4402f83289f0cb72"></a>
## hash

`function` · `sqlparser::ast::query::TableVersion::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2592, 56], "end": [2592, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48743100dad02d7ed1f967df"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableVersion::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableVersion) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2592, 35], "end": [2592, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c99c405f1e9d46fc33d3060"></a>
## serialize

`function` · `sqlparser::ast::query::TableVersion::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2593, 38], "end": [2593, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-048d299658fb428f10958fd6"></a>
## visit

`function` · `sqlparser::ast::query::TableVersion::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2594, 40], "end": [2594, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2594`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab06a410f6edfeb834234738"></a>
## visit

`function` · `sqlparser::ast::query::TableVersion::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableVersion", "path": "TableVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2594, 47], "end": [2594, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2594`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
