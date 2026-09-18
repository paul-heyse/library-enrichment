# `sqlparser::ast::query::TableAliasColumnDef`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableAliasColumnDef.json).

<a id="op-73fb458e51ee86d32e3aa701"></a>
## TableAliasColumnDef

`struct` · `sqlparser::ast::query::TableAliasColumnDef` · sqlparser 0.62.0

```rust
struct TableAliasColumnDef
```

Source: `src/ast/query.rs:2565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL column definition in a table expression alias.
Most of the time, the data type is not specified.
But some table-valued functions do require specifying the data type.

See <https://www.postgresql.org/docs/17/queries-table-expressions.html#QUERIES-TABLEFUNCTIONS>

<a id="op-c447146a8ce8dda89c1b03bf"></a>
## clone

`function` · `sqlparser::ast::query::TableAliasColumnDef::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableAliasColumnDef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2562, 17], "end": [2562, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ed02821337c206ef8180e44"></a>
## cmp

`function` · `sqlparser::ast::query::TableAliasColumnDef::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableAliasColumnDef) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2562, 51], "end": [2562, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b7c892ab73dd1fa44e93cff"></a>
## data_type

`struct_field` · `sqlparser::ast::query::TableAliasColumnDef::data_type` · sqlparser 0.62.0

```rust
data_type: Option<DataType>
```

Source: `src/ast/query.rs:2569`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Some table-valued functions require specifying the data type in the alias.

<a id="op-1dbc46ac6e4dcc46a7477c4f"></a>
## deserialize

`function` · `sqlparser::ast::query::TableAliasColumnDef::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2563, 49], "end": [2563, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a750edd87a1270a206a50234"></a>
## eq

`function` · `sqlparser::ast::query::TableAliasColumnDef::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableAliasColumnDef) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2562, 24], "end": [2562, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1990694f88f1e1ebbeb3a904"></a>
## fmt

`function` · `sqlparser::ast::query::TableAliasColumnDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2582, 1], "end": [2590, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b491658b626e7491050aa4f4"></a>
## fmt

`function` · `sqlparser::ast::query::TableAliasColumnDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2562, 10], "end": [2562, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f37464c7fc3c4c659203efce"></a>
## from_name

`function` · `sqlparser::ast::query::TableAliasColumnDef::from_name` · sqlparser 0.62.0

```rust
fn from_name<S: Into<String>>(name: S) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2572, 1], "end": [2580, 2], "filename": "src/ast/query.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/query.rs:2574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new table alias column definition with only a name and no type

<a id="op-8ebf614aa91b6668a64af613"></a>
## hash

`function` · `sqlparser::ast::query::TableAliasColumnDef::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2562, 56], "end": [2562, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3700ca1ba7476345db747df"></a>
## name

`struct_field` · `sqlparser::ast::query::TableAliasColumnDef::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/query.rs:2567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column name alias

<a id="op-0ebfd83d889e9eeea24bbd2b"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableAliasColumnDef::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableAliasColumnDef) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2562, 35], "end": [2562, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2ec76ed0692fb8699668006"></a>
## serialize

`function` · `sqlparser::ast::query::TableAliasColumnDef::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2563, 38], "end": [2563, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-483eb08cefdbbe80f24ae6a4"></a>
## span

`function` · `sqlparser::ast::query::TableAliasColumnDef::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "super::TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2199, 1], "end": [2205, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-518126aeac74c8869b48bd24"></a>
## visit

`function` · `sqlparser::ast::query::TableAliasColumnDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2564, 47], "end": [2564, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a61fb6edab8f90f491ebd4bd"></a>
## visit

`function` · `sqlparser::ast::query::TableAliasColumnDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableAliasColumnDef", "path": "TableAliasColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2564, 40], "end": [2564, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
