# `sqlparser::ast::TypedString`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TypedString.json).

<a id="op-50f41a81ea1482ae8c58c7ae"></a>
## TypedString

`struct` · `sqlparser::ast::TypedString` · sqlparser 0.62.0

```rust
struct TypedString
```

Source: `src/ast/mod.rs:8004`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A constant of form `<data_type> 'value'`.
This can represent ANSI SQL `DATE`, `TIME`, and `TIMESTAMP` literals (such as `DATE '2020-01-01'`),
as well as constants of other types (a non-standard PostgreSQL extension).

<a id="op-78027658ed54e989d826da04"></a>
## clone

`function` · `sqlparser::ast::TypedString::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TypedString
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8001, 17], "end": [8001, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb9cc5c9290e26d35eaf952d"></a>
## cmp

`function` · `sqlparser::ast::TypedString::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TypedString) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8001, 51], "end": [8001, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4be727a0a93328e49387b529"></a>
## data_type

`struct_field` · `sqlparser::ast::TypedString::data_type` · sqlparser 0.62.0

```rust
data_type: DataType
```

Source: `src/ast/mod.rs:8006`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The data type of the typed string (e.g. DATE, TIME, TIMESTAMP).

<a id="op-ab2111f302ec69a68488b2c6"></a>
## deserialize

`function` · `sqlparser::ast::TypedString::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8002, 49], "end": [8002, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ed6785aa3637959076c157e"></a>
## eq

`function` · `sqlparser::ast::TypedString::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TypedString) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8001, 24], "end": [8001, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09bb2f8c91b0dbb450ed03bb"></a>
## fmt

`function` · `sqlparser::ast::TypedString::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8023, 1], "end": [8043, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cec543261f97bfd1105b2b9d"></a>
## fmt

`function` · `sqlparser::ast::TypedString::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8001, 10], "end": [8001, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-861c11395ad137db3388d4b5"></a>
## hash

`function` · `sqlparser::ast::TypedString::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8001, 56], "end": [8001, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b1a59cfb37835443b006a5b"></a>
## partial_cmp

`function` · `sqlparser::ast::TypedString::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TypedString) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8001, 35], "end": [8001, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8001`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-786d6e9902841afba0e64d0c"></a>
## serialize

`function` · `sqlparser::ast::TypedString::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8002, 38], "end": [8002, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1db025148343e529b995a1aa"></a>
## uses_odbc_syntax

`struct_field` · `sqlparser::ast::TypedString::uses_odbc_syntax` · sqlparser 0.62.0

```rust
uses_odbc_syntax: bool
```

Source: `src/ast/mod.rs:8020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flags whether this TypedString uses the [ODBC syntax].

Example:
```sql
-- An ODBC date literal:
SELECT {d '2025-07-16'}
-- This is equivalent to the standard ANSI SQL literal:
SELECT DATE '2025-07-16'

[ODBC syntax]: https://learn.microsoft.com/en-us/sql/odbc/reference/develop-app/date-time-and-timestamp-literals?view=sql-server-2017

<a id="op-19a83e8d11722a4316c2ea79"></a>
## value

`struct_field` · `sqlparser::ast::TypedString::value` · sqlparser 0.62.0

```rust
value: ValueWithSpan
```

Source: `src/ast/mod.rs:8009`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value of the constant.
Hint: you can unwrap the string value using `value.into_string()`.

<a id="op-4a921fbd36f879507fb63d38"></a>
## visit

`function` · `sqlparser::ast::TypedString::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8003, 40], "end": [8003, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0e8a2f85f8576e335a2c960"></a>
## visit

`function` · `sqlparser::ast::TypedString::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TypedString", "path": "TypedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8003, 47], "end": [8003, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
