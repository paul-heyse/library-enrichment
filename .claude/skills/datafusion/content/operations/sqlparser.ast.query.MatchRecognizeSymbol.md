# `sqlparser::ast::query::MatchRecognizeSymbol`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.MatchRecognizeSymbol.json).

<a id="op-95b8a73d45dd6dab8b775c49"></a>
## MatchRecognizeSymbol

`enum` · `sqlparser::ast::query::MatchRecognizeSymbol` · sqlparser 0.62.0

```rust
enum MatchRecognizeSymbol
```

Source: `src/ast/query.rs:2106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A symbol in a `MATCH_RECOGNIZE` pattern.

<a id="op-bf824994ea7fe2d127c0c1cc"></a>
## End

`variant` · `sqlparser::ast::query::MatchRecognizeSymbol::End` · sqlparser 0.62.0

```rust
End
```

Source: `src/ast/query.rs:2112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A virtual symbol representing the end of the partition (`$`).

<a id="op-05ec95ab3fd41daa55f68b91"></a>
## Named

`variant` · `sqlparser::ast::query::MatchRecognizeSymbol::Named` · sqlparser 0.62.0

```rust
Named
```

Source: `src/ast/query.rs:2108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A named symbol, e.g. `S1`.

<a id="op-21deba602c1cc50317728f6a"></a>
## Start

`variant` · `sqlparser::ast::query::MatchRecognizeSymbol::Start` · sqlparser 0.62.0

```rust
Start
```

Source: `src/ast/query.rs:2110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A virtual symbol representing the start of the of partition (`^`).

<a id="op-69a6e67c2356b6594574f721"></a>
## clone

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MatchRecognizeSymbol
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2103, 17], "end": [2103, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b019a6c60c4da62e503b39fb"></a>
## cmp

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MatchRecognizeSymbol) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2103, 51], "end": [2103, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bd00596c395da06b5414f68"></a>
## deserialize

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2104, 49], "end": [2104, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7589b2268e5735a7dd5ba642"></a>
## eq

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MatchRecognizeSymbol) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2103, 24], "end": [2103, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-497c6ab424e810563ace87ee"></a>
## fmt

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2115, 1], "end": [2123, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f877e782cecd95b144cbcf61"></a>
## fmt

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2103, 10], "end": [2103, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22c4abd329b58a50126b75a9"></a>
## hash

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2103, 56], "end": [2103, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9337b6517f7d36fb2cdb44db"></a>
## partial_cmp

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MatchRecognizeSymbol) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2103, 35], "end": [2103, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd8108fa3ea70db945c254c9"></a>
## serialize

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2104, 38], "end": [2104, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35f381a9a0aa658dd6fa9e34"></a>
## visit

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2105, 40], "end": [2105, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc93e2f85bd067768f282a73"></a>
## visit

`function` · `sqlparser::ast::query::MatchRecognizeSymbol::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizeSymbol", "path": "MatchRecognizeSymbol"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2105, 47], "end": [2105, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
