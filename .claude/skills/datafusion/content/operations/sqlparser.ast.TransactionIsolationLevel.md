# `sqlparser::ast::TransactionIsolationLevel`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TransactionIsolationLevel.json).

<a id="op-5368acc0ab793a19c0c454a8"></a>
## TransactionIsolationLevel

`enum` · `sqlparser::ast::TransactionIsolationLevel` · sqlparser 0.62.0

```rust
enum TransactionIsolationLevel
```

Source: `src/ast/mod.rs:9082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Transaction isolation levels.

<a id="op-b156ed61e5391fb04e2dd92b"></a>
## ReadCommitted

`variant` · `sqlparser::ast::TransactionIsolationLevel::ReadCommitted` · sqlparser 0.62.0

```rust
ReadCommitted
```

Source: `src/ast/mod.rs:9086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

READ COMMITTED isolation level.

<a id="op-98876190783ad0a3a8ab1683"></a>
## ReadUncommitted

`variant` · `sqlparser::ast::TransactionIsolationLevel::ReadUncommitted` · sqlparser 0.62.0

```rust
ReadUncommitted
```

Source: `src/ast/mod.rs:9084`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

READ UNCOMMITTED isolation level.

<a id="op-d76360299bda2f484f9e64f8"></a>
## RepeatableRead

`variant` · `sqlparser::ast::TransactionIsolationLevel::RepeatableRead` · sqlparser 0.62.0

```rust
RepeatableRead
```

Source: `src/ast/mod.rs:9088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

REPEATABLE READ isolation level.

<a id="op-d2710b1851d114bc803bdfe7"></a>
## Serializable

`variant` · `sqlparser::ast::TransactionIsolationLevel::Serializable` · sqlparser 0.62.0

```rust
Serializable
```

Source: `src/ast/mod.rs:9090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SERIALIZABLE isolation level.

<a id="op-5f085fd3ac157ade07caee08"></a>
## Snapshot

`variant` · `sqlparser::ast::TransactionIsolationLevel::Snapshot` · sqlparser 0.62.0

```rust
Snapshot
```

Source: `src/ast/mod.rs:9092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SNAPSHOT isolation level.

<a id="op-a9cb466afee100d9eccd6dc7"></a>
## clone

`function` · `sqlparser::ast::TransactionIsolationLevel::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TransactionIsolationLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9078, 23], "end": [9078, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eef0d007e65964f6ccc2a318"></a>
## cmp

`function` · `sqlparser::ast::TransactionIsolationLevel::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TransactionIsolationLevel) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9078, 57], "end": [9078, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2262e2ae333aca92dca84787"></a>
## deserialize

`function` · `sqlparser::ast::TransactionIsolationLevel::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9079, 49], "end": [9079, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a8e2bb8af56ffc5a0e51f4f"></a>
## eq

`function` · `sqlparser::ast::TransactionIsolationLevel::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TransactionIsolationLevel) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9078, 30], "end": [9078, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c14764160854ab1b97c769b"></a>
## fmt

`function` · `sqlparser::ast::TransactionIsolationLevel::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9095, 1], "end": [9106, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9096`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a729c01ab1a3516713768fcd"></a>
## fmt

`function` · `sqlparser::ast::TransactionIsolationLevel::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9078, 10], "end": [9078, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43e09b64c10adfb7a84fa44e"></a>
## hash

`function` · `sqlparser::ast::TransactionIsolationLevel::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9078, 62], "end": [9078, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cdc226d64005551ef8d5df2"></a>
## partial_cmp

`function` · `sqlparser::ast::TransactionIsolationLevel::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TransactionIsolationLevel) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9078, 41], "end": [9078, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f47f2a001172d6cdd36950b9"></a>
## serialize

`function` · `sqlparser::ast::TransactionIsolationLevel::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9079, 38], "end": [9079, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c07487c0b272e24d906c026"></a>
## visit

`function` · `sqlparser::ast::TransactionIsolationLevel::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9080, 47], "end": [9080, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a99f88fb6524b08de7167019"></a>
## visit

`function` · `sqlparser::ast::TransactionIsolationLevel::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9080, 40], "end": [9080, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
