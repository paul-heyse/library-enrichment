# `sqlparser::ast::ddl::AlterPolicyOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterPolicyOperation.json).

<a id="op-8bd52b045981f53fe992a57d"></a>
## AlterPolicyOperation

`enum` · `sqlparser::ast::ddl::AlterPolicyOperation` · sqlparser 0.62.0

```rust
enum AlterPolicyOperation
```

Source: `src/ast/ddl.rs:545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ALTER Policy` (`Statement::AlterPolicy`) operation

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-9c5887d6986bd577ec59c699"></a>
## Apply

`variant` · `sqlparser::ast::ddl::AlterPolicyOperation::Apply` · sqlparser 0.62.0

```rust
Apply
```

Source: `src/ast/ddl.rs:552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply/modify policy properties.

<a id="op-b535b69abc87d12598705bf4"></a>
## Rename

`variant` · `sqlparser::ast::ddl::AlterPolicyOperation::Rename` · sqlparser 0.62.0

```rust
Rename
```

Source: `src/ast/ddl.rs:547`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Rename the policy to `new_name`.

<a id="op-d352e5bce20768f2b973a04c"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterPolicyOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 17], "end": [542, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25a7311f674f4b2626bfe1aa"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterPolicyOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 51], "end": [542, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0692ed1578b6184327d965d7"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 49], "end": [543, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:543`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1a8aeadc2a509438fe7b8d1"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterPolicyOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 24], "end": [542, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cddac0673c303898fc9b6f0"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [586, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f64fe325217366e17bfdcb2"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 10], "end": [542, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95b34449ee22b0ee27d7f786"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 56], "end": [542, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19e28d66187026db35b84dd"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterPolicyOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [542, 35], "end": [542, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cfdd420e87e9360cc3438e9"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 38], "end": [543, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:543`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fe527d41b37ec6e2f64d8df"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 47], "end": [544, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8d9f5bdcd306c7c1b4b13a6"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterPolicyOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicyOperation", "path": "AlterPolicyOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 40], "end": [544, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
