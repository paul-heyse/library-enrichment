# `sqlparser::ast::BeginTransactionKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.BeginTransactionKind.json).

<a id="op-35f4c5b5e14e0eaee5eb7721"></a>
## BeginTransactionKind

`enum` · `sqlparser::ast::BeginTransactionKind` · sqlparser 0.62.0

```rust
enum BeginTransactionKind
```

Source: `src/ast/mod.rs:6655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Transaction started with [ TRANSACTION | WORK | TRAN ]

<a id="op-91877bb131fb2acb4351f484"></a>
## Tran

`variant` · `sqlparser::ast::BeginTransactionKind::Tran` · sqlparser 0.62.0

```rust
Tran
```

Source: `src/ast/mod.rs:6662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MSSQL shorthand `TRAN` keyword.
See <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/begin-transaction-transact-sql>

<a id="op-aad3023bfc81d768862bca21"></a>
## Transaction

`variant` · `sqlparser::ast::BeginTransactionKind::Transaction` · sqlparser 0.62.0

```rust
Transaction
```

Source: `src/ast/mod.rs:6657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Standard `TRANSACTION` keyword.

<a id="op-22ec6ec22a862230a58f08c3"></a>
## Work

`variant` · `sqlparser::ast::BeginTransactionKind::Work` · sqlparser 0.62.0

```rust
Work
```

Source: `src/ast/mod.rs:6659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Alternate `WORK` keyword.

<a id="op-7f4e78c1db106dae40296d3d"></a>
## clone

`function` · `sqlparser::ast::BeginTransactionKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> BeginTransactionKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6652, 17], "end": [6652, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b21757de3583806d3e752d60"></a>
## cmp

`function` · `sqlparser::ast::BeginTransactionKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &BeginTransactionKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6652, 51], "end": [6652, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b915ebea6e2df5c7be60b98"></a>
## deserialize

`function` · `sqlparser::ast::BeginTransactionKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6653, 49], "end": [6653, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4103d58f9628b13b81fc3114"></a>
## eq

`function` · `sqlparser::ast::BeginTransactionKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &BeginTransactionKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6652, 24], "end": [6652, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3309dbf37452c5d4efa24dd6"></a>
## fmt

`function` · `sqlparser::ast::BeginTransactionKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6652, 10], "end": [6652, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe84b5c19d431f1962e9abc6"></a>
## fmt

`function` · `sqlparser::ast::BeginTransactionKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6665, 1], "end": [6673, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6666`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72e6e2a11266777bb5f30729"></a>
## hash

`function` · `sqlparser::ast::BeginTransactionKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6652, 56], "end": [6652, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-baabc3caefb1a74256991096"></a>
## partial_cmp

`function` · `sqlparser::ast::BeginTransactionKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &BeginTransactionKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6652, 35], "end": [6652, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b67bd24b65ca24f1412d4b7"></a>
## serialize

`function` · `sqlparser::ast::BeginTransactionKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6653, 38], "end": [6653, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c927b1ecd1fefa4bad4c865"></a>
## visit

`function` · `sqlparser::ast::BeginTransactionKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6654, 40], "end": [6654, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f140d1afbaf0a6872035ef95"></a>
## visit

`function` · `sqlparser::ast::BeginTransactionKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginTransactionKind", "path": "BeginTransactionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6654, 47], "end": [6654, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
