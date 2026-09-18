# `sqlparser::ast::ddl::IndexColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IndexColumn.json).

<a id="op-7b594611ac7779eb0ddff495"></a>
## IndexColumn

`struct` · `sqlparser::ast::ddl::IndexColumn` · sqlparser 0.62.0

```rust
struct IndexColumn
```

Source: `src/ast/ddl.rs:63`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index column type.

<a id="op-3377c1f40e6937d1fc9d9ea1"></a>
## clone

`function` · `sqlparser::ast::ddl::IndexColumn::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IndexColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 17], "end": [60, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63a7afcc8d0496f4bcd83e93"></a>
## cmp

`function` · `sqlparser::ast::ddl::IndexColumn::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IndexColumn) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 51], "end": [60, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3df3bebbceb2c51066c0e0d"></a>
## column

`struct_field` · `sqlparser::ast::ddl::IndexColumn::column` · sqlparser 0.62.0

```rust
column: ast::OrderByExpr
```

Source: `src/ast/ddl.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The indexed column expression.

<a id="op-d4e2cfe5fa20371e7932331f"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IndexColumn::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 49], "end": [61, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:61`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-425fe9837e7c9fd7ad911bd1"></a>
## eq

`function` · `sqlparser::ast::ddl::IndexColumn::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IndexColumn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 24], "end": [60, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f30b677747caeedde8b077a"></a>
## fmt

`function` · `sqlparser::ast::ddl::IndexColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [94, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51ea0c5266c8db5c40a7e0bf"></a>
## fmt

`function` · `sqlparser::ast::ddl::IndexColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a550dcd477cc8b1dc5e34e1"></a>
## from

`function` · `sqlparser::ast::ddl::IndexColumn::from` · sqlparser 0.62.0

```rust
fn from(c: Ident) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [77, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1ddc0144e80285fe33dc307"></a>
## from

`function` · `sqlparser::ast::ddl::IndexColumn::from` · sqlparser 0.62.0

```rust
fn from(c: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [84, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b490ba4bfb8df923c981164"></a>
## hash

`function` · `sqlparser::ast::ddl::IndexColumn::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 56], "end": [60, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6be0f2e44acec94fec1cba5"></a>
## operator_class

`struct_field` · `sqlparser::ast::ddl::IndexColumn::operator_class` · sqlparser 0.62.0

```rust
operator_class: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional operator class (index operator name).

<a id="op-b4e1fe3bd9057f1f29c40449"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IndexColumn::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IndexColumn) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 35], "end": [60, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fa3ff31136d81ce582f47f2"></a>
## serialize

`function` · `sqlparser::ast::ddl::IndexColumn::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 38], "end": [61, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:61`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d60a129d25196c0e33590e18"></a>
## span

`function` · `sqlparser::ast::ddl::IndexColumn::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "super::IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 1], "end": [719, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4c8f38ff1eacb9b7158681b"></a>
## visit

`function` · `sqlparser::ast::ddl::IndexColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 47], "end": [62, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1065ee6f21dd456da360075"></a>
## visit

`function` · `sqlparser::ast::ddl::IndexColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexColumn", "path": "IndexColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 40], "end": [62, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
