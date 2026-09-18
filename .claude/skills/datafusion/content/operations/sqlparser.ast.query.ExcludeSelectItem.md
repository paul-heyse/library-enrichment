# `sqlparser::ast::query::ExcludeSelectItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ExcludeSelectItem.json).

<a id="op-c57dffa284e52e71060afb86"></a>
## ExcludeSelectItem

`enum` · `sqlparser::ast::query::ExcludeSelectItem` · sqlparser 0.62.0

```rust
enum ExcludeSelectItem
```

Source: `src/ast/query.rs:1023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `EXCLUDE` information.

# Syntax
```plaintext
<col_name>
| (<col_name>, <col_name>, ...)
```

<a id="op-bd10f8fd6928ea2476ec6756"></a>
## Multiple

`variant` · `sqlparser::ast::query::ExcludeSelectItem::Multiple` · sqlparser 0.62.0

```rust
Multiple
```

Source: `src/ast/query.rs:1036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multiple column names inside parenthesis.
# Syntax
```plaintext
(<col_name>, <col_name>, ...)
```

<a id="op-cd6392b742997c5395285e86"></a>
## Single

`variant` · `sqlparser::ast::query::ExcludeSelectItem::Single` · sqlparser 0.62.0

```rust
Single
```

Source: `src/ast/query.rs:1030`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single column name without parenthesis.

# Syntax
```plaintext
<col_name>
```

<a id="op-c06670d8160355aa50899bcd"></a>
## clone

`function` · `sqlparser::ast::query::ExcludeSelectItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExcludeSelectItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1020, 17], "end": [1020, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-828cfa70ff15348360e658de"></a>
## cmp

`function` · `sqlparser::ast::query::ExcludeSelectItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExcludeSelectItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1020, 51], "end": [1020, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89a3aee107a1ae4bc202a4f4"></a>
## deserialize

`function` · `sqlparser::ast::query::ExcludeSelectItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1021, 49], "end": [1021, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1900befd55a56d31ffac056a"></a>
## eq

`function` · `sqlparser::ast::query::ExcludeSelectItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExcludeSelectItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1020, 24], "end": [1020, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64f55488d7c5f924b9319873"></a>
## fmt

`function` · `sqlparser::ast::query::ExcludeSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1039, 1], "end": [1052, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1040`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dcd656d45707bf81b8b1743"></a>
## fmt

`function` · `sqlparser::ast::query::ExcludeSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1020, 10], "end": [1020, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0ade2f9cd0c1079cacd5121"></a>
## hash

`function` · `sqlparser::ast::query::ExcludeSelectItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1020, 56], "end": [1020, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09b74ac4d3266b707c096f9a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ExcludeSelectItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExcludeSelectItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1020, 35], "end": [1020, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3f5e85f3cfbb1f6f757e6c0"></a>
## serialize

`function` · `sqlparser::ast::query::ExcludeSelectItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1021, 38], "end": [1021, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bceea20479d6f0701ec67ad"></a>
## span

`function` · `sqlparser::ast::query::ExcludeSelectItem::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "super::ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1884, 1], "end": [1891, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0557949e9f87bb35bb308589"></a>
## visit

`function` · `sqlparser::ast::query::ExcludeSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1022, 40], "end": [1022, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-783580df9cc6d511ecbb46b2"></a>
## visit

`function` · `sqlparser::ast::query::ExcludeSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExcludeSelectItem", "path": "ExcludeSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1022, 47], "end": [1022, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
