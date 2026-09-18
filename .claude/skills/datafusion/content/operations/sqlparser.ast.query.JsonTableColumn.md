# `sqlparser::ast::query::JsonTableColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.JsonTableColumn.json).

<a id="op-8dfe97833789f2b2d22e4b7d"></a>
## JsonTableColumn

`enum` · `sqlparser::ast::query::JsonTableColumn` · sqlparser 0.62.0

```rust
enum JsonTableColumn
```

Source: `src/ast/query.rs:3995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single column definition in MySQL's `JSON_TABLE` table valued function.

See
- [MySQL's JSON_TABLE documentation](https://dev.mysql.com/doc/refman/8.0/en/json-table-functions.html#function_json-table)
- [Oracle's JSON_TABLE documentation](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/JSON_TABLE.html)
- [MariaDB's JSON_TABLE documentation](https://mariadb.com/kb/en/json_table/)

```sql
SELECT *
FROM JSON_TABLE(
    '["a", "b"]',
    '$[*]' COLUMNS (
        name FOR ORDINALITY,
        value VARCHAR(20) PATH '$',
        NESTED PATH '$[*]' COLUMNS (
            value VARCHAR(20) PATH '$'
        )
    )
) AS jt;
```

<a id="op-b118638b006f69849e6903d1"></a>
## ForOrdinality

`variant` · `sqlparser::ast::query::JsonTableColumn::ForOrdinality` · sqlparser 0.62.0

```rust
ForOrdinality
```

Source: `src/ast/query.rs:3999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The FOR ORDINALITY column, which is a special column that returns the index of the current row in a JSON array.

<a id="op-3ac71ca5f5bfd78164e576c2"></a>
## Named

`variant` · `sqlparser::ast::query::JsonTableColumn::Named` · sqlparser 0.62.0

```rust
Named
```

Source: `src/ast/query.rs:3997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A named column with a JSON path

<a id="op-25231e6cb38b089323eaaae6"></a>
## Nested

`variant` · `sqlparser::ast::query::JsonTableColumn::Nested` · sqlparser 0.62.0

```rust
Nested
```

Source: `src/ast/query.rs:4001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A set of nested columns, which extracts data from a nested JSON array.

<a id="op-8cfaee832c0f22a8f43d64fc"></a>
## clone

`function` · `sqlparser::ast::query::JsonTableColumn::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonTableColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3992, 17], "end": [3992, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3992`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9f3f303bd6244e823f5a972"></a>
## cmp

`function` · `sqlparser::ast::query::JsonTableColumn::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonTableColumn) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3992, 51], "end": [3992, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3992`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cfc355b1b2f997216024362"></a>
## deserialize

`function` · `sqlparser::ast::query::JsonTableColumn::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3994, 49], "end": [3994, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f127cedf3c2faa549a85c042"></a>
## eq

`function` · `sqlparser::ast::query::JsonTableColumn::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonTableColumn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3992, 24], "end": [3992, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3992`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d9b1950a36fc69763b2012a"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4004, 1], "end": [4016, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90b2168882aa3d292b1e468b"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3992, 10], "end": [3992, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3992`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-741e6e42d6c15e9c4e351f0e"></a>
## hash

`function` · `sqlparser::ast::query::JsonTableColumn::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3992, 56], "end": [3992, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3992`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27d827e82499fdc0e6ce74db"></a>
## partial_cmp

`function` · `sqlparser::ast::query::JsonTableColumn::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonTableColumn) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3992, 35], "end": [3992, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3992`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-015929a6ac2273cddd103057"></a>
## serialize

`function` · `sqlparser::ast::query::JsonTableColumn::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3994, 38], "end": [3994, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7115e77ddd84f8d68764b52c"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3993, 47], "end": [3993, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2229b3645c8cade1f188236"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableColumn", "path": "JsonTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3993, 40], "end": [3993, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
