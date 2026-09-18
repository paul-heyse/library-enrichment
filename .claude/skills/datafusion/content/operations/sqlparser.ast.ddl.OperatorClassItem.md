# `sqlparser::ast::ddl::OperatorClassItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorClassItem.json).

<a id="op-f3300d0a0cdd511c5ed4143b"></a>
## OperatorClassItem

`enum` · `sqlparser::ast::ddl::OperatorClassItem` · sqlparser 0.62.0

```rust
enum OperatorClassItem
```

Source: `src/ast/ddl.rs:4871`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An item in a CREATE OPERATOR CLASS statement

<a id="op-4c526b9676fb26a625835328"></a>
## Function

`variant` · `sqlparser::ast::ddl::OperatorClassItem::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/ddl.rs:4884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FUNCTION` clause describing a support function for the operator class.

<a id="op-d18189292dde700429b679d1"></a>
## Operator

`variant` · `sqlparser::ast::ddl::OperatorClassItem::Operator` · sqlparser 0.62.0

```rust
Operator
```

Source: `src/ast/ddl.rs:4873`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OPERATOR` clause describing a specific operator implementation.

<a id="op-307b1e78c68724ae27816e58"></a>
## Storage

`variant` · `sqlparser::ast::ddl::OperatorClassItem::Storage` · sqlparser 0.62.0

```rust
Storage
```

Source: `src/ast/ddl.rs:4895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`STORAGE` clause specifying the storage type.

<a id="op-a35b11961b91fc15bec48917"></a>
## clone

`function` · `sqlparser::ast::ddl::OperatorClassItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OperatorClassItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4868, 17], "end": [4868, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41040c45bd33353711733982"></a>
## cmp

`function` · `sqlparser::ast::ddl::OperatorClassItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OperatorClassItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4868, 51], "end": [4868, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb163faf2366dbcdca4ad073"></a>
## deserialize

`function` · `sqlparser::ast::ddl::OperatorClassItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4869, 49], "end": [4869, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37a7273538d6b33252159315"></a>
## eq

`function` · `sqlparser::ast::ddl::OperatorClassItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OperatorClassItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4868, 24], "end": [4868, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-600ab48a70d9f40028d368c1"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorClassItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4915, 1], "end": [4954, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69df2d593f0ae1b98899eacc"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorClassItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4868, 10], "end": [4868, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dff6407601a6058deeabf466"></a>
## hash

`function` · `sqlparser::ast::ddl::OperatorClassItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4868, 56], "end": [4868, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-008ed5b34b5d41933561ca9e"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::OperatorClassItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OperatorClassItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4868, 35], "end": [4868, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac08c75d74b53c05b54f376e"></a>
## serialize

`function` · `sqlparser::ast::ddl::OperatorClassItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4869, 38], "end": [4869, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30ae84f2a4c7dba8672a45b8"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorClassItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4870, 47], "end": [4870, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4870`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9fff5ef2d05174e1c62b0ec"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorClassItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorClassItem", "path": "OperatorClassItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4870, 40], "end": [4870, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4870`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
