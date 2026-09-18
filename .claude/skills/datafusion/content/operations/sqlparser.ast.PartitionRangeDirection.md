# `sqlparser::ast::PartitionRangeDirection`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.PartitionRangeDirection.json).

<a id="op-e76559f5361afae124550e52"></a>
## PartitionRangeDirection

`enum` · `sqlparser::ast::PartitionRangeDirection` · sqlparser 0.62.0

```rust
enum PartitionRangeDirection
```

Source: `src/ast/mod.rs:8785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies which partition the boundary values on table partitioning belongs to.

<a id="op-fe9bb44fafffc4b0cd69cf46"></a>
## Left

`variant` · `sqlparser::ast::PartitionRangeDirection::Left` · sqlparser 0.62.0

```rust
Left
```

Source: `src/ast/mod.rs:8787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LEFT range direction.

<a id="op-719911948a0238349b264a06"></a>
## Right

`variant` · `sqlparser::ast::PartitionRangeDirection::Right` · sqlparser 0.62.0

```rust
Right
```

Source: `src/ast/mod.rs:8789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RIGHT range direction.

<a id="op-66098a32c4bebb753447a241"></a>
## clone

`function` · `sqlparser::ast::PartitionRangeDirection::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> PartitionRangeDirection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8782, 17], "end": [8782, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8aab01019fd699df08368d9f"></a>
## cmp

`function` · `sqlparser::ast::PartitionRangeDirection::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &PartitionRangeDirection) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8782, 57], "end": [8782, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a92815d055b3aeb570d9c21"></a>
## deserialize

`function` · `sqlparser::ast::PartitionRangeDirection::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8783, 49], "end": [8783, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a2e784afe5b4bfca83bbb20"></a>
## eq

`function` · `sqlparser::ast::PartitionRangeDirection::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &PartitionRangeDirection) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8782, 30], "end": [8782, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ee01cc766e3363f787f16a1"></a>
## fmt

`function` · `sqlparser::ast::PartitionRangeDirection::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8782, 10], "end": [8782, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01c49ab2ecea5e77eb9d2202"></a>
## hash

`function` · `sqlparser::ast::PartitionRangeDirection::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8782, 62], "end": [8782, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fb20672ffe0c9c39b4219c2"></a>
## partial_cmp

`function` · `sqlparser::ast::PartitionRangeDirection::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &PartitionRangeDirection) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8782, 41], "end": [8782, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d86e2068c5b568477009b703"></a>
## serialize

`function` · `sqlparser::ast::PartitionRangeDirection::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8783, 38], "end": [8783, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59d6be8309da77de78aa29f1"></a>
## visit

`function` · `sqlparser::ast::PartitionRangeDirection::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8784, 40], "end": [8784, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8784`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-971033992abf1255424cd3c7"></a>
## visit

`function` · `sqlparser::ast::PartitionRangeDirection::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::PartitionRangeDirection", "path": "PartitionRangeDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8784, 47], "end": [8784, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8784`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
