# `sqlparser::ast::query::ExceptSelectItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ExceptSelectItem.json).

<a id="op-4874ecd0bfaea9ff30827485"></a>
## ExceptSelectItem

`struct` · `sqlparser::ast::query::ExceptSelectItem` · sqlparser 0.62.0

```rust
struct ExceptSelectItem
```

Source: `src/ast/query.rs:1104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bigquery `EXCEPT` information, with at least one column.

# Syntax
```plaintext
EXCEPT (<col_name> [, ...])
```

<a id="op-1e394adfba9597d7b66c5226"></a>
## additional_elements

`struct_field` · `sqlparser::ast::query::ExceptSelectItem::additional_elements` · sqlparser 0.62.0

```rust
additional_elements: Vec<Ident>
```

Source: `src/ast/query.rs:1108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional columns. This list can be empty.

<a id="op-631ea96ad97c3f9e52864970"></a>
## clone

`function` · `sqlparser::ast::query::ExceptSelectItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExceptSelectItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1101, 17], "end": [1101, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c94cd703898d9a495a58d11c"></a>
## cmp

`function` · `sqlparser::ast::query::ExceptSelectItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExceptSelectItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1101, 51], "end": [1101, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4564ae7e1ee20c52d6deca70"></a>
## deserialize

`function` · `sqlparser::ast::query::ExceptSelectItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1102, 49], "end": [1102, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc671c5148515f884770cfb5"></a>
## eq

`function` · `sqlparser::ast::query::ExceptSelectItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExceptSelectItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1101, 24], "end": [1101, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e4a6291f223ab687ec12dc7"></a>
## first_element

`struct_field` · `sqlparser::ast::query::ExceptSelectItem::first_element` · sqlparser 0.62.0

```rust
first_element: Ident
```

Source: `src/ast/query.rs:1106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

First guaranteed column.

<a id="op-0c6ab970bb7be214f303e234"></a>
## fmt

`function` · `sqlparser::ast::query::ExceptSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1101, 10], "end": [1101, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f812f2144685527e0970032"></a>
## fmt

`function` · `sqlparser::ast::query::ExceptSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1111, 1], "end": [1126, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a124b4ab84b81d203542ce9"></a>
## hash

`function` · `sqlparser::ast::query::ExceptSelectItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1101, 56], "end": [1101, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d212a8042c94e5d8afba1c1"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ExceptSelectItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExceptSelectItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1101, 35], "end": [1101, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84747c8f9831e7da0fb6bd3a"></a>
## serialize

`function` · `sqlparser::ast::query::ExceptSelectItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1102, 38], "end": [1102, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13f840d5e456d9025e59317d"></a>
## span

`function` · `sqlparser::ast::query::ExceptSelectItem::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "super::ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1904, 1], "end": [1915, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-130244ec7deb8afbc10bcb92"></a>
## visit

`function` · `sqlparser::ast::query::ExceptSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 47], "end": [1103, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e71119fd56cc611ba8f4f067"></a>
## visit

`function` · `sqlparser::ast::query::ExceptSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExceptSelectItem", "path": "ExceptSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 40], "end": [1103, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
