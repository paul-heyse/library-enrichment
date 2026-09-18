# `sqlparser::ast::OnConflictAction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OnConflictAction.json).

<a id="op-e32dec377d36ddd2e0fd2917"></a>
## OnConflictAction

`enum` · `sqlparser::ast::OnConflictAction` · sqlparser 0.62.0

```rust
enum OnConflictAction
```

Source: `src/ast/mod.rs:6747`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Action to perform when an `ON CONFLICT` target is matched.

<a id="op-323fdbd59149b920c9c78d78"></a>
## DoNothing

`variant` · `sqlparser::ast::OnConflictAction::DoNothing` · sqlparser 0.62.0

```rust
DoNothing
```

Source: `src/ast/mod.rs:6749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Do nothing on conflict.

<a id="op-29e22e2a8e75e930014d6068"></a>
## DoUpdate

`variant` · `sqlparser::ast::OnConflictAction::DoUpdate` · sqlparser 0.62.0

```rust
DoUpdate
```

Source: `src/ast/mod.rs:6751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Perform an update on conflict.

<a id="op-e12b8e3f166d01cf3f6c21fc"></a>
## clone

`function` · `sqlparser::ast::OnConflictAction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OnConflictAction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6743, 17], "end": [6743, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f901390515eab384db2adee"></a>
## cmp

`function` · `sqlparser::ast::OnConflictAction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OnConflictAction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6743, 51], "end": [6743, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e2044df96fd644bb1899245"></a>
## deserialize

`function` · `sqlparser::ast::OnConflictAction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6744, 49], "end": [6744, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84adea197c3a4f82aed49468"></a>
## eq

`function` · `sqlparser::ast::OnConflictAction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OnConflictAction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6743, 24], "end": [6743, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adee9ad1c56f5d2df3d6355e"></a>
## fmt

`function` · `sqlparser::ast::OnConflictAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6794, 1], "end": [6814, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e07c18c652ead9d2635d896b"></a>
## fmt

`function` · `sqlparser::ast::OnConflictAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6743, 10], "end": [6743, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c29b3a8333d2e348a68d8ab7"></a>
## hash

`function` · `sqlparser::ast::OnConflictAction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6743, 56], "end": [6743, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b53dcbbba1bb7ed6d434e8ba"></a>
## partial_cmp

`function` · `sqlparser::ast::OnConflictAction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OnConflictAction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6743, 35], "end": [6743, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5142cfbcec4d9ec965357c46"></a>
## serialize

`function` · `sqlparser::ast::OnConflictAction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6744, 38], "end": [6744, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ceba17bc54ac1b3b3524e77"></a>
## span

`function` · `sqlparser::ast::OnConflictAction::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "super::OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 1], "end": [1415, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46963c68419c67ddf38c9d65"></a>
## visit

`function` · `sqlparser::ast::OnConflictAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6745, 40], "end": [6745, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6745`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49507fe40842d6df6e234f18"></a>
## visit

`function` · `sqlparser::ast::OnConflictAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflictAction", "path": "OnConflictAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6745, 47], "end": [6745, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6745`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
