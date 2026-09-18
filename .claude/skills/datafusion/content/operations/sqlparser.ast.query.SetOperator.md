# `sqlparser::ast::query::SetOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SetOperator.json).

<a id="op-70adcb2bfaab4bb1c2781676"></a>
## SetOperator

`enum` · `sqlparser::ast::query::SetOperator` · sqlparser 0.62.0

```rust
enum SetOperator
```

Source: `src/ast/query.rs:240`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A set operator for combining two `SetExpr`s.

<a id="op-753ad4ab42888ab38af495f9"></a>
## Except

`variant` · `sqlparser::ast::query::SetOperator::Except` · sqlparser 0.62.0

```rust
Except
```

Source: `src/ast/query.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXCEPT` set operator

<a id="op-45744a8683431e523da526bd"></a>
## Intersect

`variant` · `sqlparser::ast::query::SetOperator::Intersect` · sqlparser 0.62.0

```rust
Intersect
```

Source: `src/ast/query.rs:246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INTERSECT` set operator

<a id="op-d6c9598c9c377b6233a7c182"></a>
## Minus

`variant` · `sqlparser::ast::query::SetOperator::Minus` · sqlparser 0.62.0

```rust
Minus
```

Source: `src/ast/query.rs:248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MINUS` set operator (non-standard)

<a id="op-54ba26c381ba1928b8a6cdab"></a>
## Union

`variant` · `sqlparser::ast::query::SetOperator::Union` · sqlparser 0.62.0

```rust
Union
```

Source: `src/ast/query.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`UNION` set operator

<a id="op-39f6a569c898f20ad237a92c"></a>
## clone

`function` · `sqlparser::ast::query::SetOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 23], "end": [236, 28], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de821fd6e5791deba02b7aa5"></a>
## cmp

`function` · `sqlparser::ast::query::SetOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 57], "end": [236, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a79040a2b2e148f74c86aa8"></a>
## deserialize

`function` · `sqlparser::ast::query::SetOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 49], "end": [237, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41a20486324d07c28674e3a0"></a>
## eq

`function` · `sqlparser::ast::query::SetOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 30], "end": [236, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-398f7e7a26678d14840f2cc9"></a>
## fmt

`function` · `sqlparser::ast::query::SetOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [260, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:252`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95f6d869e64f1092d4fee2c9"></a>
## fmt

`function` · `sqlparser::ast::query::SetOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 10], "end": [236, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7066cd570109259baf5821f8"></a>
## hash

`function` · `sqlparser::ast::query::SetOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 62], "end": [236, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25f59ce06fa0dae73cd1afd0"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SetOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 41], "end": [236, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-428e8ec388f2e8027b0238b8"></a>
## serialize

`function` · `sqlparser::ast::query::SetOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 38], "end": [237, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1378eeec47e2d06acb3233c7"></a>
## visit

`function` · `sqlparser::ast::query::SetOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 47], "end": [238, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-992e932fe0913d98fcbb37e6"></a>
## visit

`function` · `sqlparser::ast::query::SetOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetOperator", "path": "SetOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 40], "end": [238, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
