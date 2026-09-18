# `sqlparser::ast::ddl::AlterFunctionOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterFunctionOperation.json).

<a id="op-f3c09dc34322b2316630da03"></a>
## AlterFunctionOperation

`enum` · `sqlparser::ast::ddl::AlterFunctionOperation` · sqlparser 0.62.0

```rust
enum AlterFunctionOperation
```

Source: `src/ast/ddl.rs:5407`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operation for `ALTER FUNCTION` / `ALTER AGGREGATE`.

<a id="op-8bbb195e7dda11888d822a0e"></a>
## Actions

`variant` · `sqlparser::ast::ddl::AlterFunctionOperation::Actions` · sqlparser 0.62.0

```rust
Actions
```

Source: `src/ast/ddl.rs:5428`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`action [ ... ] [ RESTRICT ]` (function only).

<a id="op-c4590b9c5158814bdbe551d2"></a>
## DependsOnExtension

`variant` · `sqlparser::ast::ddl::AlterFunctionOperation::DependsOnExtension` · sqlparser 0.62.0

```rust
DependsOnExtension
```

Source: `src/ast/ddl.rs:5421`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ NO ] DEPENDS ON EXTENSION extension_name`

<a id="op-13188a1484061a9c516bdbcc"></a>
## OwnerTo

`variant` · `sqlparser::ast::ddl::AlterFunctionOperation::OwnerTo` · sqlparser 0.62.0

```rust
OwnerTo
```

Source: `src/ast/ddl.rs:5414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }`

<a id="op-2d5eb84c1b1a0bf426242b3e"></a>
## RenameTo

`variant` · `sqlparser::ast::ddl::AlterFunctionOperation::RenameTo` · sqlparser 0.62.0

```rust
RenameTo
```

Source: `src/ast/ddl.rs:5409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RENAME TO new_name`

<a id="op-6fc405482bea646a10c64508"></a>
## SetSchema

`variant` · `sqlparser::ast::ddl::AlterFunctionOperation::SetSchema` · sqlparser 0.62.0

```rust
SetSchema
```

Source: `src/ast/ddl.rs:5416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET SCHEMA schema_name`

<a id="op-008bdd5c300927b1dd12c553"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterFunctionOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5404, 17], "end": [5404, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13a804ae2553c9f9d82f6092"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterFunctionOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5404, 51], "end": [5404, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84c60d4c01c6fb52b3fb5a24"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5405, 49], "end": [5405, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0f5631df3e80d679994bcb0"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterFunctionOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5404, 24], "end": [5404, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cade43bd9ec8bfbea7b447c"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5404, 10], "end": [5404, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e01860fd415021a2fd8f854a"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5503, 1], "end": [5529, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a4b9a22ec39d75abfb9a97c"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5404, 56], "end": [5404, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00f21995c4707e6c193cc340"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterFunctionOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5404, 35], "end": [5404, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc929f521cba8d1bf5e9cfd4"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5405, 38], "end": [5405, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24d4e133ab211a4e4083bd3f"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5406, 47], "end": [5406, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5406`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a808b77bf26bb2aa0007997d"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunctionOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionOperation", "path": "AlterFunctionOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5406, 40], "end": [5406, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5406`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
