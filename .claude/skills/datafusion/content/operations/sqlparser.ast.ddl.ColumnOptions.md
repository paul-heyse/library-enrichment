# `sqlparser::ast::ddl::ColumnOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ColumnOptions.json).

<a id="op-2efa5d613f59ac8df721c3a9"></a>
## ColumnOptions

`enum` · `sqlparser::ast::ddl::ColumnOptions` · sqlparser 0.62.0

```rust
enum ColumnOptions
```

Source: `src/ast/ddl.rs:1588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Representation of how multiple `ColumnOption`s are grouped for a column.

<a id="op-cb3e12fe72948a1c357f5b60"></a>
## CommaSeparated

`variant` · `sqlparser::ast::ddl::ColumnOptions::CommaSeparated` · sqlparser 0.62.0

```rust
CommaSeparated
```

Source: `src/ast/ddl.rs:1590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options separated by comma: `OPTIONS(a, b, c)`.

<a id="op-cee9bc1b975fc16b81816fae"></a>
## SpaceSeparated

`variant` · `sqlparser::ast::ddl::ColumnOptions::SpaceSeparated` · sqlparser 0.62.0

```rust
SpaceSeparated
```

Source: `src/ast/ddl.rs:1592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options separated by spaces: `OPTION_A OPTION_B`.

<a id="op-6343dbceabcb6d0d2d85eab0"></a>
## as_slice

`function` · `sqlparser::ast::ddl::ColumnOptions::as_slice` · sqlparser 0.62.0

```rust
fn as_slice(&self) -> &[ColumnOption]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1595, 1], "end": [1603, 2], "filename": "src/ast/ddl.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/ddl.rs:1597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Get the column options as a slice.

<a id="op-c888e18296427fa286c3e47d"></a>
## clone

`function` · `sqlparser::ast::ddl::ColumnOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ColumnOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 17], "end": [1584, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc9300c042d9f5457359f7f5"></a>
## cmp

`function` · `sqlparser::ast::ddl::ColumnOptions::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ColumnOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 51], "end": [1584, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-069979c4dd8254c93060cc0b"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ColumnOptions::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 49], "end": [1585, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8e47dafb8963e3fd72676b6"></a>
## eq

`function` · `sqlparser::ast::ddl::ColumnOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ColumnOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 24], "end": [1584, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc121e3b9366fddbee29cba6"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 10], "end": [1584, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-888b58f09dc43399c49b9dfe"></a>
## hash

`function` · `sqlparser::ast::ddl::ColumnOptions::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 56], "end": [1584, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-881f65eb22e3ec084a408a81"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ColumnOptions::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ColumnOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 35], "end": [1584, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-230567bff255aef49faa06a8"></a>
## serialize

`function` · `sqlparser::ast::ddl::ColumnOptions::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 38], "end": [1585, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94e60a4dbc4fb78b4b3b8079"></a>
## span

`function` · `sqlparser::ast::ddl::ColumnOptions::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "crate::ast::ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1019, 1], "end": [1023, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea1d75f69f0ec920de68c7ec"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1586, 47], "end": [1586, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1eaae9b1065ca9e51b38af1"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptions", "path": "ColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1586, 40], "end": [1586, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
