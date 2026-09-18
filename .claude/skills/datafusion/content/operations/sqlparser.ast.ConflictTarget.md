# `sqlparser::ast::ConflictTarget`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ConflictTarget.json).

<a id="op-4170d149ca4820d96190617b"></a>
## ConflictTarget

`enum` · `sqlparser::ast::ConflictTarget` · sqlparser 0.62.0

```rust
enum ConflictTarget
```

Source: `src/ast/mod.rs:6737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target specification for an `ON CONFLICT` clause.

<a id="op-b817ebeabac2cca8a4bf4450"></a>
## Columns

`variant` · `sqlparser::ast::ConflictTarget::Columns` · sqlparser 0.62.0

```rust
Columns
```

Source: `src/ast/mod.rs:6739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target specified as a list of columns.

<a id="op-11872423f5dfa22418790ef0"></a>
## OnConstraint

`variant` · `sqlparser::ast::ConflictTarget::OnConstraint` · sqlparser 0.62.0

```rust
OnConstraint
```

Source: `src/ast/mod.rs:6741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target specified as a named constraint.

<a id="op-1545fad508b932dbdd6e5d9c"></a>
## clone

`function` · `sqlparser::ast::ConflictTarget::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ConflictTarget
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6733, 17], "end": [6733, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8fb2ccaec41649b7ed96d53"></a>
## cmp

`function` · `sqlparser::ast::ConflictTarget::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ConflictTarget) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6733, 51], "end": [6733, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1f7617eddabaa38443cb47c"></a>
## deserialize

`function` · `sqlparser::ast::ConflictTarget::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6734, 49], "end": [6734, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05f6abeb7bfb3b4af4ee3339"></a>
## eq

`function` · `sqlparser::ast::ConflictTarget::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ConflictTarget) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6733, 24], "end": [6733, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e33835c950f5618d80d2d4c1"></a>
## fmt

`function` · `sqlparser::ast::ConflictTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6786, 1], "end": [6793, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f15432bc6c3b12a030b5ebc8"></a>
## fmt

`function` · `sqlparser::ast::ConflictTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6733, 10], "end": [6733, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26ddbfe861a73215ac8359fb"></a>
## hash

`function` · `sqlparser::ast::ConflictTarget::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6733, 56], "end": [6733, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d8afca4de5907d20a84ec3c"></a>
## partial_cmp

`function` · `sqlparser::ast::ConflictTarget::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ConflictTarget) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6733, 35], "end": [6733, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2903d2bd7eb81066ea3ab38e"></a>
## serialize

`function` · `sqlparser::ast::ConflictTarget::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6734, 38], "end": [6734, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26f3c8c721335650bc78d8bb"></a>
## span

`function` · `sqlparser::ast::ConflictTarget::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "super::ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1395, 1], "end": [1402, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e43d2d652020eeb804621d4"></a>
## visit

`function` · `sqlparser::ast::ConflictTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6735, 40], "end": [6735, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63af611f640d7c2116ab9554"></a>
## visit

`function` · `sqlparser::ast::ConflictTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConflictTarget", "path": "ConflictTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6735, 47], "end": [6735, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
