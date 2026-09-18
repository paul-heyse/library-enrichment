# `sqlparser::ast::dcl::Revoke`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.Revoke.json).

<a id="op-d9073bf0ca402c7fddf50b46"></a>
## Revoke

`struct` · `sqlparser::ast::dcl::Revoke` · sqlparser 0.62.0

```rust
struct Revoke
```

Source: `src/ast/dcl.rs:492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

REVOKE privileges ON objects FROM grantees

<a id="op-017d0b467fd7dc82269f4fdb"></a>
## cascade

`struct_field` · `sqlparser::ast::dcl::Revoke::cascade` · sqlparser 0.62.0

```rust
cascade: Option<ast::CascadeOption>
```

Source: `src/ast/dcl.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `CASCADE`/`RESTRICT` behavior.

<a id="op-25a1f8919351f901d51f6438"></a>
## clone

`function` · `sqlparser::ast::dcl::Revoke::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Revoke
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 17], "end": [489, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dc3b290edbf50805cd9c86a"></a>
## cmp

`function` · `sqlparser::ast::dcl::Revoke::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Revoke) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 51], "end": [489, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2bae7731527a8d52be0b9a4"></a>
## deserialize

`function` · `sqlparser::ast::dcl::Revoke::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 49], "end": [490, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dae875d5ff7761158e3f14a"></a>
## eq

`function` · `sqlparser::ast::dcl::Revoke::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Revoke) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 24], "end": [489, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af52cb27631a06df0c1a9bd8"></a>
## fmt

`function` · `sqlparser::ast::dcl::Revoke::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 1], "end": [522, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dcl.rs:508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee679876e22ecf710c9bfdb5"></a>
## fmt

`function` · `sqlparser::ast::dcl::Revoke::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 10], "end": [489, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de9d7f45580b06eedf54a20f"></a>
## granted_by

`struct_field` · `sqlparser::ast::dcl::Revoke::granted_by` · sqlparser 0.62.0

```rust
granted_by: Option<super::Ident>
```

Source: `src/ast/dcl.rs:502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `GRANTED BY` identifier.

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dcl-statements)

<a id="op-9f058461ab39d0e3dac08f09"></a>
## grantees

`struct_field` · `sqlparser::ast::dcl::Revoke::grantees` · sqlparser 0.62.0

```rust
grantees: Vec<ast::Grantee>
```

Source: `src/ast/dcl.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grantees affected by the revoke.

<a id="op-65af71de3929cad40ff3fc48"></a>
## hash

`function` · `sqlparser::ast::dcl::Revoke::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 56], "end": [489, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff183ea8021216af532728b9"></a>
## objects

`struct_field` · `sqlparser::ast::dcl::Revoke::objects` · sqlparser 0.62.0

```rust
objects: Option<ast::GrantObjects>
```

Source: `src/ast/dcl.rs:496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional objects from which to revoke.

<a id="op-e13d98d005cf25d5ecb2fe49"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::Revoke::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Revoke) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 35], "end": [489, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a4a52ce1e2d5ef73a9ac596"></a>
## privileges

`struct_field` · `sqlparser::ast::dcl::Revoke::privileges` · sqlparser 0.62.0

```rust
privileges: ast::Privileges
```

Source: `src/ast/dcl.rs:494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Privileges to revoke.

<a id="op-0b5a600d173becaaf89ec2d5"></a>
## serialize

`function` · `sqlparser::ast::dcl::Revoke::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 38], "end": [490, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70fc46c8dffd1952e91222ff"></a>
## visit

`function` · `sqlparser::ast::dcl::Revoke::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 47], "end": [491, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a85a36a2e76cafbb41a03a70"></a>
## visit

`function` · `sqlparser::ast::dcl::Revoke::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 40], "end": [491, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
