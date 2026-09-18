# `sqlparser::ast::query::SymbolDefinition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SymbolDefinition.json).

<a id="op-13c179bc0561c23bed807898"></a>
## SymbolDefinition

`struct` · `sqlparser::ast::query::SymbolDefinition` · sqlparser 0.62.0

```rust
struct SymbolDefinition
```

Source: `src/ast/query.rs:2089`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A symbol defined in a `MATCH_RECOGNIZE` operation.

See <https://docs.snowflake.com/en/sql-reference/constructs/match_recognize#define-defining-symbols>.
A symbol defined in a `MATCH_RECOGNIZE` operation.

<a id="op-90b7596dcd002b2061ca5879"></a>
## clone

`function` · `sqlparser::ast::query::SymbolDefinition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SymbolDefinition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2085, 17], "end": [2085, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f25b25e3b65c0c3b9c8e433"></a>
## cmp

`function` · `sqlparser::ast::query::SymbolDefinition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SymbolDefinition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2085, 51], "end": [2085, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4eb05bab90ddb516ca927b26"></a>
## definition

`struct_field` · `sqlparser::ast::query::SymbolDefinition::definition` · sqlparser 0.62.0

```rust
definition: Expr
```

Source: `src/ast/query.rs:2093`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression defining the symbol.

<a id="op-91777df27e9cf936dbbf2f61"></a>
## deserialize

`function` · `sqlparser::ast::query::SymbolDefinition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2086, 49], "end": [2086, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6970c6832926cb8b3da93757"></a>
## eq

`function` · `sqlparser::ast::query::SymbolDefinition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SymbolDefinition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2085, 24], "end": [2085, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d33571d28dc619f612fca74"></a>
## fmt

`function` · `sqlparser::ast::query::SymbolDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2085, 10], "end": [2085, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd2441f29bd290d8ba586d28"></a>
## fmt

`function` · `sqlparser::ast::query::SymbolDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2096, 1], "end": [2100, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2097`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cef74389ae84f19704ca951b"></a>
## hash

`function` · `sqlparser::ast::query::SymbolDefinition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2085, 56], "end": [2085, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a14ce944f27def569bc2dd0"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SymbolDefinition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SymbolDefinition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2085, 35], "end": [2085, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f480f850b5ba1a52d3ce8cda"></a>
## serialize

`function` · `sqlparser::ast::query::SymbolDefinition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2086, 38], "end": [2086, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-808077e4ab569a94c38d6009"></a>
## span

`function` · `sqlparser::ast::query::SymbolDefinition::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "super::SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2112, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd84f26184aac30fb1280a65"></a>
## symbol

`struct_field` · `sqlparser::ast::query::SymbolDefinition::symbol` · sqlparser 0.62.0

```rust
symbol: Ident
```

Source: `src/ast/query.rs:2091`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The symbol identifier.

<a id="op-40fe96249466d3a24125d47b"></a>
## visit

`function` · `sqlparser::ast::query::SymbolDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2087, 47], "end": [2087, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e85de7a020fa2fd77057788"></a>
## visit

`function` · `sqlparser::ast::query::SymbolDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SymbolDefinition", "path": "SymbolDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2087, 40], "end": [2087, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
