# `sqlparser::ast::TransactionAccessMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TransactionAccessMode.json).

<a id="op-b16dc8dca482ed455aecc493"></a>
## TransactionAccessMode

`enum` · `sqlparser::ast::TransactionAccessMode` · sqlparser 0.62.0

```rust
enum TransactionAccessMode
```

Source: `src/ast/mod.rs:9061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Transaction access mode (READ ONLY / READ WRITE).

<a id="op-3716efaff1462af76c0b38be"></a>
## ReadOnly

`variant` · `sqlparser::ast::TransactionAccessMode::ReadOnly` · sqlparser 0.62.0

```rust
ReadOnly
```

Source: `src/ast/mod.rs:9063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

READ ONLY access mode.

<a id="op-eac4746c62811346d11eb398"></a>
## ReadWrite

`variant` · `sqlparser::ast::TransactionAccessMode::ReadWrite` · sqlparser 0.62.0

```rust
ReadWrite
```

Source: `src/ast/mod.rs:9065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

READ WRITE access mode.

<a id="op-7498b12e778608d15382e56c"></a>
## clone

`function` · `sqlparser::ast::TransactionAccessMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TransactionAccessMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9057, 23], "end": [9057, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-046af02211f899b622c6a55c"></a>
## cmp

`function` · `sqlparser::ast::TransactionAccessMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TransactionAccessMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9057, 57], "end": [9057, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92082e59c567acbc2122db2a"></a>
## deserialize

`function` · `sqlparser::ast::TransactionAccessMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9058, 49], "end": [9058, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9058`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16df577d2f422cfa94c9c260"></a>
## eq

`function` · `sqlparser::ast::TransactionAccessMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TransactionAccessMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9057, 30], "end": [9057, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04432c52c90d73ea1c88465c"></a>
## fmt

`function` · `sqlparser::ast::TransactionAccessMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9068, 1], "end": [9076, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9069`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0d19640c5e06a659e8022f9"></a>
## fmt

`function` · `sqlparser::ast::TransactionAccessMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9057, 10], "end": [9057, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a79faa767676a1f9fc6ed633"></a>
## hash

`function` · `sqlparser::ast::TransactionAccessMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9057, 62], "end": [9057, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3785e4ca99840ecc63b09e70"></a>
## partial_cmp

`function` · `sqlparser::ast::TransactionAccessMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TransactionAccessMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9057, 41], "end": [9057, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-603ca6274345f463d7fe1381"></a>
## serialize

`function` · `sqlparser::ast::TransactionAccessMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9058, 38], "end": [9058, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9058`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30fdd5c709a61d61e9b2f16f"></a>
## visit

`function` · `sqlparser::ast::TransactionAccessMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9059, 40], "end": [9059, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3d1df8c047ab500ff4f22dd"></a>
## visit

`function` · `sqlparser::ast::TransactionAccessMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9059, 47], "end": [9059, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
