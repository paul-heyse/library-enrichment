# `sqlparser::ast::query::RenameSelectItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.RenameSelectItem.json).

<a id="op-c3bd9e2afa9ae1caa24e9986"></a>
## RenameSelectItem

`enum` · `sqlparser::ast::query::RenameSelectItem` · sqlparser 0.62.0

```rust
enum RenameSelectItem
```

Source: `src/ast/query.rs:1064`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `RENAME` information.

# Syntax
```plaintext
<col_name> AS <col_alias>
| (<col_name> AS <col_alias>, <col_name> AS <col_alias>, ...)
```

<a id="op-cdbc93260bb26979dd5e91c3"></a>
## Multiple

`variant` · `sqlparser::ast::query::RenameSelectItem::Multiple` · sqlparser 0.62.0

```rust
Multiple
```

Source: `src/ast/query.rs:1077`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multiple column names with aliases inside parenthesis.
# Syntax
```plaintext
(<col_name> AS <col_alias>, <col_name> AS <col_alias>, ...)
```

<a id="op-9fe745c910f5f7a2735522a0"></a>
## Single

`variant` · `sqlparser::ast::query::RenameSelectItem::Single` · sqlparser 0.62.0

```rust
Single
```

Source: `src/ast/query.rs:1071`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single column name with alias without parenthesis.

# Syntax
```plaintext
<col_name> AS <col_alias>
```

<a id="op-0796844f83249c2b45bf013c"></a>
## clone

`function` · `sqlparser::ast::query::RenameSelectItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RenameSelectItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1061, 17], "end": [1061, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2f1ab9f00ef625067210167"></a>
## cmp

`function` · `sqlparser::ast::query::RenameSelectItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RenameSelectItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1061, 51], "end": [1061, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a50c3963fa0e2ffbb846592b"></a>
## deserialize

`function` · `sqlparser::ast::query::RenameSelectItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 49], "end": [1062, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0b6693002a682f575d984ac"></a>
## eq

`function` · `sqlparser::ast::query::RenameSelectItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RenameSelectItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1061, 24], "end": [1061, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbb94885b7f9ecb95df162b4"></a>
## fmt

`function` · `sqlparser::ast::query::RenameSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1080, 1], "end": [1093, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e83ce7c9ce0c453949d1f705"></a>
## fmt

`function` · `sqlparser::ast::query::RenameSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1061, 10], "end": [1061, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28bedeed630a3eb34d4bd66a"></a>
## hash

`function` · `sqlparser::ast::query::RenameSelectItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1061, 56], "end": [1061, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77e4ca8794378cc91941297f"></a>
## partial_cmp

`function` · `sqlparser::ast::query::RenameSelectItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RenameSelectItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1061, 35], "end": [1061, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5828570c3a32db9207c17513"></a>
## serialize

`function` · `sqlparser::ast::query::RenameSelectItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 38], "end": [1062, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05ae876b10068a0f73109538"></a>
## span

`function` · `sqlparser::ast::query::RenameSelectItem::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "super::RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1893, 1], "end": [1902, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7739cb2bd891b54cc5ec36dd"></a>
## visit

`function` · `sqlparser::ast::query::RenameSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1063, 47], "end": [1063, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5efd014b675b891026deed3"></a>
## visit

`function` · `sqlparser::ast::query::RenameSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RenameSelectItem", "path": "RenameSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1063, 40], "end": [1063, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
