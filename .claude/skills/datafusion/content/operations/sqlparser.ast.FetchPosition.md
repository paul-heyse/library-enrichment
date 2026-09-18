# `sqlparser::ast::FetchPosition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FetchPosition.json).

<a id="op-84ab1b38d889dcd967e9116b"></a>
## FetchPosition

`enum` · `sqlparser::ast::FetchPosition` · sqlparser 0.62.0

```rust
enum FetchPosition
```

Source: `src/ast/mod.rs:6950`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The "position" for a FETCH statement.

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/language-elements/fetch-transact-sql)

<a id="op-5ffac76f6b491755559a8109"></a>
## From

`variant` · `sqlparser::ast::FetchPosition::From` · sqlparser 0.62.0

```rust
From
```

Source: `src/ast/mod.rs:6952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use `FROM <pos>` position specifier.

<a id="op-175004aec951fc7af0c1142a"></a>
## In

`variant` · `sqlparser::ast::FetchPosition::In` · sqlparser 0.62.0

```rust
In
```

Source: `src/ast/mod.rs:6954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use `IN <pos>` position specifier.

<a id="op-66bce4891490ecd0cabbf19f"></a>
## clone

`function` · `sqlparser::ast::FetchPosition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FetchPosition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6947, 17], "end": [6947, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee51ccbd5992b9850726783d"></a>
## cmp

`function` · `sqlparser::ast::FetchPosition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FetchPosition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6947, 51], "end": [6947, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05b1c7378815b95c286cf7d8"></a>
## deserialize

`function` · `sqlparser::ast::FetchPosition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6948, 49], "end": [6948, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5514b2c2ce1f7c5182bbdb26"></a>
## eq

`function` · `sqlparser::ast::FetchPosition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FetchPosition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6947, 24], "end": [6947, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a69e823b43a02a28d2793a8"></a>
## fmt

`function` · `sqlparser::ast::FetchPosition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6957, 1], "end": [6966, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a10f721447f00c0f0724a82"></a>
## fmt

`function` · `sqlparser::ast::FetchPosition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6947, 10], "end": [6947, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85ef144fcca6dc2d3c980564"></a>
## hash

`function` · `sqlparser::ast::FetchPosition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6947, 56], "end": [6947, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d054333a6888ae693720762"></a>
## partial_cmp

`function` · `sqlparser::ast::FetchPosition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FetchPosition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6947, 35], "end": [6947, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db071632e6157e613404f3bb"></a>
## serialize

`function` · `sqlparser::ast::FetchPosition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6948, 38], "end": [6948, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a6dc8a3733f71a868e6b1f1"></a>
## visit

`function` · `sqlparser::ast::FetchPosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6949, 47], "end": [6949, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4909f96fad2361da82e339bc"></a>
## visit

`function` · `sqlparser::ast::FetchPosition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchPosition", "path": "FetchPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6949, 40], "end": [6949, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
