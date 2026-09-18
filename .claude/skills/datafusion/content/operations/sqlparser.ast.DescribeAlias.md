# `sqlparser::ast::DescribeAlias`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DescribeAlias.json).

<a id="op-696e922be52e706fb8615283"></a>
## DescribeAlias

`enum` · `sqlparser::ast::DescribeAlias` · sqlparser 0.62.0

```rust
enum DescribeAlias
```

Source: `src/ast/mod.rs:8662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Aliases accepted for describe-style commands.

<a id="op-674e8b1ef102f85570e5c0d0"></a>
## Desc

`variant` · `sqlparser::ast::DescribeAlias::Desc` · sqlparser 0.62.0

```rust
Desc
```

Source: `src/ast/mod.rs:8668`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DESC` alias.

<a id="op-701856b2626bf349661cc9b0"></a>
## Describe

`variant` · `sqlparser::ast::DescribeAlias::Describe` · sqlparser 0.62.0

```rust
Describe
```

Source: `src/ast/mod.rs:8664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DESCRIBE` alias.

<a id="op-9bd319338965ae4a9d4937f7"></a>
## Explain

`variant` · `sqlparser::ast::DescribeAlias::Explain` · sqlparser 0.62.0

```rust
Explain
```

Source: `src/ast/mod.rs:8666`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXPLAIN` alias.

<a id="op-0321c086129570dba3360c9c"></a>
## clone

`function` · `sqlparser::ast::DescribeAlias::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DescribeAlias
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8658, 23], "end": [8658, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa07e63752378013c2db5ea2"></a>
## cmp

`function` · `sqlparser::ast::DescribeAlias::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DescribeAlias) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8658, 57], "end": [8658, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1063c98b25810a497cfdd92"></a>
## deserialize

`function` · `sqlparser::ast::DescribeAlias::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8659, 49], "end": [8659, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dc726a9815c523b6b6ac408"></a>
## eq

`function` · `sqlparser::ast::DescribeAlias::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DescribeAlias) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8658, 30], "end": [8658, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43109504520576f36e96aae8"></a>
## fmt

`function` · `sqlparser::ast::DescribeAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8658, 10], "end": [8658, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98023221f5ebf0c5dbe4b434"></a>
## fmt

`function` · `sqlparser::ast::DescribeAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8671, 1], "end": [8680, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-026a8ecd5a7b01bbf0dec49f"></a>
## hash

`function` · `sqlparser::ast::DescribeAlias::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8658, 62], "end": [8658, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c4735504e664df0c0a2af06"></a>
## partial_cmp

`function` · `sqlparser::ast::DescribeAlias::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DescribeAlias) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8658, 41], "end": [8658, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-149f0884b24ce8f04116c369"></a>
## serialize

`function` · `sqlparser::ast::DescribeAlias::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8659, 38], "end": [8659, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3f58f1f609dff16288eb8b2"></a>
## visit

`function` · `sqlparser::ast::DescribeAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8660, 40], "end": [8660, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8660`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde425b3742f8cecc4c4078d"></a>
## visit

`function` · `sqlparser::ast::DescribeAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DescribeAlias", "path": "DescribeAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8660, 47], "end": [8660, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8660`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
