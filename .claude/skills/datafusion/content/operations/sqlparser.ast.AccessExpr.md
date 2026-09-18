# `sqlparser::ast::AccessExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AccessExpr.json).

<a id="op-d2f94e541966c169e88eb3d9"></a>
## AccessExpr

`enum` · `sqlparser::ast::AccessExpr` · sqlparser 0.62.0

```rust
enum AccessExpr
```

Source: `src/ast/mod.rs:1456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An element of a [`Expr::CompoundFieldAccess`](../operations/sqlparser.ast.Expr.md#op-caa40f99aa3a3865bf4efc4e).
It can be an expression or a subscript.

<a id="op-2bbe7474717ef1d54c8a05c4"></a>
## Dot

`variant` · `sqlparser::ast::AccessExpr::Dot` · sqlparser 0.62.0

```rust
Dot
```

Source: `src/ast/mod.rs:1458`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Accesses a field using dot notation, e.g. `foo.bar.baz`.

<a id="op-fdaaf58d024be10187080391"></a>
## Subscript

`variant` · `sqlparser::ast::AccessExpr::Subscript` · sqlparser 0.62.0

```rust
Subscript
```

Source: `src/ast/mod.rs:1460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Accesses a field or array element using bracket notation, e.g. `foo['bar']`.

<a id="op-1a117c296b3623295104f362"></a>
## clone

`function` · `sqlparser::ast::AccessExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AccessExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 17], "end": [1453, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72530d73e4658785c56e3fe8"></a>
## cmp

`function` · `sqlparser::ast::AccessExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AccessExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 51], "end": [1453, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c89cfee09af5d5dde55ed70c"></a>
## deserialize

`function` · `sqlparser::ast::AccessExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1454, 49], "end": [1454, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:1454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-558f74ac0e93549f2bc609c4"></a>
## eq

`function` · `sqlparser::ast::AccessExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AccessExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 24], "end": [1453, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b39b31773d0b34189553b28"></a>
## fmt

`function` · `sqlparser::ast::AccessExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 10], "end": [1453, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bbd2953fd0ca051f529b8d1"></a>
## fmt

`function` · `sqlparser::ast::AccessExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1463, 1], "end": [1470, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:1464`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c7fcbddc6b547513c85b9c6"></a>
## hash

`function` · `sqlparser::ast::AccessExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 56], "end": [1453, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46466fc93f59dc556e150a83"></a>
## partial_cmp

`function` · `sqlparser::ast::AccessExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AccessExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1453, 35], "end": [1453, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:1453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6be3a8cd833d747a93092d8"></a>
## serialize

`function` · `sqlparser::ast::AccessExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1454, 38], "end": [1454, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:1454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6014cef90813c54b0a7af014"></a>
## span

`function` · `sqlparser::ast::AccessExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "super::AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1691, 1], "end": [1698, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1992aa5dde8fa94b3743595d"></a>
## visit

`function` · `sqlparser::ast::AccessExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1455, 47], "end": [1455, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:1455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe52743ae6a4ea5a8b65e8ca"></a>
## visit

`function` · `sqlparser::ast::AccessExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AccessExpr", "path": "AccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1455, 40], "end": [1455, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:1455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
