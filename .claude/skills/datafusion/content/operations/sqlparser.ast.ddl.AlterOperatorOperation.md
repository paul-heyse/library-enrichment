# `sqlparser::ast::ddl::AlterOperatorOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterOperatorOperation.json).

<a id="op-e3fe3d453dd435cf3356c96c"></a>
## AlterOperatorOperation

`enum` · `sqlparser::ast::ddl::AlterOperatorOperation` · sqlparser 0.62.0

```rust
enum AlterOperatorOperation
```

Source: `src/ast/ddl.rs:1183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An [AlterOperator](../operations/sqlparser.ast.ddl.AlterOperator.md#op-a05db56051abf8c6692f0daf) operation

<a id="op-3645f39dab1ad6f620f1016f"></a>
## OwnerTo

`variant` · `sqlparser::ast::ddl::AlterOperatorOperation::OwnerTo` · sqlparser 0.62.0

```rust
OwnerTo
```

Source: `src/ast/ddl.rs:1185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }`

<a id="op-fc8af89444447d7176d95471"></a>
## Set

`variant` · `sqlparser::ast::ddl::AlterOperatorOperation::Set` · sqlparser 0.62.0

```rust
Set
```

Source: `src/ast/ddl.rs:1193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET ( options )`

<a id="op-ea823c519c55fa9bd4860c92"></a>
## SetSchema

`variant` · `sqlparser::ast::ddl::AlterOperatorOperation::SetSchema` · sqlparser 0.62.0

```rust
SetSchema
```

Source: `src/ast/ddl.rs:1188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET SCHEMA new_schema`
Set the operator's schema name.

<a id="op-dddae6533c6c876a0ab8222e"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterOperatorOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1180, 17], "end": [1180, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d55de6005a0be0bf8c0257d0"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterOperatorOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1180, 51], "end": [1180, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d658be51f9bfdfb2dbe85f9a"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 49], "end": [1181, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd1155a48791cc2fc5dd2c72"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterOperatorOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1180, 24], "end": [1180, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55b3c05f54266363137e67e5"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1230, 1], "end": [1247, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60ddd403d6d6497957d9f9c8"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1180, 10], "end": [1180, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-712bda74cf9eef1b70e316e3"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1180, 56], "end": [1180, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0611a3568cf53efed118fe99"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterOperatorOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1180, 35], "end": [1180, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ce9f10c32459f28d921dced"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 38], "end": [1181, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-146aa68ac8fe91b6bbe728cf"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1182, 47], "end": [1182, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b31fae7ec5e1484892639a5d"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorOperation", "path": "AlterOperatorOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1182, 40], "end": [1182, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
