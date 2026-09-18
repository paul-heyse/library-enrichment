# `sqlparser::ast::ddl::AlterPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterPolicy.json).

<a id="op-83e4dcd9729ee35e9f69e807"></a>
## AlterPolicy

`struct` · `sqlparser::ast::ddl::AlterPolicy` · sqlparser 0.62.0

```rust
struct AlterPolicy
```

Source: `src/ast/ddl.rs:5733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ALTER POLICY statement.

```sql
ALTER POLICY <NAME> ON <TABLE NAME> [<OPERATION>]
```
(Postgresql-specific)

<a id="op-411cb71e832a223f12b01f6b"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterPolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5730, 17], "end": [5730, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2987d06316a1fcbe5bf8fa2"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterPolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterPolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5730, 51], "end": [5730, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac7137c456123606c10d959c"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterPolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5731, 49], "end": [5731, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5731`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6d6bae706c5368487b9e32e"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterPolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5730, 24], "end": [5730, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6ccd0a31b7dcfb1168de4fb"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5743, 1], "end": [5753, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bec6f56d78ea5755fd8a36c6"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5730, 10], "end": [5730, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-955683f4ae0fa5312a271c92"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterPolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5730, 56], "end": [5730, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f93b48c891ff646c2a15e296"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterPolicy::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:5735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Policy name to alter.

<a id="op-29ee002183fd06b1985b98ad"></a>
## operation

`struct_field` · `sqlparser::ast::ddl::AlterPolicy::operation` · sqlparser 0.62.0

```rust
operation: AlterPolicyOperation
```

Source: `src/ast/ddl.rs:5740`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional operation specific to the policy alteration.

<a id="op-383503f73ed34a322a53342a"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterPolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterPolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5730, 35], "end": [5730, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-461edad9d9bb1a55479f0def"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterPolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5731, 38], "end": [5731, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5731`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbc7a49c5b5acc401e42080b"></a>
## table_name

`struct_field` · `sqlparser::ast::ddl::AlterPolicy::table_name` · sqlparser 0.62.0

```rust
table_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:5738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target table name the policy is defined on.

<a id="op-0b105976e2cf5bf0d4e5062e"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5732, 47], "end": [5732, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbb42c6fff18d9195aa1062d"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5732, 40], "end": [5732, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
