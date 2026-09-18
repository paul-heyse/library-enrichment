# `sqlparser::ast::AnalyzeFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AnalyzeFormat.json).

<a id="op-1bacd3da5b72336cdac27b92"></a>
## AnalyzeFormat

`enum` · `sqlparser::ast::AnalyzeFormat` · sqlparser 0.62.0

```rust
enum AnalyzeFormat
```

Source: `src/ast/mod.rs:8326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Output formats supported for `ANALYZE`/`EXPLAIN ANALYZE`.

<a id="op-0f483e6a216eb2ec04b93ee8"></a>
## GRAPHVIZ

`variant` · `sqlparser::ast::AnalyzeFormat::GRAPHVIZ` · sqlparser 0.62.0

```rust
GRAPHVIZ
```

Source: `src/ast/mod.rs:8330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Graphviz DOT format.

<a id="op-a3d02ec7b6a2d88a8edc4c7c"></a>
## JSON

`variant` · `sqlparser::ast::AnalyzeFormat::JSON` · sqlparser 0.62.0

```rust
JSON
```

Source: `src/ast/mod.rs:8332`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

JSON format.

<a id="op-9498aac4011c98dce95ec174"></a>
## TEXT

`variant` · `sqlparser::ast::AnalyzeFormat::TEXT` · sqlparser 0.62.0

```rust
TEXT
```

Source: `src/ast/mod.rs:8328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Plain text format.

<a id="op-3a392c1e99c77ae3dd3714e5"></a>
## TRADITIONAL

`variant` · `sqlparser::ast::AnalyzeFormat::TRADITIONAL` · sqlparser 0.62.0

```rust
TRADITIONAL
```

Source: `src/ast/mod.rs:8334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Traditional explain output.

<a id="op-dcbb54bea92a90fef1fd8ca2"></a>
## TREE

`variant` · `sqlparser::ast::AnalyzeFormat::TREE` · sqlparser 0.62.0

```rust
TREE
```

Source: `src/ast/mod.rs:8336`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tree-style explain output.

<a id="op-a38be3a6f87cba0d0a00a916"></a>
## clone

`function` · `sqlparser::ast::AnalyzeFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AnalyzeFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8322, 23], "end": [8322, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8da83ebd45c7e5cf990d2e79"></a>
## cmp

`function` · `sqlparser::ast::AnalyzeFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AnalyzeFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8322, 57], "end": [8322, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13155541cd6e401c29867c0c"></a>
## deserialize

`function` · `sqlparser::ast::AnalyzeFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8323, 49], "end": [8323, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00f92eb7288e7e8a7b53a4dd"></a>
## eq

`function` · `sqlparser::ast::AnalyzeFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AnalyzeFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8322, 30], "end": [8322, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a43156db1317c3f0a216eaab"></a>
## fmt

`function` · `sqlparser::ast::AnalyzeFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8339, 1], "end": [8349, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce875a527359b6a485b540c6"></a>
## fmt

`function` · `sqlparser::ast::AnalyzeFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8322, 10], "end": [8322, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9ebc9f8f83be47c9dc7616c"></a>
## hash

`function` · `sqlparser::ast::AnalyzeFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8322, 62], "end": [8322, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e0b61d2bb6f47c1f4fb552d"></a>
## partial_cmp

`function` · `sqlparser::ast::AnalyzeFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AnalyzeFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8322, 41], "end": [8322, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5a8cf4ac18a57300b261c1e"></a>
## serialize

`function` · `sqlparser::ast::AnalyzeFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8323, 38], "end": [8323, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75ab31d6f6d7f0cacbcc0bc7"></a>
## visit

`function` · `sqlparser::ast::AnalyzeFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8324, 47], "end": [8324, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-872edfa5be00ad8ee195476c"></a>
## visit

`function` · `sqlparser::ast::AnalyzeFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AnalyzeFormat", "path": "AnalyzeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8324, 40], "end": [8324, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
