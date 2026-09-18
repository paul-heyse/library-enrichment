# `sqlparser::ast::UtilityOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.UtilityOption.json).

<a id="op-7bfbbf6b89706e1b0e9e7b9b"></a>
## UtilityOption

`struct` · `sqlparser::ast::UtilityOption` · sqlparser 0.62.0

```rust
struct UtilityOption
```

Source: `src/ast/mod.rs:10754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a single PostgreSQL utility option.

A utility option is a key-value pair where the key is an identifier (IDENT) and the value
can be one of the following:
- A number with an optional sign (`+` or `-`). Example: `+10`, `-10.2`, `3`
- A non-keyword string. Example: `option1`, `'option2'`, `"option3"`
- keyword: `TRUE`, `FALSE`, `ON` (`off` is also accept).
- Empty. Example: `ANALYZE` (identifier only)

Utility options are used in various PostgreSQL DDL statements, including statements such as
`CLUSTER`, `EXPLAIN`, `VACUUM`, and `REINDEX`. These statements format options as `( option [, ...] )`.

[CLUSTER](https://www.postgresql.org/docs/current/sql-cluster.html)
[EXPLAIN](https://www.postgresql.org/docs/current/sql-explain.html)
[VACUUM](https://www.postgresql.org/docs/current/sql-vacuum.html)
[REINDEX](https://www.postgresql.org/docs/current/sql-reindex.html)

For example, the `EXPLAIN` AND `VACUUM` statements with options might look like this:
```sql
EXPLAIN (ANALYZE, VERBOSE TRUE, FORMAT TEXT) SELECT * FROM my_table;

VACUUM (VERBOSE, ANALYZE ON, PARALLEL 10) my_table;
```

<a id="op-5394fa98486998a4c99d717c"></a>
## arg

`struct_field` · `sqlparser::ast::UtilityOption::arg` · sqlparser 0.62.0

```rust
arg: Option<Expr>
```

Source: `src/ast/mod.rs:10758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional argument for the option (number, string, keyword, etc.).

<a id="op-0009e23fbae94516c25bb4bc"></a>
## clone

`function` · `sqlparser::ast::UtilityOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UtilityOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10751, 17], "end": [10751, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-424cb94916b43317985413ee"></a>
## cmp

`function` · `sqlparser::ast::UtilityOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UtilityOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10751, 51], "end": [10751, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-446f778a296a4d7abeaca35f"></a>
## deserialize

`function` · `sqlparser::ast::UtilityOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10752, 49], "end": [10752, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10752`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afc451a784e3ebcf71f80292"></a>
## eq

`function` · `sqlparser::ast::UtilityOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UtilityOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10751, 24], "end": [10751, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09e8640edb06619fe961e81c"></a>
## fmt

`function` · `sqlparser::ast::UtilityOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10751, 10], "end": [10751, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9fdf8bb58a4272e52d5d2ad"></a>
## fmt

`function` · `sqlparser::ast::UtilityOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10761, 1], "end": [10769, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15371d2d0bc0e41762e15b24"></a>
## hash

`function` · `sqlparser::ast::UtilityOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10751, 56], "end": [10751, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cd13e34f07f1af34c9f9e67"></a>
## name

`struct_field` · `sqlparser::ast::UtilityOption::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:10756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The option name (identifier).

<a id="op-0772e9f48687f61bad22e871"></a>
## partial_cmp

`function` · `sqlparser::ast::UtilityOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UtilityOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10751, 35], "end": [10751, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-682c12d1bdda9f0dfe7163e6"></a>
## serialize

`function` · `sqlparser::ast::UtilityOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10752, 38], "end": [10752, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10752`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7934337c99e12f39ee78e26"></a>
## visit

`function` · `sqlparser::ast::UtilityOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10753, 40], "end": [10753, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d836c67998f9af9e8360593c"></a>
## visit

`function` · `sqlparser::ast::UtilityOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UtilityOption", "path": "UtilityOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10753, 47], "end": [10753, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
