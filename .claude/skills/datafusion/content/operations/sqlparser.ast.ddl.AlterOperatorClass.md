# `sqlparser::ast::ddl::AlterOperatorClass`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterOperatorClass.json).

<a id="op-27a96cde65c5d5c3b143e11a"></a>
## AlterOperatorClass

`struct` · `sqlparser::ast::ddl::AlterOperatorClass` · sqlparser 0.62.0

```rust
struct AlterOperatorClass
```

Source: `src/ast/ddl.rs:5303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALTER OPERATOR CLASS` statement
See <https://www.postgresql.org/docs/current/sql-alteropclass.html>

<a id="op-d7d5d579c8f90220d6ce7f3d"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterOperatorClass::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterOperatorClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5300, 17], "end": [5300, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f26c25da93809e27839faae"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterOperatorClass::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterOperatorClass) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5300, 51], "end": [5300, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb81498bae048bc90774ac47"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterOperatorClass::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5301, 49], "end": [5301, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bf550237e0f1135ab214fa0"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterOperatorClass::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterOperatorClass) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5300, 24], "end": [5300, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-293311a49e890f381e66813c"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorClass::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5300, 10], "end": [5300, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30aeec7b0754dd85228df3bd"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterOperatorClass::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5333, 1], "end": [5338, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcef8ed92e3517b9d2c529db"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterOperatorClass::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5300, 56], "end": [5300, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86bef496f6d0f2ac3d1c5d01"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterOperatorClass::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:5305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator class name (can be schema-qualified)

<a id="op-e9ebcaaf73716cb6d58cf719"></a>
## operation

`struct_field` · `sqlparser::ast::ddl::AlterOperatorClass::operation` · sqlparser 0.62.0

```rust
operation: AlterOperatorClassOperation
```

Source: `src/ast/ddl.rs:5309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operation to perform

<a id="op-c0c6031f43fc677ccde307c0"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterOperatorClass::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterOperatorClass) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5300, 35], "end": [5300, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-279043b4087b5f3ce9eef4ea"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterOperatorClass::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5301, 38], "end": [5301, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3255719c08dbd63453235c1"></a>
## span

`function` · `sqlparser::ast::ddl::AlterOperatorClass::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5356, 1], "end": [5360, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:5357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7848458d4e8ed1ad09bc7a43"></a>
## using

`struct_field` · `sqlparser::ast::ddl::AlterOperatorClass::using` · sqlparser 0.62.0

```rust
using: ast::Ident
```

Source: `src/ast/ddl.rs:5307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index method (btree, hash, gist, gin, etc.)

<a id="op-39529ee458d3ae4e4c5da85d"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorClass::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5302, 47], "end": [5302, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f803c47ad97a6a7d7b1eb21"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterOperatorClass::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5302, 40], "end": [5302, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
