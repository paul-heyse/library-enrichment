# `sqlparser::ast::query::ReplaceSelectElement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ReplaceSelectElement.json).

<a id="op-e079db0df3144b85cb578fae"></a>
## ReplaceSelectElement

`struct` · `sqlparser::ast::query::ReplaceSelectElement` · sqlparser 0.62.0

```rust
struct ReplaceSelectElement
```

Source: `src/ast/query.rs:1158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

# Syntax
```plaintext
<expr> [AS] <column_name>
```

<a id="op-0a443b46a5faa224603eb78f"></a>
## as_keyword

`struct_field` · `sqlparser::ast::query::ReplaceSelectElement::as_keyword` · sqlparser 0.62.0

```rust
as_keyword: bool
```

Source: `src/ast/query.rs:1164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `AS` keyword was present in the original syntax.

<a id="op-f79cc073529e5123eb713626"></a>
## clone

`function` · `sqlparser::ast::query::ReplaceSelectElement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ReplaceSelectElement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 17], "end": [1155, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-629fc3cb3c852dfb66487959"></a>
## cmp

`function` · `sqlparser::ast::query::ReplaceSelectElement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ReplaceSelectElement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 51], "end": [1155, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d75309829c2e96be1d0e10f1"></a>
## column_name

`struct_field` · `sqlparser::ast::query::ReplaceSelectElement::column_name` · sqlparser 0.62.0

```rust
column_name: Ident
```

Source: `src/ast/query.rs:1162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The target column name for the replacement.

<a id="op-bc75ce6325fb138dba2175e2"></a>
## deserialize

`function` · `sqlparser::ast::query::ReplaceSelectElement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1156, 49], "end": [1156, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d9fc953526b1eefca4d4609"></a>
## eq

`function` · `sqlparser::ast::query::ReplaceSelectElement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ReplaceSelectElement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 24], "end": [1155, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce73325af21f5e29b7e4ea61"></a>
## expr

`struct_field` · `sqlparser::ast::query::ReplaceSelectElement::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:1160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression producing the replacement value.

<a id="op-9e2b440cd892aaec0be71f9e"></a>
## fmt

`function` · `sqlparser::ast::query::ReplaceSelectElement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1175, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df41bb1167d1a6102d604a0f"></a>
## fmt

`function` · `sqlparser::ast::query::ReplaceSelectElement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 10], "end": [1155, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c368ee27a3d8c07053549d4"></a>
## hash

`function` · `sqlparser::ast::query::ReplaceSelectElement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 56], "end": [1155, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2386dd8a3b3bfe7e74c643bd"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ReplaceSelectElement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ReplaceSelectElement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 35], "end": [1155, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6c20f5e10681d26cac1fae2"></a>
## serialize

`function` · `sqlparser::ast::query::ReplaceSelectElement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1156, 38], "end": [1156, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f1c1d26dde5a60f9cc47b0"></a>
## span

`function` · `sqlparser::ast::query::ReplaceSelectElement::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "super::ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1925, 1], "end": [1935, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67fc5c589ebfa1d6d2e5326a"></a>
## visit

`function` · `sqlparser::ast::query::ReplaceSelectElement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1157, 40], "end": [1157, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc7c17f383055f9be0f2e2a1"></a>
## visit

`function` · `sqlparser::ast::query::ReplaceSelectElement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectElement", "path": "ReplaceSelectElement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1157, 47], "end": [1157, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
