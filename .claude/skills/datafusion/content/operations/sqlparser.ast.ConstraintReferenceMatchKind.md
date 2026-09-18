# `sqlparser::ast::ConstraintReferenceMatchKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ConstraintReferenceMatchKind.json).

<a id="op-533d043a021901c5822dea2f"></a>
## ConstraintReferenceMatchKind

`enum` · `sqlparser::ast::ConstraintReferenceMatchKind` · sqlparser 0.62.0

```rust
enum ConstraintReferenceMatchKind
```

Source: `src/ast/mod.rs:771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MATCH` type for constraint references

See: <https://www.postgresql.org/docs/current/sql-createtable.html#SQL-CREATETABLE-PARMS-REFERENCES>

<a id="op-a59925683ab4579461fc6dab"></a>
## Full

`variant` · `sqlparser::ast::ConstraintReferenceMatchKind::Full` · sqlparser 0.62.0

```rust
Full
```

Source: `src/ast/mod.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MATCH FULL`

<a id="op-c548d678e2f2d43eca8145ae"></a>
## Partial

`variant` · `sqlparser::ast::ConstraintReferenceMatchKind::Partial` · sqlparser 0.62.0

```rust
Partial
```

Source: `src/ast/mod.rs:775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MATCH PARTIAL`

<a id="op-86d41d13fe0e65380b29d5da"></a>
## Simple

`variant` · `sqlparser::ast::ConstraintReferenceMatchKind::Simple` · sqlparser 0.62.0

```rust
Simple
```

Source: `src/ast/mod.rs:777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MATCH SIMPLE`

<a id="op-723de338bb7928413d94c411"></a>
## clone

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ConstraintReferenceMatchKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [768, 23], "end": [768, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-832353a2fe9e8be853dcca4b"></a>
## cmp

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ConstraintReferenceMatchKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [768, 57], "end": [768, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6737e5dd11025abe04022638"></a>
## deserialize

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [769, 49], "end": [769, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f543dff8624fda18177706a5"></a>
## eq

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ConstraintReferenceMatchKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [768, 30], "end": [768, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1672c96fda73164b66a2833e"></a>
## fmt

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [780, 1], "end": [788, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2702b798257f7835cdf3c702"></a>
## fmt

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [768, 10], "end": [768, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e29b4656126e2131b208e8ca"></a>
## hash

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [768, 62], "end": [768, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb75ad29ea7976fdb8d02933"></a>
## partial_cmp

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ConstraintReferenceMatchKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [768, 41], "end": [768, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a5379c0d6e67b22464677d0"></a>
## serialize

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [769, 38], "end": [769, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31d5076fa7ef0046e3042fa1"></a>
## visit

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [770, 40], "end": [770, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:770`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f7f1ea851a7c1c8999daa17"></a>
## visit

`function` · `sqlparser::ast::ConstraintReferenceMatchKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConstraintReferenceMatchKind", "path": "ConstraintReferenceMatchKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [770, 47], "end": [770, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:770`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
