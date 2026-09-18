# `sqlparser::ast::AnalyzeFormatKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AnalyzeFormatKind.json).

<a id="op-bb535f63e4c121c3827f42de"></a>
## AnalyzeFormatKind

`enum` · `sqlparser::ast::AnalyzeFormatKind` · sqlparser 0.62.0

```rust
enum AnalyzeFormatKind
```

Source: `src/ast/mod.rs:8306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

How the `ANALYZE`/`EXPLAIN ANALYZE` format is specified.

<a id="op-ef545499c37b7f7e05e4a9f8"></a>
## Assignment

`variant` · `sqlparser::ast::AnalyzeFormatKind::Assignment` · sqlparser 0.62.0

```rust
Assignment
```

Source: `src/ast/mod.rs:8310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Format provided as an assignment, e.g. `FORMAT=JSON`.

<a id="op-afc260fcf7b311b16e0fbc4e"></a>
## Keyword

`variant` · `sqlparser::ast::AnalyzeFormatKind::Keyword` · sqlparser 0.62.0

```rust
Keyword
```

Source: `src/ast/mod.rs:8308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Format provided as a keyword, e.g. `FORMAT JSON`.

<a id="op-a1af863a07e60d93b7f5e6fa"></a>
## clone

`function` · `sqlparser::ast::AnalyzeFormatKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AnalyzeFormatKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8302, 23], "end": [8302, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7768fcaaccff0446424934d"></a>
## cmp

`function` · `sqlparser::ast::AnalyzeFormatKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AnalyzeFormatKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8302, 57], "end": [8302, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ab879b9baf731a3ffd8694b"></a>
## deserialize

`function` · `sqlparser::ast::AnalyzeFormatKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8303, 49], "end": [8303, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1896a56181268e4d1e112337"></a>
## eq

`function` · `sqlparser::ast::AnalyzeFormatKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AnalyzeFormatKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8302, 30], "end": [8302, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9c0d574b2fd25696210d66a"></a>
## fmt

`function` · `sqlparser::ast::AnalyzeFormatKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8302, 10], "end": [8302, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1dc28d776e139ef98ae1873"></a>
## fmt

`function` · `sqlparser::ast::AnalyzeFormatKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8313, 1], "end": [8320, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a832308a36fedf3b11e27d0"></a>
## hash

`function` · `sqlparser::ast::AnalyzeFormatKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8302, 62], "end": [8302, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0932b85ccfa9938d448290f0"></a>
## partial_cmp

`function` · `sqlparser::ast::AnalyzeFormatKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AnalyzeFormatKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8302, 41], "end": [8302, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71d222553037d989d589e32f"></a>
## serialize

`function` · `sqlparser::ast::AnalyzeFormatKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8303, 38], "end": [8303, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76d106105f9b6ac93abd14dd"></a>
## visit

`function` · `sqlparser::ast::AnalyzeFormatKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8304, 40], "end": [8304, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc3c478ce633aff2f7b9383b"></a>
## visit

`function` · `sqlparser::ast::AnalyzeFormatKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormatKind", "path": "AnalyzeFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8304, 47], "end": [8304, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
