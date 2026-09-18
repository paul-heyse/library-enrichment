# `sqlparser::ast::ddl::ColumnDef`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ColumnDef.json).

<a id="op-b2035c41c8e81c8fcecd9db2"></a>
## ColumnDef

`struct` · `sqlparser::ast::ddl::ColumnDef` · sqlparser 0.62.0

```rust
struct ColumnDef
```

Source: `src/ast/ddl.rs:1533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL column definition

<a id="op-745709a9c9450c36c8fd5170"></a>
## clone

`function` · `sqlparser::ast::ddl::ColumnDef::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ColumnDef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1530, 17], "end": [1530, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-648977eeea4c5764ab0064ae"></a>
## cmp

`function` · `sqlparser::ast::ddl::ColumnDef::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ColumnDef) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1530, 51], "end": [1530, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18276ab9cadf6812d7ad4120"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::ColumnDef::data_type` · sqlparser 0.62.0

```rust
data_type: ast::DataType
```

Source: `src/ast/ddl.rs:1537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column data type.

<a id="op-7dce2bb058fe282491860960"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ColumnDef::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1531, 49], "end": [1531, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1531`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02bec480cb7043678b7deecc"></a>
## eq

`function` · `sqlparser::ast::ddl::ColumnDef::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ColumnDef) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1530, 24], "end": [1530, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03251ea18488467a3e065a4b"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1542, 1], "end": [1554, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1543`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-088768e2b4af101cb5597c96"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1530, 10], "end": [1530, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-353369e65c9e00e5d5a75317"></a>
## hash

`function` · `sqlparser::ast::ddl::ColumnDef::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1530, 56], "end": [1530, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e0b03e070c335816fa92ee5"></a>
## name

`struct_field` · `sqlparser::ast::ddl::ColumnDef::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:1535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column name.

<a id="op-d461006a04c9ba43e040af20"></a>
## options

`struct_field` · `sqlparser::ast::ddl::ColumnDef::options` · sqlparser 0.62.0

```rust
options: Vec<ColumnOptionDef>
```

Source: `src/ast/ddl.rs:1539`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column options (defaults, constraints, generated, etc.).

<a id="op-ff2c832da9414714c7838bbe"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ColumnDef::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ColumnDef) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1530, 35], "end": [1530, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1aea3db01b092cb691c0cf5d"></a>
## serialize

`function` · `sqlparser::ast::ddl::ColumnDef::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1531, 38], "end": [1531, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1531`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bfd4f4c7402f657fd10fc66"></a>
## span

`function` · `sqlparser::ast::ddl::ColumnDef::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "super::ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [622, 1], "end": [632, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:623`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f8ceadab01628d34ef924d0"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1532, 40], "end": [1532, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1532`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaf8b0b793acbe10dd084d3d"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnDef", "path": "ColumnDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1532, 47], "end": [1532, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1532`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
