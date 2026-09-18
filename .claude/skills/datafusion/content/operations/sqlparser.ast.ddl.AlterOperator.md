# `sqlparser::ast::ddl::AlterOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterOperator.json).

<a id="op-a05db56051abf8c6692f0daf"></a>
## AlterOperator

`struct` · `sqlparser::ast::ddl::AlterOperator` · sqlparser 0.62.0

```rust
struct AlterOperator
```

Source: `src/ast/ddl.rs:1168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALTER OPERATOR` statement
See <https://www.postgresql.org/docs/current/sql-alteroperator.html>

<a id="op-877f523b1bf572d8916d067f"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1165, 17], "end": [1165, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9554c508a9c63085acde12e"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1165, 51], "end": [1165, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d909bb9711a92e506836864d"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1166, 49], "end": [1166, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4a404c398053954b8561b4f"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1165, 24], "end": [1165, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4734ff0a5c5f45fd484716e0"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1165, 10], "end": [1165, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e92b35e73d5eeb823529a0d4"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1218, 1], "end": [1228, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f3f1c073e7d85964aa8b908"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1165, 56], "end": [1165, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15144cd621521a487e83ca00"></a>
## left_type

`struct_field` · `sqlparser::ast::ddl::AlterOperator::left_type` · sqlparser 0.62.0

```rust
left_type: Option<ast::DataType>
```

Source: `src/ast/ddl.rs:1172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left operand type (`None` if no left operand)

<a id="op-e053dae0d7edbf1187355997"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterOperator::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:1170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator name (can be schema-qualified)

<a id="op-4e4a8e81b8758b762d221d99"></a>
## operation

`struct_field` · `sqlparser::ast::ddl::AlterOperator::operation` · sqlparser 0.62.0

```rust
operation: AlterOperatorOperation
```

Source: `src/ast/ddl.rs:1176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operation to perform

<a id="op-c0e5821dfac4b1fb9da502a2"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1165, 35], "end": [1165, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb944ab1eb33d407a1ed831d"></a>
## right_type

`struct_field` · `sqlparser::ast::ddl::AlterOperator::right_type` · sqlparser 0.62.0

```rust
right_type: ast::DataType
```

Source: `src/ast/ddl.rs:1174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right operand type

<a id="op-1a93957c8f2a1b05c7aa5dfd"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1166, 38], "end": [1166, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d5b2b93b29a34c606b75228"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 47], "end": [1167, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec767bd800acfaeacea4c697"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 40], "end": [1167, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
