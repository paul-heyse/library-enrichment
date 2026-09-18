# `sqlparser::ast::ddl::PartitionBoundValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.PartitionBoundValue.json).

<a id="op-c53710759513b1e51e73b93d"></a>
## PartitionBoundValue

`enum` · `sqlparser::ast::ddl::PartitionBoundValue` · sqlparser 0.62.0

```rust
enum PartitionBoundValue
```

Source: `src/ast/ddl.rs:3447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A value in a partition bound specification.

Used in RANGE partition bounds where values can be expressions,
MINVALUE (negative infinity), or MAXVALUE (positive infinity).

<a id="op-49a513778a6a18d0f5f5706d"></a>
## Expr

`variant` · `sqlparser::ast::ddl::PartitionBoundValue::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/ddl.rs:3449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression representing a partition bound value.

<a id="op-ec4d49a4a79744bd33874a48"></a>
## MaxValue

`variant` · `sqlparser::ast::ddl::PartitionBoundValue::MaxValue` · sqlparser 0.62.0

```rust
MaxValue
```

Source: `src/ast/ddl.rs:3453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents positive infinity in partition bounds.

<a id="op-0e653cb873b8bdac5c259be2"></a>
## MinValue

`variant` · `sqlparser::ast::ddl::PartitionBoundValue::MinValue` · sqlparser 0.62.0

```rust
MinValue
```

Source: `src/ast/ddl.rs:3451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents negative infinity in partition bounds.

<a id="op-09456afb02467aa7f04ed971"></a>
## clone

`function` · `sqlparser::ast::ddl::PartitionBoundValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> PartitionBoundValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3444, 17], "end": [3444, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d41e5501932318afe8e4c07"></a>
## cmp

`function` · `sqlparser::ast::ddl::PartitionBoundValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &PartitionBoundValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3444, 51], "end": [3444, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95aaace17560c0b587d4fc66"></a>
## deserialize

`function` · `sqlparser::ast::ddl::PartitionBoundValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3445, 49], "end": [3445, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c6eca8354642caea9ec0ec"></a>
## eq

`function` · `sqlparser::ast::ddl::PartitionBoundValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &PartitionBoundValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3444, 24], "end": [3444, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e70c989de9223d4fb089d63"></a>
## fmt

`function` · `sqlparser::ast::ddl::PartitionBoundValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3444, 10], "end": [3444, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92e323ba20ae064fe8aefb63"></a>
## fmt

`function` · `sqlparser::ast::ddl::PartitionBoundValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3456, 1], "end": [3464, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be0bc7b1b69151823d696711"></a>
## hash

`function` · `sqlparser::ast::ddl::PartitionBoundValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3444, 56], "end": [3444, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2871c29a3996b82fc5d719b"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::PartitionBoundValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &PartitionBoundValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3444, 35], "end": [3444, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6365e6a024e6939d5af9f95a"></a>
## serialize

`function` · `sqlparser::ast::ddl::PartitionBoundValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3445, 38], "end": [3445, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27c9d98a14458a4bcdc166f1"></a>
## span

`function` · `sqlparser::ast::ddl::PartitionBoundValue::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "super::PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [657, 1], "end": [666, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10d2960d19d1678eb0b2ec1f"></a>
## visit

`function` · `sqlparser::ast::ddl::PartitionBoundValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3446, 40], "end": [3446, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dabd96e2edc172c2e1d6b27"></a>
## visit

`function` · `sqlparser::ast::ddl::PartitionBoundValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::PartitionBoundValue", "path": "PartitionBoundValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3446, 47], "end": [3446, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
