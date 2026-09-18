# `sqlparser::ast::ListAggOnOverflow`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ListAggOnOverflow.json).

<a id="op-4985ee1d185da88b7520ca0e"></a>
## ListAggOnOverflow

`enum` · `sqlparser::ast::ListAggOnOverflow` · sqlparser 0.62.0

```rust
enum ListAggOnOverflow
```

Source: `src/ast/mod.rs:8391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `ON OVERFLOW` clause of a LISTAGG invocation

<a id="op-b00a6c01b1fb43a5dcd87c9e"></a>
## Error

`variant` · `sqlparser::ast::ListAggOnOverflow::Error` · sqlparser 0.62.0

```rust
Error
```

Source: `src/ast/mod.rs:8393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ON OVERFLOW ERROR`

<a id="op-2559eedd302c0bb403771016"></a>
## Truncate

`variant` · `sqlparser::ast::ListAggOnOverflow::Truncate` · sqlparser 0.62.0

```rust
Truncate
```

Source: `src/ast/mod.rs:8396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ON OVERFLOW TRUNCATE [ <filler> ] WITH[OUT] COUNT`

<a id="op-a4bed3aea1fb81604e6104ca"></a>
## clone

`function` · `sqlparser::ast::ListAggOnOverflow::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ListAggOnOverflow
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8388, 17], "end": [8388, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b76a9315eda4db83cacaf99"></a>
## cmp

`function` · `sqlparser::ast::ListAggOnOverflow::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ListAggOnOverflow) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8388, 51], "end": [8388, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa9113a4ebf2b0539b3c66af"></a>
## deserialize

`function` · `sqlparser::ast::ListAggOnOverflow::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8389, 49], "end": [8389, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6e73930e1e852e20dd6deef"></a>
## eq

`function` · `sqlparser::ast::ListAggOnOverflow::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ListAggOnOverflow) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8388, 24], "end": [8388, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d1601f46206e6086f6787b4"></a>
## fmt

`function` · `sqlparser::ast::ListAggOnOverflow::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8388, 10], "end": [8388, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f58ff99d2dfe8d297a8402d"></a>
## fmt

`function` · `sqlparser::ast::ListAggOnOverflow::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8404, 1], "end": [8423, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f86195cd2e08422bddc1eed1"></a>
## hash

`function` · `sqlparser::ast::ListAggOnOverflow::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8388, 56], "end": [8388, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2ad7e01bbbcfaefdc34dee2"></a>
## partial_cmp

`function` · `sqlparser::ast::ListAggOnOverflow::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ListAggOnOverflow) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8388, 35], "end": [8388, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76af5ffa09717dd8a531643e"></a>
## serialize

`function` · `sqlparser::ast::ListAggOnOverflow::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8389, 38], "end": [8389, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b35bdd15dc065da5130649c"></a>
## visit

`function` · `sqlparser::ast::ListAggOnOverflow::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8390, 47], "end": [8390, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a27e692a04870c86f8017db"></a>
## visit

`function` · `sqlparser::ast::ListAggOnOverflow::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ListAggOnOverflow", "path": "ListAggOnOverflow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8390, 40], "end": [8390, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
