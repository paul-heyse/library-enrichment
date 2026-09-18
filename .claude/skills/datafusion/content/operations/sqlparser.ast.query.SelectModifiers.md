# `sqlparser::ast::query::SelectModifiers`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SelectModifiers.json).

<a id="op-be4f6199799f4061623e358e"></a>
## SelectModifiers

`struct` · `sqlparser::ast::query::SelectModifiers` · sqlparser 0.62.0

```rust
struct SelectModifiers
```

Source: `src/ast/query.rs:357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific SELECT modifiers that appear after the SELECT keyword.

These modifiers affect query execution and optimization. They can appear in any order after
SELECT and before the column list, can be repeated, and can be interleaved with
DISTINCT/DISTINCTROW/ALL:

```sql
SELECT
    [ALL | DISTINCT | DISTINCTROW]
    [HIGH_PRIORITY]
    [STRAIGHT_JOIN]
    [SQL_SMALL_RESULT] [SQL_BIG_RESULT] [SQL_BUFFER_RESULT]
    [SQL_NO_CACHE] [SQL_CALC_FOUND_ROWS]
    select_expr [, select_expr] ...
```

See [MySQL SELECT](https://dev.mysql.com/doc/refman/8.4/en/select.html).

<a id="op-fd7e7f8a64504b946381fb8a"></a>
## clone

`function` · `sqlparser::ast::query::SelectModifiers::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SelectModifiers
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 17], "end": [354, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-269da3a42e92be5e71c1560f"></a>
## cmp

`function` · `sqlparser::ast::query::SelectModifiers::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SelectModifiers) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 51], "end": [354, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88d7e61c935e570028df7d26"></a>
## default

`function` · `sqlparser::ast::query::SelectModifiers::default` · sqlparser 0.62.0

```rust
fn default() -> SelectModifiers
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 62], "end": [354, 69], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/query.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9086079328c1c812837386b4"></a>
## deserialize

`function` · `sqlparser::ast::query::SelectModifiers::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [355, 49], "end": [355, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6cd3b27fbea7dacbbb9f10d"></a>
## eq

`function` · `sqlparser::ast::query::SelectModifiers::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SelectModifiers) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 24], "end": [354, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-369b4ca8f161ab07132a39d2"></a>
## fmt

`function` · `sqlparser::ast::query::SelectModifiers::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 10], "end": [354, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53670a5e85be2049e10ab69e"></a>
## fmt

`function` · `sqlparser::ast::query::SelectModifiers::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [414, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e93ec2568ec5a1cd7ca1797"></a>
## hash

`function` · `sqlparser::ast::query::SelectModifiers::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 56], "end": [354, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fa68bfe63ffc3f8d4c74191"></a>
## high_priority

`struct_field` · `sqlparser::ast::query::SelectModifiers::high_priority` · sqlparser 0.62.0

```rust
high_priority: bool
```

Source: `src/ast/query.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`HIGH_PRIORITY` gives the SELECT higher priority than statements that update a table.

<https://dev.mysql.com/doc/refman/8.4/en/select.html>

<a id="op-5f00fc4983b36b5e130ce3fe"></a>
## is_any_set

`function` · `sqlparser::ast::query::SelectModifiers::is_any_set` · sqlparser 0.62.0

```rust
fn is_any_set(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [437, 2], "filename": "src/ast/query.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/query.rs:418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if any of the modifiers are set.

<a id="op-14000dd5d90cde4bb0fb0e2e"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SelectModifiers::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SelectModifiers) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 35], "end": [354, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23db35ada912a67be98945a1"></a>
## serialize

`function` · `sqlparser::ast::query::SelectModifiers::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [355, 38], "end": [355, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7aff285d2e0040384936f922"></a>
## sql_big_result

`struct_field` · `sqlparser::ast::query::SelectModifiers::sql_big_result` · sqlparser 0.62.0

```rust
sql_big_result: bool
```

Source: `src/ast/query.rs:373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SQL_BIG_RESULT` hints that the result set is large, using disk-based temp tables.

<https://dev.mysql.com/doc/refman/8.4/en/select.html>

<a id="op-ef888c50a7d58fce49195809"></a>
## sql_buffer_result

`struct_field` · `sqlparser::ast::query::SelectModifiers::sql_buffer_result` · sqlparser 0.62.0

```rust
sql_buffer_result: bool
```

Source: `src/ast/query.rs:377`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SQL_BUFFER_RESULT` forces the result to be put into a temporary table to release locks early.

<https://dev.mysql.com/doc/refman/8.4/en/select.html>

<a id="op-afaea8d24d99a403a2e0af49"></a>
## sql_calc_found_rows

`struct_field` · `sqlparser::ast::query::SelectModifiers::sql_calc_found_rows` · sqlparser 0.62.0

```rust
sql_calc_found_rows: bool
```

Source: `src/ast/query.rs:386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SQL_CALC_FOUND_ROWS` tells MySQL to calculate the total number of rows. (Deprecated in 8.0.17+.)

- [MySQL SELECT modifiers](https://dev.mysql.com/doc/refman/8.4/en/select.html)
- [`FOUND_ROWS()`](https://dev.mysql.com/doc/refman/8.4/en/information-functions.html#function_found-rows)

<a id="op-7ca5574c64fe629ddb120050"></a>
## sql_no_cache

`struct_field` · `sqlparser::ast::query::SelectModifiers::sql_no_cache` · sqlparser 0.62.0

```rust
sql_no_cache: bool
```

Source: `src/ast/query.rs:381`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SQL_NO_CACHE` tells MySQL not to cache the query result. (Deprecated in 8.4+.)

<https://dev.mysql.com/doc/refman/8.4/en/select.html>

<a id="op-81ac812a9fdc6c61f5798ccb"></a>
## sql_small_result

`struct_field` · `sqlparser::ast::query::SelectModifiers::sql_small_result` · sqlparser 0.62.0

```rust
sql_small_result: bool
```

Source: `src/ast/query.rs:369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SQL_SMALL_RESULT` hints that the result set is small, using in-memory temp tables.

<https://dev.mysql.com/doc/refman/8.4/en/select.html>

<a id="op-0d660b7ea7825e221b063a38"></a>
## straight_join

`struct_field` · `sqlparser::ast::query::SelectModifiers::straight_join` · sqlparser 0.62.0

```rust
straight_join: bool
```

Source: `src/ast/query.rs:365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`STRAIGHT_JOIN` forces the optimizer to join tables in the order listed in the FROM clause.

<https://dev.mysql.com/doc/refman/8.4/en/select.html>

<a id="op-192ec1fb5b4aeb7682d51f29"></a>
## visit

`function` · `sqlparser::ast::query::SelectModifiers::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 47], "end": [356, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fea4403edbc9a7f59da48f43"></a>
## visit

`function` · `sqlparser::ast::query::SelectModifiers::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectModifiers", "path": "SelectModifiers"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 40], "end": [356, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
