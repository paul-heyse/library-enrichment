# `sqlparser::ast::query::EmptyMatchesMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.EmptyMatchesMode.json).

<a id="op-8b1d6d6357185363bf443678"></a>
## EmptyMatchesMode

`enum` · `sqlparser::ast::query::EmptyMatchesMode` · sqlparser 0.62.0

```rust
enum EmptyMatchesMode
```

Source: `src/ast/query.rs:2063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The mode for handling empty matches in a `MATCH_RECOGNIZE` operation.

<a id="op-397f09171a2ba2aebb5e1123"></a>
## Omit

`variant` · `sqlparser::ast::query::EmptyMatchesMode::Omit` · sqlparser 0.62.0

```rust
Omit
```

Source: `src/ast/query.rs:2067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OMIT EMPTY MATCHES`

<a id="op-a76d29da780913bf2add8fe3"></a>
## Show

`variant` · `sqlparser::ast::query::EmptyMatchesMode::Show` · sqlparser 0.62.0

```rust
Show
```

Source: `src/ast/query.rs:2065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SHOW EMPTY MATCHES`

<a id="op-6ccb3598ddfed824f51ed6f1"></a>
## WithUnmatched

`variant` · `sqlparser::ast::query::EmptyMatchesMode::WithUnmatched` · sqlparser 0.62.0

```rust
WithUnmatched
```

Source: `src/ast/query.rs:2069`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WITH UNMATCHED ROWS`

<a id="op-f14e94c9e6c71005d4bd2474"></a>
## clone

`function` · `sqlparser::ast::query::EmptyMatchesMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> EmptyMatchesMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2059, 17], "end": [2059, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43f68d11d551c68b2761fc5c"></a>
## cmp

`function` · `sqlparser::ast::query::EmptyMatchesMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &EmptyMatchesMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2059, 51], "end": [2059, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4d9513a64b4857d392c1aa5"></a>
## deserialize

`function` · `sqlparser::ast::query::EmptyMatchesMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2060, 49], "end": [2060, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecb31d00a5c99c7f52dea1dc"></a>
## eq

`function` · `sqlparser::ast::query::EmptyMatchesMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &EmptyMatchesMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2059, 24], "end": [2059, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fbe0133154863765c2fda43"></a>
## fmt

`function` · `sqlparser::ast::query::EmptyMatchesMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2072, 1], "end": [2080, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96cd39ce31fd45c016b51b62"></a>
## fmt

`function` · `sqlparser::ast::query::EmptyMatchesMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2059, 10], "end": [2059, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-690ce60a9a747b40f911c985"></a>
## hash

`function` · `sqlparser::ast::query::EmptyMatchesMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2059, 56], "end": [2059, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-422e164c0226bbd316a69e0f"></a>
## partial_cmp

`function` · `sqlparser::ast::query::EmptyMatchesMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &EmptyMatchesMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2059, 35], "end": [2059, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7538bd719715033ed453061"></a>
## serialize

`function` · `sqlparser::ast::query::EmptyMatchesMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2060, 38], "end": [2060, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3ec8b84e7adec62a24bcbd6"></a>
## visit

`function` · `sqlparser::ast::query::EmptyMatchesMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2061, 47], "end": [2061, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ad2b3057d21a3d367beb31"></a>
## visit

`function` · `sqlparser::ast::query::EmptyMatchesMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::EmptyMatchesMode", "path": "EmptyMatchesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2061, 40], "end": [2061, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
