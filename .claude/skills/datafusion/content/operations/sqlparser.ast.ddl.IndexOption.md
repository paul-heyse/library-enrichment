# `sqlparser::ast::ddl::IndexOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IndexOption.json).

<a id="op-0dc5af5d56d2665aea0a2488"></a>
## IndexOption

`enum` · `sqlparser::ast::ddl::IndexOption` · sqlparser 0.62.0

```rust
enum IndexOption
```

Source: `src/ast/ddl.rs:1455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL index option, used in [`CREATE TABLE`], [`CREATE INDEX`], and [`ALTER TABLE`].

[`CREATE TABLE`]: https://dev.mysql.com/doc/refman/8.4/en/create-table.html
[`CREATE INDEX`]: https://dev.mysql.com/doc/refman/8.4/en/create-index.html
[`ALTER TABLE`]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html

<a id="op-d85189585ad69c1678847df7"></a>
## Comment

`variant` · `sqlparser::ast::ddl::IndexOption::Comment` · sqlparser 0.62.0

```rust
Comment
```

Source: `src/ast/ddl.rs:1461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`COMMENT 'string'`: Specifies a comment for the index.

<a id="op-bff8026faae79ff449fa0546"></a>
## Using

`variant` · `sqlparser::ast::ddl::IndexOption::Using` · sqlparser 0.62.0

```rust
Using
```

Source: `src/ast/ddl.rs:1459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`USING { BTREE | HASH }`: Index type to use for the index.

Note that we permissively parse non-MySQL index types, like `GIN`.

<a id="op-b05a01774bd314756601aa58"></a>
## clone

`function` · `sqlparser::ast::ddl::IndexOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IndexOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1452, 17], "end": [1452, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc39b7442f060b6140f9a7f3"></a>
## cmp

`function` · `sqlparser::ast::ddl::IndexOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IndexOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1452, 51], "end": [1452, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ba72047439ca160de6b3931"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IndexOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 49], "end": [1453, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f63e53dfb1c928ef555b092b"></a>
## eq

`function` · `sqlparser::ast::ddl::IndexOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IndexOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1452, 24], "end": [1452, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-161f0b7ec12310c9b3e4b395"></a>
## fmt

`function` · `sqlparser::ast::ddl::IndexOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1452, 10], "end": [1452, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea5aada85aeb2862ea2d210f"></a>
## fmt

`function` · `sqlparser::ast::ddl::IndexOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1471, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-199a789cacfe6f194535de9f"></a>
## hash

`function` · `sqlparser::ast::ddl::IndexOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1452, 56], "end": [1452, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4d0d8b3ed001db93172b23b"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IndexOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IndexOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1452, 35], "end": [1452, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8009c91f40eedc01335c8a2"></a>
## serialize

`function` · `sqlparser::ast::ddl::IndexOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 38], "end": [1453, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a8e2d86dada099ea6eb098b"></a>
## visit

`function` · `sqlparser::ast::ddl::IndexOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1454, 40], "end": [1454, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b83fac80b919424e7111f0cd"></a>
## visit

`function` · `sqlparser::ast::ddl::IndexOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexOption", "path": "IndexOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1454, 47], "end": [1454, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
