# `sqlparser::ast::DoUpdate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DoUpdate.json).

<a id="op-b998e0a5e21e1abcdd6782a4"></a>
## DoUpdate

`struct` · `sqlparser::ast::DoUpdate` · sqlparser 0.62.0

```rust
struct DoUpdate
```

Source: `src/ast/mod.rs:6758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Details for `DO UPDATE` action of an `ON CONFLICT` clause.

<a id="op-7163a3fe23ed988297c84778"></a>
## assignments

`struct_field` · `sqlparser::ast::DoUpdate::assignments` · sqlparser 0.62.0

```rust
assignments: Vec<Assignment>
```

Source: `src/ast/mod.rs:6760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column assignments to perform on update.

<a id="op-f55ea13991935b16bcf0bf7c"></a>
## clone

`function` · `sqlparser::ast::DoUpdate::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DoUpdate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6754, 17], "end": [6754, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b696dbd31cb46a034974c331"></a>
## cmp

`function` · `sqlparser::ast::DoUpdate::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DoUpdate) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6754, 51], "end": [6754, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d9fa04c49c1617ad436763b"></a>
## deserialize

`function` · `sqlparser::ast::DoUpdate::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6755, 49], "end": [6755, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d30feba4f5cb31a286f7584"></a>
## eq

`function` · `sqlparser::ast::DoUpdate::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DoUpdate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6754, 24], "end": [6754, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b0394b6a75f17ed4e8dd0ef"></a>
## fmt

`function` · `sqlparser::ast::DoUpdate::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6754, 10], "end": [6754, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edc8c1317093f9533d5b37d7"></a>
## hash

`function` · `sqlparser::ast::DoUpdate::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6754, 56], "end": [6754, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d8cd827356d3705a867aa86"></a>
## partial_cmp

`function` · `sqlparser::ast::DoUpdate::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DoUpdate) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6754, 35], "end": [6754, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfba45dca92981a8fcb1c4af"></a>
## selection

`struct_field` · `sqlparser::ast::DoUpdate::selection` · sqlparser 0.62.0

```rust
selection: Option<Expr>
```

Source: `src/ast/mod.rs:6762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional WHERE clause limiting the update.

<a id="op-37d12237b037da710229882d"></a>
## serialize

`function` · `sqlparser::ast::DoUpdate::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6755, 38], "end": [6755, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35f2bc33604409426631d17c"></a>
## span

`function` · `sqlparser::ast::DoUpdate::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "super::DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1417, 1], "end": [1431, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c02d763e85e4585e53f5af3e"></a>
## visit

`function` · `sqlparser::ast::DoUpdate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6756, 47], "end": [6756, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e92bcb95cdf78dca54941703"></a>
## visit

`function` · `sqlparser::ast::DoUpdate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DoUpdate", "path": "DoUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6756, 40], "end": [6756, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
