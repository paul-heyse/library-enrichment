# `sqlparser::ast::query::ReplaceSelectItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ReplaceSelectItem.json).

<a id="op-691b5be60a535cb7b591da89"></a>
## ReplaceSelectItem

`struct` · `sqlparser::ast::query::ReplaceSelectItem` · sqlparser 0.62.0

```rust
struct ReplaceSelectItem
```

Source: `src/ast/query.rs:1138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bigquery `REPLACE` information.

# Syntax
```plaintext
REPLACE (<new_expr> [AS] <col_name>)
REPLACE (<col_name> [AS] <col_alias>, <col_name> [AS] <col_alias>, ...)
```

<a id="op-3b1a9871a299af34069000f5"></a>
## clone

`function` · `sqlparser::ast::query::ReplaceSelectItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ReplaceSelectItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1135, 17], "end": [1135, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52d686e25150b2e778a538f2"></a>
## cmp

`function` · `sqlparser::ast::query::ReplaceSelectItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ReplaceSelectItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1135, 51], "end": [1135, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a5d9e53f5ac307e2c0afcd0"></a>
## deserialize

`function` · `sqlparser::ast::query::ReplaceSelectItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1136, 49], "end": [1136, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13f588fd70c18995e159889a"></a>
## eq

`function` · `sqlparser::ast::query::ReplaceSelectItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ReplaceSelectItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1135, 24], "end": [1135, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d42d3522a97b63520edaac1"></a>
## fmt

`function` · `sqlparser::ast::query::ReplaceSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1135, 10], "end": [1135, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee5e5a08be69ccd74a6b1307"></a>
## fmt

`function` · `sqlparser::ast::query::ReplaceSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1143, 1], "end": [1149, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce1448d9af4df97030e55906"></a>
## hash

`function` · `sqlparser::ast::query::ReplaceSelectItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1135, 56], "end": [1135, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b42a2364b03241f2b3678944"></a>
## items

`struct_field` · `sqlparser::ast::query::ReplaceSelectItem::items` · sqlparser 0.62.0

```rust
items: Vec<Box<ReplaceSelectElement>>
```

Source: `src/ast/query.rs:1140`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of replacement elements contained in the `REPLACE(...)` clause.

<a id="op-ab708da365011431c81de839"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ReplaceSelectItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ReplaceSelectItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1135, 35], "end": [1135, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e8e35d15b00b1bd0b075181"></a>
## serialize

`function` · `sqlparser::ast::query::ReplaceSelectItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1136, 38], "end": [1136, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6743106859dc022ff4185c8"></a>
## span

`function` · `sqlparser::ast::query::ReplaceSelectItem::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "super::ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1917, 1], "end": [1923, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a5e91499547a6e68c65f5b0"></a>
## visit

`function` · `sqlparser::ast::query::ReplaceSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 47], "end": [1137, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d5077a5ddce7779a6ae1cff"></a>
## visit

`function` · `sqlparser::ast::query::ReplaceSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ReplaceSelectItem", "path": "ReplaceSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 40], "end": [1137, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
