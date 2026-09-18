# `sqlparser::ast::TableObject`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TableObject.json).

<a id="op-6214531cdff0d86c95d2a33f"></a>
## TableObject

`enum` · `sqlparser::ast::TableObject` · sqlparser 0.62.0

```rust
enum TableObject
```

Source: `src/ast/mod.rs:11000`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the referenced table in an `INSERT INTO` statement

<a id="op-78fcb200f9831af0d0407b28"></a>
## TableFunction

`variant` · `sqlparser::ast::TableObject::TableFunction` · sqlparser 0.62.0

```rust
TableFunction
```

Source: `src/ast/mod.rs:11014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table specified as a function.
Example:
```sql
INSERT INTO TABLE FUNCTION remote('localhost', default.simple_table)
```
[Clickhouse](https://clickhouse.com/docs/en/sql-reference/table-functions)

<a id="op-ecc1ee239f8d1a9c388bb79e"></a>
## TableName

`variant` · `sqlparser::ast::TableObject::TableName` · sqlparser 0.62.0

```rust
TableName
```

Source: `src/ast/mod.rs:11006`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table specified by name.
Example:
```sql
INSERT INTO my_table
```

<a id="op-ca43969a26cfa94724ffbd94"></a>
## TableQuery

`variant` · `sqlparser::ast::TableObject::TableQuery` · sqlparser 0.62.0

```rust
TableQuery
```

Source: `src/ast/mod.rs:11024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table specified through a sub-query
Example:
```sql
INSERT INTO
(SELECT employee_id, last_name, email, hire_date, job_id,  salary, commission_pct FROM employees)
VALUES (207, 'Gregory', 'pgregory@example.com', sysdate, 'PU_CLERK', 1.2E3, NULL);
```
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/INSERT.html#GUID-903F8043-0254-4EE9-ACC1-CB8AC0AF3423__I2126242)

<a id="op-45a705dbf331fd461daf031a"></a>
## clone

`function` · `sqlparser::ast::TableObject::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableObject
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10997, 17], "end": [10997, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-386fde0c8372d9579508ec14"></a>
## cmp

`function` · `sqlparser::ast::TableObject::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableObject) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10997, 51], "end": [10997, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b927151aa25e2875cf86d31f"></a>
## deserialize

`function` · `sqlparser::ast::TableObject::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10998, 49], "end": [10998, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10ae7d5653391e8c69171cc9"></a>
## eq

`function` · `sqlparser::ast::TableObject::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableObject) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10997, 24], "end": [10997, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2f26f196e71b120b0f4570c"></a>
## fmt

`function` · `sqlparser::ast::TableObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11027, 1], "end": [11035, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecf63078b8f1e6565ab5fd8b"></a>
## fmt

`function` · `sqlparser::ast::TableObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10997, 10], "end": [10997, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06ab2eec99b2984ecb4dbb2b"></a>
## hash

`function` · `sqlparser::ast::TableObject::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10997, 56], "end": [10997, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51c479e907570feb16e56421"></a>
## partial_cmp

`function` · `sqlparser::ast::TableObject::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableObject) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10997, 35], "end": [10997, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2b698b749300fa2b936202e"></a>
## serialize

`function` · `sqlparser::ast::TableObject::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10998, 38], "end": [10998, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfe9d57c6cdab16e24b5368d"></a>
## span

`function` · `sqlparser::ast::TableObject::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "super::TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2405, 1], "end": [2415, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2406`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d618b72c3b587e8ee1438be4"></a>
## visit

`function` · `sqlparser::ast::TableObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10999, 47], "end": [10999, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eee2d911e9c25cdbdff7381e"></a>
## visit

`function` · `sqlparser::ast::TableObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableObject", "path": "TableObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10999, 40], "end": [10999, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
