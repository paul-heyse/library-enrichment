# `sqlparser::ast::query::SetQuantifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SetQuantifier.json).

<a id="op-918eb2f5c404679c5ddb6b0f"></a>
## SetQuantifier

`enum` · `sqlparser::ast::query::SetQuantifier` · sqlparser 0.62.0

```rust
enum SetQuantifier
```

Source: `src/ast/query.rs:268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A quantifier for [SetOperator](../operations/sqlparser.ast.query.SetOperator.md#op-70adcb2bfaab4bb1c2781676).

<a id="op-2b0a1926d4875f3df83a2421"></a>
## All

`variant` · `sqlparser::ast::query::SetQuantifier::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/query.rs:270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALL` quantifier

<a id="op-e59bf1f6ff6e3cfb3ccb5e2e"></a>
## AllByName

`variant` · `sqlparser::ast::query::SetQuantifier::AllByName` · sqlparser 0.62.0

```rust
AllByName
```

Source: `src/ast/query.rs:276`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALL BY NAME` quantifier

<a id="op-92ef262138709aa601a97d12"></a>
## ByName

`variant` · `sqlparser::ast::query::SetQuantifier::ByName` · sqlparser 0.62.0

```rust
ByName
```

Source: `src/ast/query.rs:274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`BY NAME` quantifier

<a id="op-41fbea15e1018a299907b2e0"></a>
## Distinct

`variant` · `sqlparser::ast::query::SetQuantifier::Distinct` · sqlparser 0.62.0

```rust
Distinct
```

Source: `src/ast/query.rs:272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTINCT` quantifier

<a id="op-4e251c5830dd612667948463"></a>
## DistinctByName

`variant` · `sqlparser::ast::query::SetQuantifier::DistinctByName` · sqlparser 0.62.0

```rust
DistinctByName
```

Source: `src/ast/query.rs:278`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTINCT BY NAME` quantifier

<a id="op-87a714b638297a32b9a0fbb0"></a>
## None

`variant` · `sqlparser::ast::query::SetQuantifier::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/query.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No quantifier specified

<a id="op-69c04acab20bf97c6ed37883"></a>
## clone

`function` · `sqlparser::ast::query::SetQuantifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetQuantifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 23], "end": [265, 28], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa4a0c2c1843a4449f88fcd1"></a>
## cmp

`function` · `sqlparser::ast::query::SetQuantifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetQuantifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 57], "end": [265, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4eef211e506c42d075099893"></a>
## deserialize

`function` · `sqlparser::ast::query::SetQuantifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 49], "end": [266, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6229a31bf6fca7d3249e1631"></a>
## eq

`function` · `sqlparser::ast::query::SetQuantifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetQuantifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 30], "end": [265, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08439fe2f3ba470dfd0dfee6"></a>
## fmt

`function` · `sqlparser::ast::query::SetQuantifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [294, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd80c9cd177e284a4c327e9a"></a>
## fmt

`function` · `sqlparser::ast::query::SetQuantifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 10], "end": [265, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33c7ef063b346726c8cad274"></a>
## hash

`function` · `sqlparser::ast::query::SetQuantifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 62], "end": [265, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5017b32e5015991565a1df1"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SetQuantifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetQuantifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 41], "end": [265, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e0f4e4993ec843b64098936"></a>
## serialize

`function` · `sqlparser::ast::query::SetQuantifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 38], "end": [266, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09b246fb1da7701c195d4bb2"></a>
## visit

`function` · `sqlparser::ast::query::SetQuantifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [267, 47], "end": [267, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-106c1ce9b201b8fa44b638d2"></a>
## visit

`function` · `sqlparser::ast::query::SetQuantifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [267, 40], "end": [267, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
