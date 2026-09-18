# `sqlparser::ast::ddl::ViewColumnDef`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ViewColumnDef.json).

<a id="op-1a1c1ffd79dd9074608efebb"></a>
## ViewColumnDef

`struct` · `sqlparser::ast::ddl::ViewColumnDef` · sqlparser 0.62.0

```rust
struct ViewColumnDef
```

Source: `src/ast/ddl.rs:1575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column definition specified in a `CREATE VIEW` statement.

Syntax
```markdown
<name> [data_type][OPTIONS(option, ...)]

option: <name> = <value>
```

Examples:
```sql
name
age OPTIONS(description = "age column", tag = "prod")
amount COMMENT 'The total amount for the order line'
created_at DateTime64
```

<a id="op-a0cb251acaaf96b8157d2551"></a>
## clone

`function` · `sqlparser::ast::ddl::ViewColumnDef::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ViewColumnDef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 17], "end": [1572, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5eea3e54d47d517f7c9fc25"></a>
## cmp

`function` · `sqlparser::ast::ddl::ViewColumnDef::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ViewColumnDef) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 51], "end": [1572, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03b7c89a3f665d6ac83838f8"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::ViewColumnDef::data_type` · sqlparser 0.62.0

```rust
data_type: Option<ast::DataType>
```

Source: `src/ast/ddl.rs:1579`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional data type for the column.

<a id="op-25862326be09b959df9b815f"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ViewColumnDef::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1573, 49], "end": [1573, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72a899df879ea258299b6872"></a>
## eq

`function` · `sqlparser::ast::ddl::ViewColumnDef::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ViewColumnDef) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 24], "end": [1572, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75e7eae1dacface7ae3d4fdf"></a>
## fmt

`function` · `sqlparser::ast::ddl::ViewColumnDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 10], "end": [1572, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d61c4dac5a4791b7c7700c72"></a>
## fmt

`function` · `sqlparser::ast::ddl::ViewColumnDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1605, 1], "end": [1623, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1606`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9468cd3d7a79a5be059f795a"></a>
## hash

`function` · `sqlparser::ast::ddl::ViewColumnDef::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 56], "end": [1572, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56d935894b6e2198ee92b0b9"></a>
## name

`struct_field` · `sqlparser::ast::ddl::ViewColumnDef::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:1577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column identifier.

<a id="op-6ca9e1c01dc2290ac26ffbd5"></a>
## options

`struct_field` · `sqlparser::ast::ddl::ViewColumnDef::options` · sqlparser 0.62.0

```rust
options: Option<ColumnOptions>
```

Source: `src/ast/ddl.rs:1581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional column options (defaults, comments, etc.).

<a id="op-9f8dde934bbae7441a1c6a6e"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ViewColumnDef::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ViewColumnDef) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 35], "end": [1572, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-061a261cda23939c1bee016d"></a>
## serialize

`function` · `sqlparser::ast::ddl::ViewColumnDef::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1573, 38], "end": [1573, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63bde8f816ce5b966f72255d"></a>
## span

`function` · `sqlparser::ast::ddl::ViewColumnDef::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "super::ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1007, 1], "end": [1017, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1008`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dbb1ac4f3ff54b41929b4a6"></a>
## visit

`function` · `sqlparser::ast::ddl::ViewColumnDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1574, 47], "end": [1574, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-babcea7335c80b13b59a59f2"></a>
## visit

`function` · `sqlparser::ast::ddl::ViewColumnDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ViewColumnDef", "path": "ViewColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1574, 40], "end": [1574, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
