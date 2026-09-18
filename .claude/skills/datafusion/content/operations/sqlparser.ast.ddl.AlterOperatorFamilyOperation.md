# `sqlparser::ast::ddl::AlterOperatorFamilyOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterOperatorFamilyOperation.json).

<a id="op-381b478f732f1982a1cede46"></a>
## AlterOperatorFamilyOperation

`enum` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation` · sqlparser 0.62.0

```rust
enum AlterOperatorFamilyOperation
```

Source: `src/ast/ddl.rs:5234`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An [AlterOperatorFamily](../operations/sqlparser.ast.ddl.AlterOperatorFamily.md#op-2199620fb97e7c93e0221bb9) operation

<a id="op-2902fdc71ff7e79575923a7b"></a>
## Add

`variant` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::Add` · sqlparser 0.62.0

```rust
Add
```

Source: `src/ast/ddl.rs:5236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ADD { OPERATOR ... | FUNCTION ... } [, ...]`

<a id="op-125fe0cfc44cf0482492cf47"></a>
## Drop

`variant` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::Drop` · sqlparser 0.62.0

```rust
Drop
```

Source: `src/ast/ddl.rs:5241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP { OPERATOR ... | FUNCTION ... } [, ...]`

<a id="op-71b05675671538e6da04230b"></a>
## OwnerTo

`variant` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::OwnerTo` · sqlparser 0.62.0

```rust
OwnerTo
```

Source: `src/ast/ddl.rs:5251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }`

<a id="op-dae53d21878400c5e9a2c1e2"></a>
## RenameTo

`variant` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::RenameTo` · sqlparser 0.62.0

```rust
RenameTo
```

Source: `src/ast/ddl.rs:5246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RENAME TO new_name`

<a id="op-95da11a0b4d0738f84c725b3"></a>
## SetSchema

`variant` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::SetSchema` · sqlparser 0.62.0

```rust
SetSchema
```

Source: `src/ast/ddl.rs:5253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET SCHEMA new_schema`

<a id="op-908f550f8916bc885d1089e3"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterOperatorFamilyOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5231, 17], "end": [5231, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d9a9aa600168af0f8e4b330"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterOperatorFamilyOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5231, 51], "end": [5231, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-512533ec8c85906b8252ebc3"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5232, 49], "end": [5232, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-548a7620324712615208e760"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterOperatorFamilyOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5231, 24], "end": [5231, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89f0dc49dacbbed41e3e546f"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5270, 1], "end": [5290, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b11a73485cda39d3285aa7ee"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5231, 10], "end": [5231, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8017183c62db2b4a767af66"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5231, 56], "end": [5231, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a284d4c210077ece34f2b47"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterOperatorFamilyOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5231, 35], "end": [5231, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e37f3ed7e98d070b1a665f3"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5232, 38], "end": [5232, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d620c71ede0116b8dae496b"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5233, 40], "end": [5233, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4f2a89695ff32adde308fca"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorFamilyOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamilyOperation", "path": "AlterOperatorFamilyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5233, 47], "end": [5233, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
