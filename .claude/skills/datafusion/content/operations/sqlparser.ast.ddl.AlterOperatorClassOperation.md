# `sqlparser::ast::ddl::AlterOperatorClassOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterOperatorClassOperation.json).

<a id="op-6235f1dc7649a6f5015f97c6"></a>
## AlterOperatorClassOperation

`enum` · `sqlparser::ast::ddl::AlterOperatorClassOperation` · sqlparser 0.62.0

```rust
enum AlterOperatorClassOperation
```

Source: `src/ast/ddl.rs:5316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An [AlterOperatorClass](../operations/sqlparser.ast.ddl.AlterOperatorClass.md#op-27a96cde65c5d5c3b143e11a) operation

<a id="op-ea371d3edba43b8769813f2b"></a>
## OwnerTo

`variant` · `sqlparser::ast::ddl::AlterOperatorClassOperation::OwnerTo` · sqlparser 0.62.0

```rust
OwnerTo
```

Source: `src/ast/ddl.rs:5324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }`

<a id="op-979b82eb7e79e0e31ab2adc3"></a>
## RenameTo

`variant` · `sqlparser::ast::ddl::AlterOperatorClassOperation::RenameTo` · sqlparser 0.62.0

```rust
RenameTo
```

Source: `src/ast/ddl.rs:5319`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RENAME TO new_name`
Rename the operator class to a new name.

<a id="op-896800130a8e34e6fdbecdac"></a>
## SetSchema

`variant` · `sqlparser::ast::ddl::AlterOperatorClassOperation::SetSchema` · sqlparser 0.62.0

```rust
SetSchema
```

Source: `src/ast/ddl.rs:5327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET SCHEMA new_schema`
Set the schema for the operator class.

<a id="op-03f411262600c815877ca486"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterOperatorClassOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5313, 17], "end": [5313, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd78fffc19344db3dabab877"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterOperatorClassOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5313, 51], "end": [5313, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5da5d63a740709d674460bf"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5314, 49], "end": [5314, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-573541230c967bfa38d1cb3f"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterOperatorClassOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5313, 24], "end": [5313, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49c72efcd4a9196284988a13"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5313, 10], "end": [5313, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7b899841c4c3bc432b19792"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5340, 1], "end": [5354, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64fddaaaea26cfcff79e9162"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5313, 56], "end": [5313, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c74de4f4c78a1d6a0493c273"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterOperatorClassOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5313, 35], "end": [5313, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e1cf867c417a20eeb7181f5"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5314, 38], "end": [5314, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-333ba3fbb81423881c90553b"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5315, 40], "end": [5315, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73a42c84a8ad2772556e38ee"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorClassOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClassOperation", "path": "AlterOperatorClassOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5315, 47], "end": [5315, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
