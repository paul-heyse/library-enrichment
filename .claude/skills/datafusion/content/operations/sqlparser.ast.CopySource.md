# `sqlparser::ast::CopySource`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CopySource.json).

<a id="op-06ffbb3f6cb50c07982d8ec9"></a>
## CopySource

`enum` · `sqlparser::ast::CopySource` · sqlparser 0.62.0

```rust
enum CopySource
```

Source: `src/ast/mod.rs:9254`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Source for the `COPY` command: a table or a query.

<a id="op-a8bac27a8eb5e9c77b40bf55"></a>
## Query

`variant` · `sqlparser::ast::CopySource::Query` · sqlparser 0.62.0

```rust
Query
```

Source: `src/ast/mod.rs:9264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Copy from the results of a query.

<a id="op-428dec3b7076f66b829298d0"></a>
## Table

`variant` · `sqlparser::ast::CopySource::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/mod.rs:9256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Copy from a table with optional column list.

<a id="op-7f7016271a79b83770eea88d"></a>
## clone

`function` · `sqlparser::ast::CopySource::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CopySource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9250, 17], "end": [9250, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c1196a70993dfec3619f753"></a>
## cmp

`function` · `sqlparser::ast::CopySource::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CopySource) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9250, 51], "end": [9250, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b18f630c850539d17082857"></a>
## deserialize

`function` · `sqlparser::ast::CopySource::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9251, 49], "end": [9251, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-792d8a5ae0f933931c53053a"></a>
## eq

`function` · `sqlparser::ast::CopySource::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CopySource) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9250, 24], "end": [9250, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f90648e66e124350c97fee7c"></a>
## fmt

`function` · `sqlparser::ast::CopySource::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9250, 10], "end": [9250, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdb648749d8a855168e1825b"></a>
## hash

`function` · `sqlparser::ast::CopySource::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9250, 56], "end": [9250, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe8155cc7bad68d50b239c0d"></a>
## partial_cmp

`function` · `sqlparser::ast::CopySource::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CopySource) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9250, 35], "end": [9250, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e9ba8cd28489fe4427ad685"></a>
## serialize

`function` · `sqlparser::ast::CopySource::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9251, 38], "end": [9251, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b88e3e308d3c34a9c29e4d3c"></a>
## span

`function` · `sqlparser::ast::CopySource::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "super::CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [907, 1], "end": [919, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-794afb5025d50867c9e1b133"></a>
## visit

`function` · `sqlparser::ast::CopySource::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9252, 47], "end": [9252, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9252`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdcba1f6f0b40b13b7fb1070"></a>
## visit

`function` · `sqlparser::ast::CopySource::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopySource", "path": "CopySource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9252, 40], "end": [9252, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9252`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
