# `sqlparser::ast::query::JsonTableNamedColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.JsonTableNamedColumn.json).

<a id="op-ee3057281683a896f2703692"></a>
## JsonTableNamedColumn

`struct` · `sqlparser::ast::query::JsonTableNamedColumn` · sqlparser 0.62.0

```rust
struct JsonTableNamedColumn
```

Source: `src/ast/query.rs:4053`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single column definition in MySQL's `JSON_TABLE` table valued function.

See <https://mariadb.com/kb/en/json_table/#path-columns>

```sql
        value VARCHAR(20) PATH '$'
```

<a id="op-618a8221eb81e887f027187f"></a>
## clone

`function` · `sqlparser::ast::query::JsonTableNamedColumn::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonTableNamedColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4050, 17], "end": [4050, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-059cd23328120c522c512967"></a>
## cmp

`function` · `sqlparser::ast::query::JsonTableNamedColumn::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonTableNamedColumn) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4050, 51], "end": [4050, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d13466b6490446c918830a0b"></a>
## deserialize

`function` · `sqlparser::ast::query::JsonTableNamedColumn::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4052, 49], "end": [4052, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7d089eefac39c46339d4e49"></a>
## eq

`function` · `sqlparser::ast::query::JsonTableNamedColumn::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonTableNamedColumn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4050, 24], "end": [4050, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7007ae2de0a41f28a287c5e"></a>
## exists

`struct_field` · `sqlparser::ast::query::JsonTableNamedColumn::exists` · sqlparser 0.62.0

```rust
exists: bool
```

Source: `src/ast/query.rs:4061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

true if the column is a boolean set to true if the given path exists

<a id="op-140b89776bf6060adeceb3d6"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableNamedColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4068, 1], "end": [4086, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4069`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2050add4e30a1f0499a1661"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableNamedColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4050, 10], "end": [4050, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb337d76894ec97ccee2105a"></a>
## hash

`function` · `sqlparser::ast::query::JsonTableNamedColumn::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4050, 56], "end": [4050, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-576a5f7cafea85c63f3f45c1"></a>
## name

`struct_field` · `sqlparser::ast::query::JsonTableNamedColumn::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/query.rs:4055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the column to be extracted.

<a id="op-ea5fd983615e3c6814f4e56b"></a>
## on_empty

`struct_field` · `sqlparser::ast::query::JsonTableNamedColumn::on_empty` · sqlparser 0.62.0

```rust
on_empty: Option<JsonTableColumnErrorHandling>
```

Source: `src/ast/query.rs:4063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The empty handling clause of the column

<a id="op-cfdb6a32ed02aa0c737a14e0"></a>
## on_error

`struct_field` · `sqlparser::ast::query::JsonTableNamedColumn::on_error` · sqlparser 0.62.0

```rust
on_error: Option<JsonTableColumnErrorHandling>
```

Source: `src/ast/query.rs:4065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The error handling clause of the column

<a id="op-b3e4900794ff8923058b6a28"></a>
## partial_cmp

`function` · `sqlparser::ast::query::JsonTableNamedColumn::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonTableNamedColumn) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4050, 35], "end": [4050, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c55803f9362c137407ead38a"></a>
## path

`struct_field` · `sqlparser::ast::query::JsonTableNamedColumn::path` · sqlparser 0.62.0

```rust
path: ValueWithSpan
```

Source: `src/ast/query.rs:4059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The path to the column to be extracted. Must be a literal string.

<a id="op-e6ccc8dea040164083d8501c"></a>
## serialize

`function` · `sqlparser::ast::query::JsonTableNamedColumn::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4052, 38], "end": [4052, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a2388ffee7e4174ebc1adf6"></a>
## type

`struct_field` · `sqlparser::ast::query::JsonTableNamedColumn::type` · sqlparser 0.62.0

```rust
type: DataType
```

Source: `src/ast/query.rs:4057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of the column to be extracted.

<a id="op-931c5aa849d9f86572fae9d8"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableNamedColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4051, 47], "end": [4051, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97db28ba9c6126a29cc319f8"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableNamedColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNamedColumn", "path": "JsonTableNamedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4051, 40], "end": [4051, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
