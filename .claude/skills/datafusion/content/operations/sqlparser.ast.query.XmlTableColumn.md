# `sqlparser::ast::query::XmlTableColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.XmlTableColumn.json).

<a id="op-29c96af03f47e7c3290907ca"></a>
## XmlTableColumn

`struct` · `sqlparser::ast::query::XmlTableColumn` · sqlparser 0.62.0

```rust
struct XmlTableColumn
```

Source: `src/ast/query.rs:4229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single column definition in XMLTABLE

```sql
COLUMNS
    id int PATH '@id',
    ordinality FOR ORDINALITY,
    "COUNTRY_NAME" text,
    country_id text PATH 'COUNTRY_ID',
    size_sq_km float PATH 'SIZE[@unit = "sq_km"]',
    size_other text PATH 'concat(SIZE[@unit!="sq_km"], " ", SIZE[@unit!="sq_km"]/@unit)',
    premier_name text PATH 'PREMIER_NAME' DEFAULT 'not specified'
```

<a id="op-b2ed4084feb4907cf6e67786"></a>
## clone

`function` · `sqlparser::ast::query::XmlTableColumn::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> XmlTableColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4226, 17], "end": [4226, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c2846bc43c7790499372473"></a>
## cmp

`function` · `sqlparser::ast::query::XmlTableColumn::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &XmlTableColumn) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4226, 51], "end": [4226, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-520d24704543475169536442"></a>
## deserialize

`function` · `sqlparser::ast::query::XmlTableColumn::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4228, 49], "end": [4228, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-430292f7df5370cb592fc82b"></a>
## eq

`function` · `sqlparser::ast::query::XmlTableColumn::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &XmlTableColumn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4226, 24], "end": [4226, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb2f61b877bba49f1fbbbc81"></a>
## fmt

`function` · `sqlparser::ast::query::XmlTableColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4226, 10], "end": [4226, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c288f049390fdfd4d990e0cb"></a>
## fmt

`function` · `sqlparser::ast::query::XmlTableColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4236, 1], "end": [4263, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bab974607fba8d6081ae7b06"></a>
## hash

`function` · `sqlparser::ast::query::XmlTableColumn::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4226, 56], "end": [4226, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd84b252712ef53b6b404c2e"></a>
## name

`struct_field` · `sqlparser::ast::query::XmlTableColumn::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/query.rs:4231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the column.

<a id="op-60dd099eee25e5d4682e6178"></a>
## option

`struct_field` · `sqlparser::ast::query::XmlTableColumn::option` · sqlparser 0.62.0

```rust
option: XmlTableColumnOption
```

Source: `src/ast/query.rs:4233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column options: type/path/default or FOR ORDINALITY

<a id="op-98d963f8d7214d0d815b8225"></a>
## partial_cmp

`function` · `sqlparser::ast::query::XmlTableColumn::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &XmlTableColumn) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4226, 35], "end": [4226, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-404350c5d03551d3f27bd2f3"></a>
## serialize

`function` · `sqlparser::ast::query::XmlTableColumn::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4228, 38], "end": [4228, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1416173011de50283ca642e4"></a>
## visit

`function` · `sqlparser::ast::query::XmlTableColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4227, 47], "end": [4227, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99a1fdfa58fe6adcdd7e2d38"></a>
## visit

`function` · `sqlparser::ast::query::XmlTableColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlTableColumn", "path": "XmlTableColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4227, 40], "end": [4227, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
